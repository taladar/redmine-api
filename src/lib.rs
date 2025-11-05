#![deny(unknown_lints)]
#![deny(renamed_and_removed_lints)]
#![forbid(unsafe_code)]
#![deny(deprecated)]
#![forbid(private_interfaces)]
#![forbid(private_bounds)]
#![forbid(non_fmt_panics)]
#![deny(unreachable_code)]
#![deny(unreachable_patterns)]
#![forbid(unused_doc_comments)]
#![forbid(unused_must_use)]
#![deny(while_true)]
#![deny(unused_parens)]
#![deny(redundant_semicolons)]
#![deny(non_ascii_idents)]
#![deny(confusable_idents)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::cargo_common_metadata)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_debug_implementations)]
#![deny(clippy::mod_module_files)]
//#![warn(clippy::pedantic)]
#![warn(clippy::redundant_else)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_errors_doc)]
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
