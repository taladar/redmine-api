#![deny(unknown_lints)]
#![deny(renamed_and_removed_lints)]
#![forbid(unsafe_code)]
#![deny(deprecated)]
#![forbid(private_in_public)]
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
#![warn(missing_debug_implementations)]
#![doc = include_str!("../README.md")]

pub mod api;

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
}
