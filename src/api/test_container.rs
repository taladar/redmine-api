//! Automatically provides a local Redmine container to run the tests against
//!
//! This is enabled by setting `REDMINE_TEST_CONTAINER_RUNTIME` to `podman` or
//! `docker` (in the environment or the .env file). The container uses the
//! official `docker.io/library/redmine` image with the tag from
//! `REDMINE_VERSION`, publishes the Redmine port on the loopback port from
//! `REDMINE_URL` and uses the key from `REDMINE_API_KEY` as the API key of the
//! admin user.
//!
//! Redmine uses a PostgreSQL database running in a second container since
//! the SQLite fallback of the image fails with "database is locked" errors
//! when the tests run in parallel. Both containers are attached to a
//! dedicated network.
//!
//! The containers are named `redmine-api-test-<version>` and
//! `redmine-api-test-<version>-db` and are left running after the tests so
//! later test runs can reuse them. If they do not exist they are created
//! (pulling the images if necessary), if they are stopped they are started
//! (stopping other Redmine containers created by this module that use the
//! same port first). Once Redmine answers it is seeded with the default data,
//! an enabled REST API and the configured API key unless the API key already
//! works.
//!
//! To start over with an empty database remove both containers, e.g.
//! `podman rm -f redmine-api-test-7.0.3 redmine-api-test-7.0.3-db`.

use std::error::Error;
use std::io::{Read as _, Write as _};
use std::process::Command;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tracing::{debug, info};

/// label set on all containers created by this module and used as prefix for
/// the names of the containers and the network
const CONTAINER_LABEL: &str = "redmine-api-test";

/// the PostgreSQL image used for the database container
const POSTGRES_IMAGE: &str = "docker.io/library/postgres:17";

/// user name, password and database name of the database, the database
/// container is only reachable from the test network and holds only test data
const POSTGRES_CREDENTIAL: &str = "redmine";

/// the `secret_key_base` used by the Redmine container, the container is
/// only reachable on the loopback interface and holds only test data
const SECRET_KEY_BASE: &str = "redmine-api-test-container-secret-key-base-not-secret";

/// how long to wait for Redmine or the database to become ready after
/// creating or starting the containers (the first start of Redmine runs all
/// the database migrations)
const STARTUP_TIMEOUT: Duration = Duration::from_secs(600);

/// Ruby code run with `rails runner` to seed a fresh container, the API key
/// is passed in the `REDMINE_TEST_API_KEY` environment variable
const SEED_SCRIPT: &str = r"
if Redmine::DefaultData::Loader.no_data?
  Redmine::DefaultData::Loader.load('en')
end
Setting.rest_api_enabled = '1'
admin = User.find_by_login('admin')
admin.must_change_passwd = false
admin.save!
# creates the token with a random value if it does not exist yet
admin.api_key
admin.api_token.update_column(:value, ENV.fetch('REDMINE_TEST_API_KEY'))
";

/// the result of the one time container setup in this test process
static CONTAINER_SETUP: OnceLock<Result<(), String>> = OnceLock::new();

/// Makes sure the test containers are running if
/// `REDMINE_TEST_CONTAINER_RUNTIME` is set, does nothing otherwise
///
/// The setup only runs once per test process, all other callers wait for it
/// and get the same result.
///
/// # Errors
///
/// This returns an error if the configuration is invalid or the containers
/// could not be set up
pub fn ensure_container() -> Result<(), Box<dyn Error>> {
    CONTAINER_SETUP
        .get_or_init(|| setup().map_err(|e| format!("Setting up the Redmine test container: {e}")))
        .clone()
        .map_err(Into::into)
}

/// the settings for the test containers
#[derive(Debug)]
struct ContainerConfig {
    /// the container runtime command (podman or docker)
    runtime: String,
    /// the Redmine version, used as the image tag
    version: String,
    /// the loopback port the Redmine port is published on
    port: u16,
    /// the API key of the admin user
    api_key: String,
}

