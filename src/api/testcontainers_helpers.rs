//! Helpers for testing with testcontainers
use std::error::Error;
use std::sync::OnceLock;
use std::time::Duration;

use rand::distributions::{Alphanumeric, DistString};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use testcontainers::Image;
use testcontainers::ImageExt;
use testcontainers::TestcontainersError;
use testcontainers::core::{Container, ContainerPort, ExecCommand, Mount, WaitFor};
use tokio::sync::Semaphore;
use tracing::trace;

use crate::api::{Redmine, RedmineAsync};

static CONTAINER_SEMAPHORE: OnceLock<Semaphore> = OnceLock::new();

fn get_container_semaphore() -> &'static Semaphore {
    CONTAINER_SEMAPHORE.get_or_init(|| Semaphore::new(10))
}

const REDMINE_CONTAINER_READY_MESSAGE: &str = "* Listening on http://0.0.0.0:3000";
const REDMINE_STARTUP_TIMEOUT_SECONDS: u64 = 120;

/// Represents a Redmine Docker image with its name and tag.
#[derive(Debug, Clone)]
pub struct RedmineImage {
    name: String,
    tag: String,
}

impl RedmineImage {
    /// Creates a new `RedmineImage` with the given Redmine version.
    ///
    /// # Arguments
    ///
    /// * `version` - The version of the Redmine image.
    #[must_use]
    pub fn new(version: String) -> Self {
        Self {
            name: "docker.io/redmine".to_string(),
            tag: format!("{}", version),
        }
    }
}

impl Image for RedmineImage {
    fn name(&self) -> &str {
        &self.name
    }

    fn tag(&self) -> &str {
        &self.tag
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout(REDMINE_CONTAINER_READY_MESSAGE)]
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        const PORTS: [ContainerPort; 1] = [ContainerPort::Tcp(3000)];
        &PORTS
    }

    fn mounts(&self) -> impl IntoIterator<Item = &Mount> {
        &[] // empty slice
    }

    fn entrypoint(&self) -> Option<&str> {
        None
    }

    fn exec_after_start(
        &self,
        _cs: testcontainers::core::ContainerState,
    ) -> Result<Vec<ExecCommand>, TestcontainersError> {
        // Changed return type to Box<dyn Error>
        Ok(vec![])
    }

    fn env_vars(
        &self,
    ) -> impl std::iter::IntoIterator<
        Item = (
            impl std::convert::Into<std::borrow::Cow<'_, str>>,
            impl std::convert::Into<std::borrow::Cow<'_, str>>,
        ),
    > {
        vec![("REDMINE_DB_SQLITE", "redmine.db")]
    }
}

