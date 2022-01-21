//! Redmine API
//!
//! [`Redmine Documentation`](https://www.redmine.org/projects/redmine/wiki/rest_api)
//!
//! - [x] authentication
//! - [ ] pagination
//!   - [ ] add Pageable instances to all types that need them
//!   - [ ] figure out a way to write a general "fetch all pages" function (problem is the different key name in the wrapper)
//! - [x] impersonation
//! - [ ] attachments
//! - [ ] add all the wrappers I somehow missed
//!   - [ ] check if admin and send_information truly are not part of the user hash in Create/UpdateUser or if the wiki docs are wrong
//!
//! Potential breaking changes ahead
//! - [ ] use Enum for sort column
//! - [ ] typed ids
//! - [ ] change project_id_or_name to Enum
//! - [ ] extra filter expressions I overlooked/did not know about
//! - [ ] parameters that are more flexible than they appear
//! - [ ] async support?

pub mod attachments;
pub mod custom_fields;
pub mod enumerations;
pub mod files;
pub mod groups;
pub mod issue_categories;
pub mod issue_relations;
pub mod issue_statuses;
pub mod issues;
pub mod my_account;
pub mod news;
pub mod project_memberships;
pub mod projects;
pub mod queries;
pub mod roles;
pub mod search;
pub mod time_entries;
pub mod trackers;
pub mod users;
pub mod versions;
pub mod wiki_pages;

use std::str::from_utf8;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Deserializer;

use http::Method;
use std::borrow::Cow;

use reqwest::{blocking::Client, Url};
use tracing::{debug, error, trace};

fn parse_url<'de, D>(deserializer: D) -> Result<url::Url, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    url::Url::parse(&buf).map_err(serde::de::Error::custom)
}

/// main API client object
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Redmine {
    client: Client,
    redmine_url: Url,
    #[derivative(Debug="ignore")]
    api_key: String,
    impersonate_user_id: Option<u64>,
}

/// used to deserialize the required options from the environment
#[derive(Debug, serde::Deserialize)]
struct EnvOptions {
    redmine_api_key: String,

    #[serde(deserialize_with = "parse_url")]
    redmine_url: url::Url,
}

#[derive(Debug, Clone, Deserialize)]
struct ResponseCounts {
    total_count: u64,
    offset: u64,
    limit: u64,
}

/// Return value from paged requests, includes the actual value as well as
/// pagination data
#[derive(Debug, Clone)]
pub struct ResponsePage<T> {
    /// The actual value returned by Redmine deserialized into a user provided type
    pub value: T,
    /// The total number of values that could be returned by requesting all pages
    pub total_count: u64,
    /// The offset from the start (zero-based)
    pub offset: u64,
    /// How many entries were returned
    pub limit: u64,
}

impl Redmine {
    /// create a [Redmine] object
    pub fn new(redmine_url: url::Url, api_key: &str) -> Result<Self, crate::Error> {
        let client = Client::new();

        Ok(Self {
            client,
            redmine_url,
            api_key: api_key.to_string(),
            impersonate_user_id: None,
        })
    }

    /// create a [Redmine] object from the environment variables
    ///
    /// REDMINE_API_KEY
    /// REDMINE_URL
    pub fn from_env() -> Result<Self, crate::Error> {
        let env_options = envy::from_env::<EnvOptions>()?;

        let redmine_url = env_options.redmine_url;
        let api_key = env_options.redmine_api_key;

        Self::new(redmine_url, &api_key)
    }

    /// Sets the user id of a user to impersonate in all future API calls
    ///
    /// this requires Redmine admin privileges
    pub fn impersonate_user(&mut self, id: u64) {
        self.impersonate_user_id = Some(id);
    }

    /// returns the issue URL for a given issue id
    ///
    /// this is mostly for convenience since we are already storing the
    /// redmine URL and it works entirely on the client
    pub fn issue_url(&self, issue_id: u64) -> Url {
        let Redmine { redmine_url, .. } = self;
        // we can unwrap here because we know /issues/<number>
        // parses successfully as an url fragment
        redmine_url.join(&format!("/issues/{}", issue_id)).unwrap()
    }

