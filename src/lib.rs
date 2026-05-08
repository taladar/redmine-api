#![expect(
    single_use_lifetimes,
    reason = "endpoint structs hold Cow<'a, str> for cheap borrowed string parameters; the lifetime appears once in the struct but propagates to every call site"
)]
#![expect(
    clippy::module_name_repetitions,
    reason = "Redmine REST resource types share their module's name (e.g. ListIssues in api::issues) for self-documenting public re-exports"
)]
#![expect(
    clippy::future_not_send,
    reason = "the async client is not Send by design; the underlying reqwest::Client is shared via Arc"
)]
#![cfg_attr(
    test,
    expect(
        unreachable_pub,
        clippy::unwrap_used,
        clippy::panic,
        clippy::indexing_slicing,
        clippy::arithmetic_side_effects,
        clippy::print_stderr,
        reason = "tests use panicking patterns and helper items idiomatically"
    )
)]
#![doc = include_str!("../README.md")]

pub mod api;
/// re-export the reqwest crate so users of redmine-api can use the exact
/// dependency version we use for constructing clients
pub use reqwest;

use thiserror::Error;

/// Error type for redmine_api
#[derive(Debug, Error)]
pub enum Error {
    /// An error occurred in the reqwest library (HTTP)
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    /// An error occurred when serializing/deserializing JSON
    #[error("error in json serialization/deserialization: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    /// An error occurred when parsing a URL
    #[error("error when parsing URL: {0}")]
    UrlParseError(#[from] url::ParseError),
    /// An error occurred when reading configuration from environment variables
    #[error("error when reading environment variables: {0}")]
    EnvyError(#[from] envy::Error),
    /// Response body was empty so we can not deserialize it as JSON
    #[error("empty response body with status: {0}")]
    EmptyResponseBody(reqwest::StatusCode),
    /// Response body was valid JSON but not an object
    #[error("JSON but non-object response body with status: {0}")]
    NonObjectResponseBody(reqwest::StatusCode),
    /// Missing response pagination key (total_counts, offset, limit or the wrapper key)
    #[error("JSON wrapper pagination key missing: {0}")]
    PaginationKeyMissing(String),
    /// Response pagination key has the wrong type (total_counts, offset, limit)
    #[error("JSON wrapper pagination key has an unexpected type: {0}")]
    PaginationKeyHasWrongType(String),
    /// Parsing a time string to a time object (OffsetDateTime) failed
    #[error("Parsing string {0} to time object failed")]
    TimeParseError(String, time::error::Parse),
    /// Error reading a file we are supposed to upload
    #[error("Error when opening or reading file {0} to upload: {1}")]
    UploadFileError(std::path::PathBuf, std::io::Error),
    /// HTTP Error response
    #[error("HTTP Error response: {0}")]
    HttpErrorResponse(reqwest::StatusCode),
}
