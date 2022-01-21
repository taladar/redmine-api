//! News Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_News)
//!
//! - [x] all news endpoint
//! - [x] project news endpoint
//!
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;

/// The endpoint for all news
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct News {}

impl News {
    /// Create a builder for the endpoint.
    pub fn builder() -> NewsBuilder {
        NewsBuilder::default()
    }
}

impl<'a> Endpoint for News {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "news.json".into()
    }
}

/// The endpoint for project news
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ProjectNews<'a> {
    /// project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl<'a> ProjectNews<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectNewsBuilder<'a> {
        ProjectNewsBuilder::default()
    }
}

impl<'a> Endpoint for ProjectNews<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/news.json", self.project_id_or_name).into()
    }
}