    /// call the given endpoint and return a type that is provided by the user
    /// and deserializes the JSON returned by the API
    ///
    /// The advantage of this is that the user can skip any fields they do not
    /// care about.
    ///
    /// a typical use could look like this
    ///
    /// ```
    /// use redmine_api::api::Redmine;
    ///
    /// #[derive(Debug, serde::Deserialize)]
    /// pub struct User {
    ///    pub id: u64,
    /// }
    ///
    /// #[derive(Debug, serde::Deserialize)]
    /// pub struct MyAccountResponse {
    ///     pub user: User,
    /// }
    ///
    /// pub fn whoami(
    ///     redmine : &Redmine,
    /// ) -> Result<u64, Box<dyn std::error::Error>> {
    ///     let endpoint = redmine_api::api::my_account::MyAccount::builder().build()?;
    ///
    ///     Ok(redmine.rest::<_, MyAccountResponse>(&endpoint)?.user.id)
    /// }
    /// ```
    ///
    /// If this is used with an [Endpoint] which is [Pageable] this will
    /// only return the first page by whatever is set as the default page
    /// size in Redmine. Use [Redmine::rest_page] instead.
    pub fn rest<E, R>(&self, endpoint: &E) -> Result<R, crate::Error>
    where
        E: Endpoint,
        R: DeserializeOwned + std::fmt::Debug,
    {
        let Redmine {
            client,
            redmine_url,
            api_key,
            impersonate_user_id,
        } = self;
        let mut url = redmine_url.join(&endpoint.endpoint())?;
        endpoint.parameters().add_to_url(&mut url);
        let method = endpoint.method();
        debug!(%url, %method, "Calling redmine");
        let req = client
            .request(method.clone(), url.clone())
            .header("x-redmine-api-key", api_key);
        let req = if let Some(user_id) = impersonate_user_id {
            req.header("X-Redmine-Switch-User", format!("{}", user_id))
        } else { req };
        let req = if let Some((mime, data)) = endpoint.body()? {
            req.body(data).header("Content-Type", mime)
        } else {
            req
        };
        let result = req.send();
        if let Err(ref e) = result {
            error!(%url, %method, "Redmine send error: {:?}", e);
        }
        let result = result?;
        let status = result.status();
        let response_body = result.bytes()?;
        match from_utf8(&response_body) {
            Ok(response_body) => {
                trace!("Response body:\n{}", &response_body);
            }
            Err(e) => {
                trace!(
                    "Response body that could not be parsed as utf8 because of {}:\n{:?}",
                    &e,
                    &response_body
                );
            }
        }
        if status.is_client_error() {
            error!(%url, %method, "Redmine status error (client error): {:?}", status);
        } else if status.is_server_error() {
            error!(%url, %method, "Redmine status error (server error): {:?}", status);
        }
        let result = serde_json::from_slice::<R>(&response_body);
        if let Ok(ref parsed_response_body) = result {
            trace!("Parsed response body:\n{:?}", parsed_response_body);
        }
        Ok(result?)
    }

    /// call the given pageable endpoint and return a type that is provided by the user
    /// as well as the total_count, offset and limit values returned by Redmine
    pub fn rest_page<E, R>(&self, endpoint: &E, offset: u64, limit: u64) -> Result<ResponsePage<R>, crate::Error>
    where
        E: Endpoint + Pageable,
        R: DeserializeOwned + std::fmt::Debug,
    {
        let Redmine {
            client,
            redmine_url,
            api_key,
            impersonate_user_id,
        } = self;
        let mut url = redmine_url.join(&endpoint.endpoint())?;
        let mut parameters = endpoint.parameters();
        parameters.push("offset", offset);
        parameters.push("limit", limit);
        parameters.add_to_url(&mut url);
        let method = endpoint.method();
        debug!(%url, %method, %offset, %limit, "Calling redmine");
        let req = client
            .request(method.clone(), url.clone())
            .header("x-redmine-api-key", api_key);
        let req = if let Some(user_id) = impersonate_user_id {
            req.header("X-Redmine-Switch-User", format!("{}", user_id))
        } else { req };
        let req = if let Some((mime, data)) = endpoint.body()? {
            req.body(data).header("Content-Type", mime)
        } else {
            req
        };
        let result = req.send();
        if let Err(ref e) = result {
            error!(%url, %method, %offset, %limit, "Redmine send error: {:?}", e);
        }
        let result = result?;
        let status = result.status();
        let response_body = result.bytes()?;
        match from_utf8(&response_body) {
            Ok(response_body) => {
                trace!("Response body:\n{}", &response_body);
            }
            Err(e) => {
                trace!(
                    "Response body that could not be parsed as utf8 because of {}:\n{:?}",
                    &e,
                    &response_body
                );
            }
        }
        if status.is_client_error() {
            error!(%url, %method, "Redmine status error (client error): {:?}", status);
        } else if status.is_server_error() {
            error!(%url, %method, "Redmine status error (server error): {:?}", status);
        }
        let response_counts = serde_json::from_slice(&response_body);
        if let Err(ref e) = response_counts {
            error!(%url, %method, %offset, %limit, "Failed parsing the response counts supplied by Redmine: {}", e);
        }
        let ResponseCounts { total_count, offset, limit } = response_counts?;
        let result = serde_json::from_slice::<R>(&response_body);
        if let Ok(ref parsed_response_body) = result {
            trace!("Parsed response body:\n{:?}", parsed_response_body);
        }
        Ok(ResponsePage { value: result?, total_count, offset, limit } )
    }
}

