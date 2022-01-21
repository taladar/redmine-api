//! Issue Statuses Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_IssueStatuses)
//!
//! - [x] all issue statuses endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;

/// The endpoint for all issue statuses
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct IssueStatuses {}

impl IssueStatuses {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueStatusesBuilder {
        IssueStatusesBuilder::default()
    }
}

impl<'a> Endpoint for IssueStatuses {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "issue_statuses.json".into()
    }
}
