//! Time Entries Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_TimeEntries)
//!
//! - [x] all time entries endpoint
//!   - [x] user_id filter
//!   - [x] project_id filter
//!   - [x] issue_id filter
//!   - [x] activity_id filter
//!   - [x] spent_on filter (date)
//!   - [x] from filter
//!   - [x] to filter
//! - [x] specific time entry endpoint
//! - [x] create time entry endpoint
//! - [x] update time entry endpoint
//! - [x] delete time entry endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use serde::Serialize;
use crate::api::{Endpoint,QueryParams};

/// The endpoint for all time entries
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct TimeEntries<'a> {
    /// user who spent the time
    #[builder(default)]
    user_id: Option<u64>,
    /// project id or name as it appears in the URL on which the time was spent
    #[builder(setter(into), default)]
    project_id_or_name: Option<Cow<'a, str>>,
    /// issue on which the time was spent
    #[builder(default)]
    issue_id: Option<u64>,
    /// activity for the spent time
    #[builder(default)]
    activity_id: Option<u64>,
    /// day the time was spent on
    #[builder(default)]
    spent_on: Option<chrono::NaiveDate>,
    /// from day filter for spent on
    #[builder(default)]
    from: Option<chrono::NaiveDate>,
    /// to day filter for spent on
    #[builder(default)]
    to: Option<chrono::NaiveDate>,
}

impl<'a> TimeEntries<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> TimeEntriesBuilder<'a> {
        TimeEntriesBuilder::default()
    }
}

impl<'a> Endpoint for TimeEntries<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "time_entries.json".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("user_id", self.user_id);
        params.push_opt("project_id", self.project_id_or_name.as_ref());
        params.push_opt("issue_id", self.issue_id);
        params.push_opt("activity_id", self.activity_id);
        params.push_opt("spent_on", self.spent_on);
        params.push_opt("from", self.from);
        params.push_opt("to", self.to);
        params
    }
}

/// The endpoint for a specific time entry
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct TimeEntry {
    /// the id of the time entry to retrieve
    id: u64,
}

impl<'a> TimeEntry {
    /// Create a builder for the endpoint.
    pub fn builder() -> TimeEntryBuilder {
        TimeEntryBuilder::default()
    }
}

impl<'a> Endpoint for TimeEntry {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("time_entries/{}.json", self.id).into()
    }
}

/// The endpoint to create a Redmine time entry
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option),build_fn(validate = "Self::validate"))]
pub struct CreateTimeEntry<'a> {
    /// Issue id is required if project_id is not specified
    #[builder(default)]
    issue_id: Option<u64>,
    /// Project id is required if issue_id is not specified
    #[builder(default)]
    project_id: Option<u64>,
    /// The date the time was spent, default is today
    #[builder(default)]
    spent_on: Option<chrono::NaiveDate>,
    /// the hours spent
    #[builder(default)]
    hours: Option<f64>,
    /// This is required unless there is a default activity defined in Redmine
    #[builder(default)]
    activity_id: Option<u64>,
    /// Short description for teh entry (255 characters max)
    #[builder(default)]
    comments: Option<Cow<'a,str>>,
    /// User Id is only required when posting time on behalf of another user, defaults to current user
    #[builder(default)]
    user_id: Option<u64>,
}

impl<'a> CreateTimeEntryBuilder<'a> {
    fn validate(&self) -> Result<(), String> {
        if self.issue_id.is_none() && self.project_id.is_none() {
            Err("Either issue_id or project_id need to be specified".to_string())
        } else {
            Ok(())
        }
    }
}

impl<'a> CreateTimeEntry<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateTimeEntryBuilder<'a> {
        CreateTimeEntryBuilder::default()
    }
}

impl<'a> Endpoint for CreateTimeEntry<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "time_entries.json".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to update an existing Redmine time entry
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option),build_fn(validate = "Self::validate"))]
pub struct UpdateTimeEntry<'a> {
    /// the id of the time entry to update
    #[serde(skip_serializing)]
    id: u64,
    /// Issue id is required if project_id is not specified
    #[builder(default)]
    issue_id: Option<u64>,
    /// Project id is required if issue_id is not specified
    #[builder(default)]
    project_id: Option<u64>,
    /// The date the time was spent, default is today
    #[builder(default)]
    spent_on: Option<chrono::NaiveDate>,
    /// the hours spent
    #[builder(default)]
    hours: Option<f64>,
    /// This is required unless there is a default activity defined in Redmine
    #[builder(default)]
    activity_id: Option<u64>,
    /// Short description for teh entry (255 characters max)
    #[builder(default)]
    comments: Option<Cow<'a,str>>,
    /// User Id is only required when posting time on behalf of another user, defaults to current user
    #[builder(default)]
    user_id: Option<u64>,
}

impl<'a> UpdateTimeEntryBuilder<'a> {
    fn validate(&self) -> Result<(), String> {
        if self.issue_id.is_none() && self.project_id.is_none() {
            Err("Either issue_id or project_id need to be specified".to_string())
        } else {
            Ok(())
        }
    }
}

impl<'a> UpdateTimeEntry<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UpdateTimeEntryBuilder<'a> {
        UpdateTimeEntryBuilder::default()
    }
}

impl<'a> Endpoint for UpdateTimeEntry<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("time_entries/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to delete a Redmine time entry
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteTimeEntry {
    /// the id of the time entry to delete
    id: u64,
}

impl DeleteTimeEntry {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteTimeEntryBuilder {
        DeleteTimeEntryBuilder::default()
    }
}

impl<'a> Endpoint for DeleteTimeEntry {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
       format!("time_entries/{}.json", &self.id).into()
    }
}