/// A trait representing a parameter value.
pub trait ParamValue<'a> {
    #[allow(clippy::wrong_self_convention)]
    /// The parameter value as a string.
    fn as_value(&self) -> Cow<'a, str>;
}

impl ParamValue<'static> for bool {
    fn as_value(&self) -> Cow<'static, str> {
        if *self {
            "true".into()
        } else {
            "false".into()
        }
    }
}

impl<'a> ParamValue<'a> for &'a str {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl ParamValue<'static> for String {
    fn as_value(&self) -> Cow<'static, str> {
        self.clone().into()
    }
}

impl<'a> ParamValue<'a> for &'a String {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

/// serialize a [Vec<T>] where T implements [ToString] as a string
/// of comma-seperated values
impl<T> ParamValue<'static> for Vec<T>
where
    T: ToString,
{
    fn as_value(&self) -> Cow<'static, str> {
        self.iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(",")
            .into()
    }
}

/// serialize a [`&Vec<T>`](Vec<T>) where T implements [ToString] as a string
/// of comma-seperated values
impl<'a, T> ParamValue<'a> for &'a Vec<T>
where
    T: ToString,
{
    fn as_value(&self) -> Cow<'a, str> {
        self.iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(",")
            .into()
    }
}

impl<'a> ParamValue<'a> for Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        self.clone()
    }
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).clone()
    }
}

impl ParamValue<'static> for u64 {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self).into()
    }
}

impl ParamValue<'static> for f64 {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self).into()
    }
}

impl ParamValue<'static> for time::OffsetDateTime {
    fn as_value(&self) -> Cow<'static, str> {
        self.format(&time::format_description::well_known::Rfc3339).unwrap().into()
    }
}

impl ParamValue<'static> for time::Date {
    fn as_value(&self) -> Cow<'static, str> {
        let format = time::format_description::parse("[year]-[month]-[day]").unwrap();
        self.format(&format).unwrap().into()
    }
}

/// A structure for query parameters.
#[derive(Debug, Default, Clone)]
pub struct QueryParams<'a> {
    params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> QueryParams<'a> {
    /// Push a single parameter.
    pub fn push<'b, K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params.push((key.into(), value.as_value()));
        self
    }

    /// Push a single parameter.
    pub fn push_opt<'b, K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        if let Some(value) = value {
            self.params.push((key.into(), value.as_value()));
        }
        self
    }

    /// Push a set of parameters.
    pub fn extend<'b, I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params
            .extend(iter.map(|(key, value)| (key.into(), value.as_value())));
        self
    }

    /// Add the parameters to a URL.
    pub fn add_to_url(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();
        pairs.extend_pairs(self.params.iter());
    }
}

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;
    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;

    /// Query parameters for the endpoint.
    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    /// The body for the endpoint.
    ///
    /// Returns the `Content-Encoding` header for the data as well as the data itself.
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(None)
    }
}

/// A trait to indicate that an endpoint is pageable.
pub trait Pageable {}