impl ContainerConfig {
    /// reads the configuration from the environment, returns None if the
    /// container is not enabled
    fn from_env() -> Result<Option<Self>, Box<dyn Error>> {
        let Ok(runtime) = std::env::var("REDMINE_TEST_CONTAINER_RUNTIME") else {
            return Ok(None);
        };
        let runtime = runtime.trim().to_owned();
        if runtime.is_empty() {
            return Ok(None);
        }
        if runtime != "podman" && runtime != "docker" {
            return Err(format!(
                "REDMINE_TEST_CONTAINER_RUNTIME must be podman or docker, not {runtime:?}"
            )
            .into());
        }
        let version = std::env::var("REDMINE_VERSION")
            .map_err(|e| format!("REDMINE_VERSION is required for the test container: {e}"))?
            .trim()
            .to_owned();
        // validate the version format, it is used as image tag and in the name
        version.parse::<crate::api::test_helpers::RedmineVersion>()?;
        let url: url::Url = std::env::var("REDMINE_URL")
            .map_err(|e| format!("REDMINE_URL is required for the test container: {e}"))?
            .parse()?;
        if url.scheme() != "http"
            || !matches!(url.host_str(), Some("127.0.0.1" | "localhost"))
            || url.path() != "/"
        {
            return Err(format!(
                "REDMINE_URL must look like http://127.0.0.1:<port> for the test container, not {url}"
            )
            .into());
        }
        let port = url
            .port()
            .ok_or("REDMINE_URL must contain an explicit port for the test container")?;
        let api_key = std::env::var("REDMINE_API_KEY")
            .map_err(|e| format!("REDMINE_API_KEY is required for the test container: {e}"))?;
        // Redmine only accepts alphanumeric API keys
        if api_key.is_empty() || !api_key.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(
                "REDMINE_API_KEY must only contain letters and digits (e.g. 40 hex digits) \
                 for the test container"
                    .into(),
            );
        }
        Ok(Some(Self {
            runtime,
            version,
            port,
            api_key,
        }))
    }

    /// the name of the Redmine container
    fn name(&self) -> String {
        format!("{CONTAINER_LABEL}-{}", self.version)
    }

    /// the name of the database container belonging to the Redmine container
    /// with the given name
    fn db_name_for(name: &str) -> String {
        format!("{name}-db")
    }

    /// the name of the database container
    fn db_name(&self) -> String {
        Self::db_name_for(&self.name())
    }

    /// the Redmine image
    fn image(&self) -> String {
        format!("docker.io/library/redmine:{}", self.version)
    }

    /// runs the container runtime with the given arguments and returns stdout
    fn run(&self, args: &[&str]) -> Result<String, Box<dyn Error>> {
        debug!(runtime = %self.runtime, ?args, "Running container runtime");
        let output = Command::new(&self.runtime)
            .args(args)
            .output()
            .map_err(|e| format!("Running {} failed: {e}", self.runtime))?;
        if !output.status.success() {
            return Err(format!(
                "{} {} failed with {}: {}",
                self.runtime,
                args.join(" "),
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            )
            .into());
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }

    /// returns whether the container with the given name is running or None
    /// if it does not exist
    fn state(&self, name: &str) -> Result<Option<bool>, Box<dyn Error>> {
        // unlike inspect ps does not fail for missing containers
        let names = self.run(&[
            "ps",
            "-a",
            "--filter",
            &format!("label={CONTAINER_LABEL}"),
            "--format",
            "{{.Names}}",
        ])?;
        if !names.lines().any(|n| n.trim() == name) {
            return Ok(None);
        }
        let running = self.run(&["inspect", "--format", "{{.State.Running}}", name])?;
        Ok(Some(running == "true"))
    }

    /// creates the network for the containers if it does not exist yet
    fn ensure_network(&self) -> Result<(), Box<dyn Error>> {
        let networks = self.run(&["network", "ls", "--format", "{{.Name}}"])?;
        if networks.lines().any(|n| n.trim() == CONTAINER_LABEL) {
            return Ok(());
        }
        info!(
            network = CONTAINER_LABEL,
            "Creating network for Redmine test containers"
        );
        if let Err(e) = self.run(&["network", "create", CONTAINER_LABEL]) {
            // another test process might have created it at the same time
            let networks = self.run(&["network", "ls", "--format", "{{.Name}}"])?;
            if !networks.lines().any(|n| n.trim() == CONTAINER_LABEL) {
                return Err(e);
            }
        }
        Ok(())
    }

    /// stops other Redmine containers created by this module that publish our
    /// port (and their database containers)
    fn stop_conflicting_containers(&self) -> Result<(), Box<dyn Error>> {
        let name = self.name();
        let others = self.run(&[
            "ps",
            "--filter",
            &format!("label={CONTAINER_LABEL}.port={}", self.port),
            "--format",
            "{{.Names}}",
        ])?;
        for other in others
            .lines()
            .map(str::trim)
            .filter(|n| !n.is_empty() && *n != name)
        {
            info!(container = %other, port = self.port, "Stopping test container using the same port");
            self.run(&["stop", other])?;
            let other_db = Self::db_name_for(other);
            if self.state(&other_db)? == Some(true) {
                self.run(&["stop", &other_db])?;
            }
        }
        Ok(())
    }

    /// creates and starts a container with the given name and arguments
    /// (everything after `run --detach --name <name>`)
    fn create(&self, name: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
        info!(container = %name, "Creating test container");
        let mut run_args = vec!["run", "--detach", "--name", name];
        run_args.extend_from_slice(args);
        if let Err(e) = self.run(&run_args) {
            // another test process might have created it at the same time
            if self.state(name)?.is_some() {
                debug!("Container was created concurrently: {e}");
            } else {
                return Err(e);
            }
        }
        Ok(())
    }

    /// makes sure the database container is running and accepts connections
    fn ensure_database(&self) -> Result<(), Box<dyn Error>> {
        let db_name = self.db_name();
        match self.state(&db_name)? {
            Some(true) => {}
            Some(false) => {
                info!(container = %db_name, "Starting database test container");
                self.run(&["start", &db_name])?;
            }
            None => {
                let password = format!("POSTGRES_PASSWORD={POSTGRES_CREDENTIAL}");
                let user = format!("POSTGRES_USER={POSTGRES_CREDENTIAL}");
                let database = format!("POSTGRES_DB={POSTGRES_CREDENTIAL}");
                self.create(
                    &db_name,
                    &[
                        "--label",
                        CONTAINER_LABEL,
                        "--network",
                        CONTAINER_LABEL,
                        "--env",
                        &password,
                        "--env",
                        &user,
                        "--env",
                        &database,
                        POSTGRES_IMAGE,
                    ],
                )?;
            }
        }
        let start = Instant::now();
        loop {
            // the official image restarts the server once after the initial
            // setup so we check the actual database we need via TCP
            match self.run(&[
                "exec",
                &db_name,
                "pg_isready",
                "--host",
                "127.0.0.1",
                "--username",
                POSTGRES_CREDENTIAL,
                "--dbname",
                POSTGRES_CREDENTIAL,
            ]) {
                Ok(_) => return Ok(()),
                Err(e) => debug!("Database not ready yet: {e}"),
            }
            if start.elapsed() > STARTUP_TIMEOUT {
                return Err(format!(
                    "Database in container {db_name} did not become ready within {STARTUP_TIMEOUT:?}"
                )
                .into());
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    /// makes sure the Redmine container is running (but not necessarily
    /// ready yet)
    fn ensure_redmine(&self) -> Result<(), Box<dyn Error>> {
        let name = self.name();
        match self.state(&name)? {
            Some(true) => {}
            Some(false) => {
                info!(container = %name, "Starting Redmine test container");
                self.run(&["start", &name])?;
            }
            None => {
                let port_label = format!("{CONTAINER_LABEL}.port={}", self.port);
                let publish = format!("127.0.0.1:{}:3000", self.port);
                let db_host = format!("REDMINE_DB_POSTGRES={}", self.db_name());
                let db_user = format!("REDMINE_DB_USERNAME={POSTGRES_CREDENTIAL}");
                let db_password = format!("REDMINE_DB_PASSWORD={POSTGRES_CREDENTIAL}");
                let db_database = format!("REDMINE_DB_DATABASE={POSTGRES_CREDENTIAL}");
                let secret = format!("SECRET_KEY_BASE={SECRET_KEY_BASE}");
                let image = self.image();
                self.create(
                    &name,
                    &[
                        "--label",
                        CONTAINER_LABEL,
                        "--label",
                        &port_label,
                        "--network",
                        CONTAINER_LABEL,
                        "--publish",
                        &publish,
                        "--env",
                        &db_host,
                        "--env",
                        &db_user,
                        "--env",
                        &db_password,
                        "--env",
                        &db_database,
                        "--env",
                        &secret,
                        &image,
                    ],
                )?;
            }
        }
        Ok(())
    }

    /// sends a GET request to the given path and returns the status code
    ///
    /// this uses a plain TCP connection instead of reqwest since the blocking
    /// reqwest client can not be used inside the async tests
    fn http_status(&self, path: &str, api_key: Option<&str>) -> Result<u16, Box<dyn Error>> {
        let mut stream = std::net::TcpStream::connect(("127.0.0.1", self.port))?;
        stream.set_read_timeout(Some(Duration::from_secs(60)))?;
        let key_header = api_key
            .map(|k| format!("X-Redmine-API-Key: {k}\r\n"))
            .unwrap_or_default();
        write!(
            stream,
            "GET {path} HTTP/1.0\r\nHost: 127.0.0.1:{}\r\n{key_header}\r\n",
            self.port
        )?;
        let mut response = Vec::new();
        stream.read_to_end(&mut response)?;
        let response = String::from_utf8_lossy(&response);
        let status_line = response.lines().next().unwrap_or_default();
        status_line
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| format!("Invalid HTTP response from Redmine: {status_line:?}").into())
    }

    /// waits until Redmine answers HTTP requests
    fn wait_until_ready(&self) -> Result<(), Box<dyn Error>> {
        let start = Instant::now();
        loop {
            match self.http_status("/", None) {
                Ok(status) if status < 500 => return Ok(()),
                Ok(status) => debug!(status, "Redmine not ready yet"),
                Err(e) => debug!("Redmine not ready yet: {e}"),
            }
            if self.state(&self.name())? == Some(false) {
                return Err(format!(
                    "Redmine container {} stopped while waiting for it to become ready, \
                     check its logs with {} logs {}",
                    self.name(),
                    self.runtime,
                    self.name()
                )
                .into());
            }
            if start.elapsed() > STARTUP_TIMEOUT {
                return Err(format!(
                    "Redmine in container {} did not become ready within {STARTUP_TIMEOUT:?}",
                    self.name()
                )
                .into());
            }
            std::thread::sleep(Duration::from_secs(2));
        }
    }

    /// returns whether the configured API key works
    fn api_key_works(&self) -> Result<bool, Box<dyn Error>> {
        Ok(self.http_status("/users/current.json", Some(&self.api_key))? == 200)
    }

    /// seeds Redmine with default data, enables the REST API and sets the API
    /// key of the admin user
    fn seed(&self) -> Result<(), Box<dyn Error>> {
        info!(container = %self.name(), "Seeding Redmine test container");
        self.run(&[
            "exec",
            "--env",
            &format!("SECRET_KEY_BASE={SECRET_KEY_BASE}"),
            "--env",
            &format!("REDMINE_TEST_API_KEY={}", self.api_key),
            &self.name(),
            "bundle",
            "exec",
            "rails",
            "runner",
            SEED_SCRIPT,
        ])?;
        Ok(())
    }
}

/// does the actual container setup
fn setup() -> Result<(), Box<dyn Error>> {
    let Some(config) = ContainerConfig::from_env()? else {
        return Ok(());
    };
    if config.state(&config.name())? != Some(true) {
        config.stop_conflicting_containers()?;
    }
    config.ensure_network()?;
    config.ensure_database()?;
    config.ensure_redmine()?;
    config.wait_until_ready()?;
    if !config.api_key_works()? {
        config.seed()?;
        if !config.api_key_works()? {
            return Err(format!(
                "API key still does not work after seeding container {}",
                config.name()
            )
            .into());
        }
    }
    Ok(())
}
