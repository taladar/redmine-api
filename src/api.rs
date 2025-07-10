//! Redmine API
//!
//! [`Redmine Documentation`](https://www.redmine.org/projects/redmine/wiki/rest_api)
//!
//! - [x] authentication
//! - [x] pagination
//!   - [x] add Pageable instances to all types that need them
//!   - [x] figure out a way to write a general "fetch all pages" function (problem is the different key name in the wrapper)
//! - [x] impersonation
//! - [x] attachments
//! - [x] add all the wrappers I somehow missed
//!   - [x] check if admin and send_information truly are not part of the user hash in Create/UpdateUser or if the wiki docs are wrong (admin is, send_information is not)
//! - [x] test include parameters and add relevant data to the return types
//! - [x] async support
//!
//! Potential breaking changes ahead
//! - [ ] use Enum for sort column
//! - [ ] typed ids
//! - [ ] change project_id_or_name to Enum
//! - [ ] extra filter expressions I overlooked/did not know about
//! - [ ] parameters that are more flexible than they appear

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
#[cfg(test)]
pub mod test_helpers;
pub mod time_entries;
pub mod trackers;
pub mod uploads;
pub mod users;
pub mod versions;
pub mod wiki_pages;

use std::str::from_utf8;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

use reqwest::Method;
use std::borrow::Cow;

use reqwest::Url;
use tracing::{debug, error, trace};

/// main API client object (sync)
#[derive(derive_more::Debug)]
pub struct Redmine {
    /// the reqwest client we use to perform our API requests
    client: reqwest::blocking::Client,
    /// the redmine base url
    redmine_url: Url,
    /// a redmine API key, usually 40 hex digits where the letters (a-f) are lower case
    #[debug(skip)]
    api_key: String,
    /// the user id we want to impersonate, only works if the API key we use has admin privileges
    impersonate_user_id: Option<u64>,
}

/// main API client object (async)
#[derive(derive_more::Debug)]
pub struct RedmineAsync {
    /// the reqwest client we use to perform our API requests
    client: reqwest::Client,
    /// the redmine base url
    redmine_url: Url,
    /// a redmine API key, usually 40 hex digits where the letters (a-f) are lower case
    #[debug(skip)]
    api_key: String,
    /// the user id we want to impersonate, only works if the API key we use has admin privileges
    impersonate_user_id: Option<u64>,
}

/// helper function to parse the redmine URL in the environment variable
fn parse_url<'de, D>(deserializer: D) -> Result<url::Url, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    url::Url::parse(&buf).map_err(serde::de::Error::custom)
}

/// used to deserialize the required options from the environment
#[derive(Debug, Clone, serde::Deserialize)]
struct EnvOptions {
    /// a redmine API key, usually 40 hex digits where the letters (a-f) are lower case
    redmine_api_key: String,

    /// the redmine base url
    #[serde(deserialize_with = "parse_url")]
    redmine_url: url::Url,
}

/// Return value from paged requests, includes the actual value as well as
/// pagination data
#[derive(Debug, Clone)]
pub struct ResponsePage<T> {
    /// The actual value returned by Redmine deserialized into a user provided type
    pub values: Vec<T>,
    /// The total number of values that could be returned by requesting all pages
    pub total_count: u64,
    /// The offset from the start (zero-based)
    pub offset: u64,
    /// How many entries were returned
    pub limit: u64,
}

