//! Enumerations Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Enumerations)
//!
//! - [x] all issue priorities endpoint
//! - [x] all time entry activities endpoint
//! - [x] all document categories endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;

/// The endpoint for all issue priorities
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct IssuePriorities {}

impl IssuePriorities {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssuePrioritiesBuilder {
        IssuePrioritiesBuilder::default()
    }
}

impl<'a> Endpoint for IssuePriorities {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "enumerations/issue_priorities.json".into()
    }
}

/// The endpoint for all time entry activities
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct TimeEntryActivities {}

impl TimeEntryActivities {
    /// Create a builder for the endpoint.
    pub fn builder() -> TimeEntryActivitiesBuilder {
        TimeEntryActivitiesBuilder::default()
    }
}

impl<'a> Endpoint for TimeEntryActivities {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "enumerations/time_entry_activities.json".into()
    }
}

/// The endpoint for all document categories
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DocumentCategories {}

impl DocumentCategories {
    /// Create a builder for the endpoint.
    pub fn builder() -> DocumentCategoriesBuilder {
        DocumentCategoriesBuilder::default()
    }
}

impl<'a> Endpoint for DocumentCategories {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "trackers.json".into()
    }
}