/// The main function to create a Redmine container and run a test closure
///
/// # Panics
///
/// Panics if the Redmine container fails to provide an API key, indicating a critical setup failure.
///
/// # Errors
///
/// Returns an error if the Redmine container cannot be started, ports cannot be mapped,
/// if the API key cannot be retrieved, or if the provided closure `f` returns an error.
pub fn with_redmine_container<F>(version: &str, f: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&Redmine, &Container<RedmineImage>) -> Result<(), Box<dyn Error>>,
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let _permit = runtime.block_on(async {
        get_container_semaphore()
            .acquire()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    })?;
    trace!("Create Redmine {} container", version);
    let image = RedmineImage::new(version.to_string());
    let container =
        <testcontainers::ContainerRequest<RedmineImage> as testcontainers::runners::SyncRunner<
            _,
        >>::start(
            image.with_startup_timeout(Duration::from_secs(REDMINE_STARTUP_TIMEOUT_SECONDS)),
        )?;
    let port = container.get_host_port_ipv4(3000)?;
    let url = format!("http://127.0.0.1:{port}");
    trace!("Redmine {version} container running at {url}");
    let client = Client::builder()
        .cookie_store(true)
        .use_rustls_tls()
        .build()?;
    let login_url = format!("{}/login", url);
    let login_page = client.get(&login_url).send()?.text()?;
    let selector = Selector::parse("input[name=\"authenticity_token\"]").unwrap();
    let doc = Html::parse_document(&login_page);
    let authenticity_token = doc
        .select(&selector)
        .next()
        .and_then(|e| e.value().attr("value"))
        .unwrap();
    client
        .post(&login_url)
        .form(&[
            ("authenticity_token", authenticity_token),
            ("username", "admin"),
            ("password", "admin"),
        ])
        .send()?;
    trace!("Redmine {version} logged in as admin");
    let password_change_url = format!("{}/my/password", url);
    let password_change_page = client.get(&password_change_url).send()?.text()?;
    let doc = Html::parse_document(&password_change_page);
    let authenticity_token = doc
        .select(&selector)
        .next()
        .and_then(|e| e.value().attr("value"))
        .unwrap();
    let new_password = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    client
        .post(&password_change_url)
        .form(&[
            ("authenticity_token", authenticity_token),
            ("password", "admin"),
            ("new_password", &new_password),
            ("new_password_confirmation", &new_password),
        ])
        .send()?;
    trace!("Redmine {version} password changed");
    let settings_url = format!("{}/settings?tab=api", url);
    let settings_page = client.get(&settings_url).send()?.text()?;
    let doc = Html::parse_document(&settings_page);
    let authenticity_token = doc
        .select(&selector)
        .next()
        .and_then(|e| e.value().attr("value"))
        .unwrap();
    client
        .post(&settings_url)
        .form(&[
            ("authenticity_token", authenticity_token),
            ("settings[api_enabled]", "1"),
            ("settings[jsonp_enabled]", "0"),
            ("commit", "Save"),
        ])
        .send()?;
    trace!("Redmine {version} REST API enabled");
    let admin_url = format!("{}/admin", url);
    let admin_page = client.get(&admin_url).send()?.text()?;
    let doc = Html::parse_document(&admin_page);
    let authenticity_token = doc
        .select(&selector)
        .next()
        .and_then(|e| e.value().attr("value"))
        .unwrap();
    let default_config_url = format!("{}/admin/default_configuration", url);
    client
        .post(&default_config_url)
        .form(&[
            ("authenticity_token", authenticity_token),
            ("lang", "en"),
            ("commit", "Load the default configuration"),
        ])
        .send()?;
    trace!("Redmine {version} default data loaded");
    let api_key_url = format!("{}/my/api_key", url);
    let api_key_page = client.get(&api_key_url).send()?.text()?;
    let doc = Html::parse_document(&api_key_page);
    let selector = Selector::parse("#content .box pre").unwrap();
    let api_key = doc.select(&selector).next().unwrap().inner_html();
    trace!("Redmine {version} container API key is {api_key}");
    let client = reqwest::blocking::Client::builder()
        .use_rustls_tls()
        .build()?;
    let redmine = Redmine::new(client, url.parse()?, &api_key)?;
    trace!("Redmine {version} client created");
    f(&redmine, &container)?;
    trace!("Redmine {version} test closure finished");
    Ok(())
}
/// The main function to create a Redmine container and run an async test closure
///
/// # Panics
///
/// Panics if the Redmine container fails to provide an API key, indicating a critical setup failure.
///
/// # Errors
///
/// Returns an error if the Redmine container cannot be started, ports cannot be mapped,
/// if the API key cannot be retrieved, or if the provided asynchronous closure `f` returns an error.
pub async fn with_redmine_container_async<F, Fut>(version: &str, f: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&std::sync::Arc<RedmineAsync>, &testcontainers::ContainerAsync<RedmineImage>) -> Fut
        + Send
        + 'static,
    Fut: std::future::Future<Output = Result<(), Box<dyn Error>>> + Send + 'static,
{
    let _permit = get_container_semaphore().acquire().await?;
    trace!("Create Redmine {} container", version);
    let image = RedmineImage::new(version.to_string());
    let container =
        <testcontainers::ContainerRequest<RedmineImage> as testcontainers::runners::AsyncRunner<
            _,
        >>::start(
            image.with_startup_timeout(Duration::from_secs(REDMINE_STARTUP_TIMEOUT_SECONDS))
        )
        .await?;
    let port = container.get_host_port_ipv4(3000).await?;
    let url = format!("http://127.0.0.1:{port}");
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .use_rustls_tls()
        .build()?;
    let login_url = format!("{}/login", url);
    let login_page = client.get(&login_url).send().await?.text().await?;
    let selector = Selector::parse("input[name=\"authenticity_token\"]").unwrap();
    let doc = Html::parse_document(&login_page);
    let authenticity_token = doc
        .select(&selector)
        .next()
        .and_then(|e| e.value().attr("value"))
        .unwrap();
    client
        .post(&login_url)
        .form(&[
            ("authenticity_token", authenticity_token),
            ("username", "admin"),
            ("password", "admin"),
        ])
        .send()
        .await?;
    trace!("Redmine {version} logged in as admin");
    let password_change_url = format!("{}/my/password", url);
    let password_change_page = client
        .get(&password_change_url)
        .send()
        .await?
        .text()
        .await?;
    let doc = Html::parse_document(&password_change_page);
    let authenticity_token = doc
        .select(&selector)
        .next()
        .and_then(|e| e.value().attr("value"))
        .unwrap();
    let new_password = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    client
        .post(&password_change_url)
        .form(&[
            ("authenticity_token", authenticity_token),
            ("password", "admin"),
            ("new_password", &new_password),
            ("new_password_confirmation", &new_password),
        ])
        .send()
        .await?;
    trace!("Redmine {version} password changed");
    let settings_url = format!("{}/settings?tab=api", url);
    let settings_page = client.get(&settings_url).send().await?.text().await?;
    let doc = Html::parse_document(&settings_page);
    let authenticity_token = doc
        .select(&selector)
        .next()
        .and_then(|e| e.value().attr("value"))
        .unwrap();
    client
        .post(&settings_url)
        .form(&[
            ("authenticity_token", authenticity_token),
            ("settings[api_enabled]", "1"),
            ("settings[jsonp_enabled]", "0"),
            ("commit", "Save"),
        ])
        .send()
        .await?;
    trace!("Redmine {version} REST API enabled");
    let admin_url = format!("{}/admin", url);
    let admin_page = client.get(&admin_url).send().await?.text().await?;
    let doc = Html::parse_document(&admin_page);
    let authenticity_token = doc
        .select(&selector)
        .next()
        .and_then(|e| e.value().attr("value"))
        .unwrap();
    let default_config_url = format!("{}/admin/default_configuration", url);
    client
        .post(&default_config_url)
        .form(&[
            ("authenticity_token", authenticity_token),
            ("lang", "en"),
            ("commit", "Load the default configuration"),
        ])
        .send()
        .await?;
    trace!("Redmine {version} default data loaded");
    let api_key_url = format!("{}/my/api_key", url);
    let api_key_page = client.get(&api_key_url).send().await?.text().await?;
    let doc = Html::parse_document(&api_key_page);
    let selector = Selector::parse("#content .box pre").unwrap();
    let api_key = doc.select(&selector).next().unwrap().inner_html();
    trace!("Redmine {version} container API key is {api_key}");
    let client = reqwest::Client::builder().use_rustls_tls().build()?;
    let redmine = RedmineAsync::new(client, url.parse()?, &api_key)?;
    trace!("Redmine {version} client created");
    f(&redmine, &container).await?;
    trace!("Redmine {version} test closure finished");
    Ok(())
}