impl Redmine {
    /// create a [Redmine] object
    ///
    /// # Errors
    ///
    /// This will return [`crate::Error::ReqwestError`] if initialization of Reqwest client is failed.
    pub fn new(
        client: reqwest::blocking::Client,
        redmine_url: url::Url,
        api_key: &str,
    ) -> Result<Self, crate::Error> {
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
    ///
    /// # Errors
    ///
    /// This will return an error if the environment variables are
    /// missing or the URL can not be parsed
    pub fn from_env(client: reqwest::blocking::Client) -> Result<Self, crate::Error> {
        let env_options = envy::from_env::<EnvOptions>()?;

        let redmine_url = env_options.redmine_url;
        let api_key = env_options.redmine_api_key;

        Self::new(client, redmine_url, &api_key)
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
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn issue_url(&self, issue_id: u64) -> Url {
        let Redmine { redmine_url, .. } = self;
        // we can unwrap here because we know /issues/<number>
        // parses successfully as an url fragment
        redmine_url.join(&format!("/issues/{issue_id}")).unwrap()
    }

    /// internal method for shared logic between the methods below which
    /// diff in how they parse the response body and how often they call this
    fn rest(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        parameters: QueryParams,
        mime_type_and_body: Option<(&str, Vec<u8>)>,
    ) -> Result<(reqwest::StatusCode, bytes::Bytes), crate::Error> {
        let Redmine {
            client,
            redmine_url,
            api_key,
            impersonate_user_id,
        } = self;
        let mut url = redmine_url.join(endpoint)?;
        parameters.add_to_url(&mut url);
        debug!(%url, %method, "Calling redmine");
        let req = client
            .request(method.clone(), url.clone())
            .header("x-redmine-api-key", api_key);
        let req = if let Some(user_id) = impersonate_user_id {
            req.header("X-Redmine-Switch-User", format!("{user_id}"))
        } else {
            req
        };
        let req = if let Some((mime, data)) = mime_type_and_body {
            if let Ok(request_body) = from_utf8(&data) {
                trace!("Request body (Content-Type: {}):\n{}", mime, request_body);
            } else {
                trace!(
                    "Request body (Content-Type: {}) could not be parsed as UTF-8:\n{:?}",
                    mime,
                    data
                );
            }
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
        Ok((status, response_body))
    }

    /// use this with endpoints that have no response body, e.g. those just deleting
    /// a Redmine object
    ///
    /// # Errors
    ///
    /// This can return an error if the endpoint returns an error when creating the request
    /// body or when the web request fails
    pub fn ignore_response_body<E>(&self, endpoint: &E) -> Result<(), crate::Error>
    where
        E: Endpoint,
    {
        let method = endpoint.method();
        let url = endpoint.endpoint();
        let parameters = endpoint.parameters();
        let mime_type_and_body = endpoint.body()?;
        self.rest(method, &url, parameters, mime_type_and_body)?;
        Ok(())
    }

    /// use this with endpoints which return a JSON response but do not support pagination
    ///
    /// # Errors
    ///
    /// This can return an error if the endpoint returns an error when creating the request body,
    /// when the web request fails or when the response can not be parsed as a JSON object
    /// into the result type
    pub fn json_response_body<E, R>(&self, endpoint: &E) -> Result<R, crate::Error>
    where
        E: Endpoint + ReturnsJsonResponse + NoPagination,
        R: DeserializeOwned + std::fmt::Debug,
    {
        let method = endpoint.method();
        let url = endpoint.endpoint();
        let parameters = endpoint.parameters();
        let mime_type_and_body = endpoint.body()?;
        let (status, response_body) = self.rest(method, &url, parameters, mime_type_and_body)?;
        if response_body.is_empty() {
            Err(crate::Error::EmptyResponseBody(status))
        } else {
            let result = serde_json::from_slice::<R>(&response_body);
            if let Ok(ref parsed_response_body) = result {
                trace!("Parsed response body:\n{:#?}", parsed_response_body);
            }
            Ok(result?)
        }
    }

    /// use this to get a single page of a paginated JSON response
    /// # Errors
    ///
    /// This can return an error if the endpoint returns an error when creating the
    /// request body, when the web request fails, when the response can not be parsed
    /// as a JSON object, when any of the pagination keys or the value key are missing
    /// in the JSON object or when the values can not be parsed as the result type.
    pub fn json_response_body_page<E, R>(
        &self,
        endpoint: &E,
        offset: u64,
        limit: u64,
    ) -> Result<ResponsePage<R>, crate::Error>
    where
        E: Endpoint + ReturnsJsonResponse + Pageable,
        R: DeserializeOwned + std::fmt::Debug,
    {
        let method = endpoint.method();
        let url = endpoint.endpoint();
        let mut parameters = endpoint.parameters();
        parameters.push("offset", offset);
        parameters.push("limit", limit);
        let mime_type_and_body = endpoint.body()?;
        let (status, response_body) = self.rest(method, &url, parameters, mime_type_and_body)?;
        if response_body.is_empty() {
            Err(crate::Error::EmptyResponseBody(status))
        } else {
            let json_value_response_body: serde_json::Value =
                serde_json::from_slice(&response_body)?;
            let json_object_response_body = json_value_response_body.as_object();
            if let Some(json_object_response_body) = json_object_response_body {
                let total_count = json_object_response_body
                    .get("total_count")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("total_count".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let offset = json_object_response_body
                    .get("offset")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("offset".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let limit = json_object_response_body
                    .get("limit")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("limit".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let response_wrapper_key = endpoint.response_wrapper_key();
                let inner_response_body = json_object_response_body
                    .get(&response_wrapper_key)
                    .ok_or(crate::Error::PaginationKeyMissing(response_wrapper_key))?;
                let result = serde_json::from_value::<Vec<R>>(inner_response_body.to_owned());
                if let Ok(ref parsed_response_body) = result {
                    trace!(%total_count, %offset, %limit, "Parsed response body:\n{:?}", parsed_response_body);
                }
                Ok(ResponsePage {
                    values: result?,
                    total_count,
                    offset,
                    limit,
                })
            } else {
                Err(crate::Error::NonObjectResponseBody(status))
            }
        }
    }

    /// use this to get the results for all pages of a paginated JSON response
    ///
    /// # Errors
    ///
    /// This can return an error if the endpoint returns an error when creating the
    /// request body, when any of the web requests fails, when the response can not be
    /// parsed as a JSON object, when any of the pagination keys or the value key are missing
    /// in the JSON object or when the values can not be parsed as the result type.
    ///
    pub fn json_response_body_all_pages<E, R>(&self, endpoint: &E) -> Result<Vec<R>, crate::Error>
    where
        E: Endpoint + ReturnsJsonResponse + Pageable,
        R: DeserializeOwned + std::fmt::Debug,
    {
        let method = endpoint.method();
        let url = endpoint.endpoint();
        let mut offset = 0;
        let limit = 100;
        let mut total_results = vec![];
        loop {
            let mut page_parameters = endpoint.parameters();
            page_parameters.push("offset", offset);
            page_parameters.push("limit", limit);
            let mime_type_and_body = endpoint.body()?;
            let (status, response_body) =
                self.rest(method.clone(), &url, page_parameters, mime_type_and_body)?;
            if response_body.is_empty() {
                return Err(crate::Error::EmptyResponseBody(status));
            }
            let json_value_response_body: serde_json::Value =
                serde_json::from_slice(&response_body)?;
            let json_object_response_body = json_value_response_body.as_object();
            if let Some(json_object_response_body) = json_object_response_body {
                let total_count: u64 = json_object_response_body
                    .get("total_count")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("total_count".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let response_offset: u64 = json_object_response_body
                    .get("offset")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("offset".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let response_limit: u64 = json_object_response_body
                    .get("limit")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("limit".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let response_wrapper_key = endpoint.response_wrapper_key();
                let inner_response_body = json_object_response_body
                    .get(&response_wrapper_key)
                    .ok_or(crate::Error::PaginationKeyMissing(response_wrapper_key))?;
                let result = serde_json::from_value::<Vec<R>>(inner_response_body.to_owned());
                if let Ok(ref parsed_response_body) = result {
                    trace!(%total_count, %offset, %limit, "Parsed response body:\n{:?}", parsed_response_body);
                }
                total_results.extend(result?);
                if total_count < (response_offset + response_limit) {
                    break;
                }
                offset += limit;
            } else {
                return Err(crate::Error::NonObjectResponseBody(status));
            }
        }
        Ok(total_results)
    }
}

impl RedmineAsync {
    /// create a [RedmineAsync] object
    ///
    /// # Errors
    ///
    /// This will return [`crate::Error::ReqwestError`] if initialization of Reqwest client is failed.
    pub fn new(
        client: reqwest::Client,
        redmine_url: url::Url,
        api_key: &str,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            client,
            redmine_url,
            api_key: api_key.to_string(),
            impersonate_user_id: None,
        })
    }

    /// create a [RedmineAsync] object from the environment variables
    ///
    /// REDMINE_API_KEY
    /// REDMINE_URL
    ///
    /// # Errors
    ///
    /// This will return an error if the environment variables are
    /// missing or the URL can not be parsed
    pub fn from_env(client: reqwest::Client) -> Result<Self, crate::Error> {
        let env_options = envy::from_env::<EnvOptions>()?;

        let redmine_url = env_options.redmine_url;
        let api_key = env_options.redmine_api_key;

        Self::new(client, redmine_url, &api_key)
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
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn issue_url(&self, issue_id: u64) -> Url {
        let RedmineAsync { redmine_url, .. } = self;
        // we can unwrap here because we know /issues/<number>
        // parses successfully as an url fragment
        redmine_url.join(&format!("/issues/{issue_id}")).unwrap()
    }

    /// internal method for shared logic between the methods below which
    /// diff in how they parse the response body and how often they call this
    async fn rest(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        parameters: QueryParams<'_>,
        mime_type_and_body: Option<(&str, Vec<u8>)>,
    ) -> Result<(reqwest::StatusCode, bytes::Bytes), crate::Error> {
        let RedmineAsync {
            client,
            redmine_url,
            api_key,
            impersonate_user_id,
        } = self;
        let mut url = redmine_url.join(endpoint)?;
        parameters.add_to_url(&mut url);
        debug!(%url, %method, "Calling redmine");
        let req = client
            .request(method.clone(), url.clone())
            .header("x-redmine-api-key", api_key);
        let req = if let Some(user_id) = impersonate_user_id {
            req.header("X-Redmine-Switch-User", format!("{user_id}"))
        } else {
            req
        };
        let req = if let Some((mime, data)) = mime_type_and_body {
            if let Ok(request_body) = from_utf8(&data) {
                trace!("Request body (Content-Type: {}):\n{}", mime, request_body);
            } else {
                trace!(
                    "Request body (Content-Type: {}) could not be parsed as UTF-8:\n{:?}",
                    mime,
                    data
                );
            }
            req.body(data).header("Content-Type", mime)
        } else {
            req
        };
        let result = req.send().await;
        if let Err(ref e) = result {
            error!(%url, %method, "Redmine send error: {:?}", e);
        }
        let result = result?;
        let status = result.status();
        let response_body = result.bytes().await?;
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
        Ok((status, response_body))
    }

    /// use this with endpoints that have no response body, e.g. those just deleting
    /// a Redmine object
    ///
    /// # Errors
    ///
    /// This can return an error if the endpoint returns an error when creating the request
    /// body or when the web request fails
    pub async fn ignore_response_body<E>(&self, endpoint: &E) -> Result<(), crate::Error>
    where
        E: Endpoint,
    {
        let method = endpoint.method();
        let url = endpoint.endpoint();
        let parameters = endpoint.parameters();
        let mime_type_and_body = endpoint.body()?;
        self.rest(method, &url, parameters, mime_type_and_body)
            .await?;
        Ok(())
    }

    /// use this with endpoints which return a JSON response but do not support pagination
    ///
    /// you can use it with those that support pagination but they will only return the first page
    ///
    /// # Errors
    ///
    /// This can return an error if the endpoint returns an error when creating the request body,
    /// when the web request fails or when the response can not be parsed as a JSON object
    /// into the result type
    pub async fn json_response_body<E, R>(&self, endpoint: &E) -> Result<R, crate::Error>
    where
        E: Endpoint + ReturnsJsonResponse + NoPagination,
        R: DeserializeOwned + std::fmt::Debug,
    {
        let method = endpoint.method();
        let url = endpoint.endpoint();
        let parameters = endpoint.parameters();
        let mime_type_and_body = endpoint.body()?;
        let (status, response_body) = self
            .rest(method, &url, parameters, mime_type_and_body)
            .await?;
        if response_body.is_empty() {
            Err(crate::Error::EmptyResponseBody(status))
        } else {
            let result = serde_json::from_slice::<R>(&response_body);
            if let Ok(ref parsed_response_body) = result {
                trace!("Parsed response body:\n{:#?}", parsed_response_body);
            }
            Ok(result?)
        }
    }

    /// use this to get a single page of a paginated JSON response
    /// # Errors
    ///
    /// This can return an error if the endpoint returns an error when creating the
    /// request body, when the web request fails, when the response can not be parsed
    /// as a JSON object, when any of the pagination keys or the value key are missing
    /// in the JSON object or when the values can not be parsed as the result type.
    pub async fn json_response_body_page<E, R>(
        &self,
        endpoint: &E,
        offset: u64,
        limit: u64,
    ) -> Result<ResponsePage<R>, crate::Error>
    where
        E: Endpoint + ReturnsJsonResponse + Pageable,
        R: DeserializeOwned + std::fmt::Debug,
    {
        let method = endpoint.method();
        let url = endpoint.endpoint();
        let mut parameters = endpoint.parameters();
        parameters.push("offset", offset);
        parameters.push("limit", limit);
        let mime_type_and_body = endpoint.body()?;
        let (status, response_body) = self
            .rest(method, &url, parameters, mime_type_and_body)
            .await?;
        if response_body.is_empty() {
            Err(crate::Error::EmptyResponseBody(status))
        } else {
            let json_value_response_body: serde_json::Value =
                serde_json::from_slice(&response_body)?;
            let json_object_response_body = json_value_response_body.as_object();
            if let Some(json_object_response_body) = json_object_response_body {
                let total_count = json_object_response_body
                    .get("total_count")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("total_count".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let offset = json_object_response_body
                    .get("offset")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("offset".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let limit = json_object_response_body
                    .get("limit")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("limit".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let response_wrapper_key = endpoint.response_wrapper_key();
                let inner_response_body = json_object_response_body
                    .get(&response_wrapper_key)
                    .ok_or(crate::Error::PaginationKeyMissing(response_wrapper_key))?;
                let result = serde_json::from_value::<Vec<R>>(inner_response_body.to_owned());
                if let Ok(ref parsed_response_body) = result {
                    trace!(%total_count, %offset, %limit, "Parsed response body:\n{:?}", parsed_response_body);
                }
                Ok(ResponsePage {
                    values: result?,
                    total_count,
                    offset,
                    limit,
                })
            } else {
                Err(crate::Error::NonObjectResponseBody(status))
            }
        }
    }

    /// use this to get the results for all pages of a paginated JSON response
    ///
    /// # Errors
    ///
    /// This can return an error if the endpoint returns an error when creating the
    /// request body, when any of the web requests fails, when the response can not be
    /// parsed as a JSON object, when any of the pagination keys or the value key are missing
    /// in the JSON object or when the values can not be parsed as the result type.
    ///
    pub async fn json_response_body_all_pages<E, R>(
        &self,
        endpoint: &E,
    ) -> Result<Vec<R>, crate::Error>
    where
        E: Endpoint + ReturnsJsonResponse + Pageable,
        R: DeserializeOwned + std::fmt::Debug,
    {
        let method = endpoint.method();
        let url = endpoint.endpoint();
        let mut offset = 0;
        let limit = 100;
        let mut total_results = vec![];
        loop {
            let mut page_parameters = endpoint.parameters();
            page_parameters.push("offset", offset);
            page_parameters.push("limit", limit);
            let mime_type_and_body = endpoint.body()?;
            let (status, response_body) = self
                .rest(method.clone(), &url, page_parameters, mime_type_and_body)
                .await?;
            if response_body.is_empty() {
                return Err(crate::Error::EmptyResponseBody(status));
            }
            let json_value_response_body: serde_json::Value =
                serde_json::from_slice(&response_body)?;
            let json_object_response_body = json_value_response_body.as_object();
            if let Some(json_object_response_body) = json_object_response_body {
                let total_count: u64 = json_object_response_body
                    .get("total_count")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("total_count".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let response_offset: u64 = json_object_response_body
                    .get("offset")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("offset".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let response_limit: u64 = json_object_response_body
                    .get("limit")
                    .ok_or_else(|| crate::Error::PaginationKeyMissing("limit".to_string()))?
                    .as_u64()
                    .ok_or_else(|| {
                        crate::Error::PaginationKeyHasWrongType("total_count".to_string())
                    })?;
                let response_wrapper_key = endpoint.response_wrapper_key();
                let inner_response_body = json_object_response_body
                    .get(&response_wrapper_key)
                    .ok_or(crate::Error::PaginationKeyMissing(response_wrapper_key))?;
                let result = serde_json::from_value::<Vec<R>>(inner_response_body.to_owned());
                if let Ok(ref parsed_response_body) = result {
                    trace!(%total_count, %offset, %limit, "Parsed response body:\n{:?}", parsed_response_body);
                }
                total_results.extend(result?);
                if total_count < (response_offset + response_limit) {
                    break;
                }
                offset += limit;
            } else {
                return Err(crate::Error::NonObjectResponseBody(status));
            }
        }
        Ok(total_results)
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

/// serialize a [`Vec<T>`] where T implements [ToString] as a string
/// of comma-separated values
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
/// of comma-separated values
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
        format!("{self}").into()
    }
}

impl ParamValue<'static> for f64 {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{self}").into()
    }
}

impl ParamValue<'static> for time::OffsetDateTime {
    fn as_value(&self) -> Cow<'static, str> {
        self.format(&time::format_description::well_known::Rfc3339)
            .unwrap()
            .into()
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
    /// the actual parameters
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
    ///
    /// # Errors
    ///
    /// The default implementation will never return an error
    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(None)
    }
}

/// A trait to indicate that an endpoint is expected to return a JSON result
pub trait ReturnsJsonResponse {}

/// A trait to indicate that an endpoint requires pagination to yield all results
/// or in other words that the non-pagination API should not be used on it or one
/// might miss some results
#[diagnostic::on_unimplemented(
    message = "{Self} is an endpoint that requires pagination, use `.json_response_body_page(&endpoint, offset, limit)` or `.json_response_body_all_pages(&endpoint)` instead of `.json_response_body(&endpoint)`"
)]
pub trait NoPagination {}

/// A trait to indicate that an endpoint is pageable.
#[diagnostic::on_unimplemented(
    message = "{Self} is an endpoint that does not implement pagination, use `.json_response_body(&endpoint)` instead of `.json_response_body_page(&endpoint, offset, limit)` or `.json_response_body_all_pages(&endpoint)`"
)]
pub trait Pageable {
    /// returns the name of the key in the response that contains the list of results
    fn response_wrapper_key(&self) -> String;
}

/// helper to parse created_on and updated_on in the correct format
/// (default time serde implementation seems to use a different format)
///
/// # Errors
///
/// This will return an error if the underlying string can not be deserialized or
/// can not be parsed as an RFC3339 date and time
pub fn deserialize_rfc3339<'de, D>(deserializer: D) -> Result<time::OffsetDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    time::OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
        .map_err(serde::de::Error::custom)
}

/// helper to serialize created_on and updated_on in the correct format
/// (default time serde implementation seems to use a different format)
///
/// # Errors
///
/// This will return an error if the date time can not be formatted as an RFC3339
/// date time or the resulting string can not be serialized
pub fn serialize_rfc3339<S>(t: &time::OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = t
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(serde::ser::Error::custom)?;

    s.serialize(serializer)
}

/// helper to parse created_on and updated_on in the correct format
/// (default time serde implementation seems to use a different format)
///
/// # Errors
///
/// This will return an error if the underlying string can not be deserialized
/// or it can not be parsed as an RFC3339 date and time
pub fn deserialize_optional_rfc3339<'de, D>(
    deserializer: D,
) -> Result<Option<time::OffsetDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = <Option<String> as Deserialize<'de>>::deserialize(deserializer)?;

    if let Some(s) = s {
        Ok(Some(
            time::OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
                .map_err(serde::de::Error::custom)?,
        ))
    } else {
        Ok(None)
    }
}

/// helper to serialize created_on and updated_on in the correct format
/// (default time serde implementation seems to use a different format)
///
/// # Errors
///
/// This will return an error if the parameter can not be formatted as RFC3339
/// or the resulting string can not be serialized
pub fn serialize_optional_rfc3339<S>(
    t: &Option<time::OffsetDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(t) = t {
        let s = t
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(serde::ser::Error::custom)?;

        s.serialize(serializer)
    } else {
        let n: Option<String> = None;
        n.serialize(serializer)
    }
}
