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
use reqwest::Method;
use std::borrow::Cow;

use crate::api::custom_fields::CustomFieldEssentialsWithValue;
use crate::api::enumerations::TimeEntryActivityEssentials;
use crate::api::issues::{
    IssueEssentials, IssueStatusFilter, MemberOfGroupFilter, RoleFilter, UserFilter,
};
use crate::api::projects::{ProjectEssentials, ProjectFilter, ProjectStatusFilter};
use crate::api::users::UserEssentials;
use crate::api::{
    ActivityFilter, CustomFieldFilter, DateFilter, Endpoint, FloatFilter, NoPagination, Pageable,
    QueryParams, ReturnsJsonResponse, StringFieldFilter, TrackerFilter, VersionFilter,
};
use serde::Serialize;

/// a type for time entries to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct TimeEntry {
    /// numeric id
    pub id: u64,
    /// The user spending the time
    pub user: UserEssentials,
    /// the hours spent
    pub hours: f64,
    /// the activity
    pub activity: TimeEntryActivityEssentials,
    /// the comment
    #[serde(default)]
    pub comments: Option<String>,
    /// issue the time was spent on
    pub issue: Option<IssueEssentials>,
    /// project
    pub project: Option<ProjectEssentials>,
    /// day the time was spent on
    pub spent_on: Option<time::Date>,
    /// custom fields with values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldEssentialsWithValue>>,

    /// The time when this time entry was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// The time when this time entry was last updated
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub updated_on: time::OffsetDateTime,
}

/// The endpoint for all time entries
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListTimeEntries<'a> {
    /// user who spent the time
    #[builder(default)]
    author_id: Option<UserFilter>,
    /// project id or name as it appears in the URL on which the time was spent
    #[builder(setter(into), default)]
    project_id_or_name: Option<Cow<'a, str>>,
    /// Filter by time entry activity.
    #[builder(default)]
    activity_id: Option<ActivityFilter>,
    /// day the time was spent on
    #[builder(default)]
    spent_on: Option<DateFilter>,
    /// comments text search
    #[builder(setter(into), default)]
    comments: Option<StringFieldFilter>,
    /// hours spent
    #[builder(default)]
    hours: Option<FloatFilter>,
    /// Filter by issue tracker.
    #[builder(default)]
    tracker_id: Option<TrackerFilter>,
    /// issue status id
    #[builder(default)]
    status_id: Option<IssueStatusFilter>,
    /// Filter by the ID of the target version (milestone) to which the issue is assigned.
    #[builder(default)]
    fixed_version_id: Option<VersionFilter>,
    /// issue subject text search
    #[builder(setter(into), default)]
    subject: Option<StringFieldFilter>,
    /// user group id
    #[builder(default)]
    group_id: Option<MemberOfGroupFilter>,
    /// user role id
    #[builder(default)]
    role_id: Option<RoleFilter>,
    /// Filter by project status (e.g., active, closed). Uses the ProjectStatusFilter enum.
    #[builder(default)]
    project_status: Option<ProjectStatusFilter>,
    /// Filter by subproject.
    #[builder(default)]
    subproject_id: Option<ProjectFilter>,
    /// custom field filters
    #[builder(default)]
    custom_field_filters: Option<Vec<CustomFieldFilter>>,
}

impl ReturnsJsonResponse for ListTimeEntries<'_> {}
impl Pageable for ListTimeEntries<'_> {
    fn response_wrapper_key(&self) -> String {
        "time_entries".to_string()
    }
}

impl<'a> ListTimeEntries<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListTimeEntriesBuilder<'a> {
        ListTimeEntriesBuilder::default()
    }
}

impl Endpoint for ListTimeEntries<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "time_entries.json".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("user_id", self.author_id.as_ref().map(|f| f.to_string()));
        params.push_opt("project_id", self.project_id_or_name.as_ref());
        params.push_opt(
            "activity_id",
            self.activity_id.as_ref().map(|f| f.to_string()),
        );
        params.push_opt("spent_on", self.spent_on.as_ref().map(|f| f.to_string()));
        params.push_opt("comments", self.comments.as_ref().map(|f| f.to_string()));
        params.push_opt("hours", self.hours.as_ref().map(|f| f.to_string()));
        params.push_opt(
            "tracker_id",
            self.tracker_id.as_ref().map(|f| f.to_string()),
        );
        params.push_opt("status_id", self.status_id.as_ref().map(|f| f.to_string()));
        params.push_opt(
            "fixed_version_id",
            self.fixed_version_id.as_ref().map(|f| f.to_string()),
        );
        params.push_opt("subject", self.subject.as_ref().map(|f| f.to_string()));
        params.push_opt("group_id", self.group_id.as_ref().map(|f| f.to_string()));
        params.push_opt("role_id", self.role_id.as_ref().map(|f| f.to_string()));
        params.push_opt(
            "project_status",
            self.project_status.as_ref().map(|f| f.to_string()),
        );
        params.push_opt(
            "subproject_id",
            self.subproject_id.as_ref().map(|f| f.to_string()),
        );

        if let Some(custom_field_filters) = &self.custom_field_filters {
            for cf_filter in custom_field_filters {
                params.push(format!("cf_{}", cf_filter.id), cf_filter.value.to_string());
            }
        }
        params
    }
}

/// The endpoint for a specific time entry
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetTimeEntry {
    /// the id of the time entry to retrieve
    id: u64,
}

impl ReturnsJsonResponse for GetTimeEntry {}
impl NoPagination for GetTimeEntry {}

impl GetTimeEntry {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetTimeEntryBuilder {
        GetTimeEntryBuilder::default()
    }
}

impl Endpoint for GetTimeEntry {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("time_entries/{}.json", self.id).into()
    }
}

/// The endpoint to create a Redmine time entry
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
pub struct CreateTimeEntry<'a> {
    /// Issue id is required if project_id is not specified
    #[builder(default)]
    issue_id: Option<u64>,
    /// Project id is required if issue_id is not specified
    #[builder(default)]
    project_id: Option<u64>,
    /// The date the time was spent, default is today
    #[builder(default)]
    spent_on: Option<time::Date>,
    /// the hours spent
    hours: f64,
    /// This is required unless there is a default activity defined in Redmine
    #[builder(default)]
    activity_id: Option<u64>,
    /// Short description for the entry (255 characters max)
    #[builder(default)]
    comments: Option<Cow<'a, str>>,
    /// User Id is only required when posting time on behalf of another user, defaults to current user
    #[builder(default)]
    user_id: Option<u64>,
}

impl ReturnsJsonResponse for CreateTimeEntry<'_> {}
impl NoPagination for CreateTimeEntry<'_> {}

impl CreateTimeEntryBuilder<'_> {
    /// ensures that either issue_id or project_id is non-None when [Self::build()] is called
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
    #[must_use]
    pub fn builder() -> CreateTimeEntryBuilder<'a> {
        CreateTimeEntryBuilder::default()
    }
}

impl Endpoint for CreateTimeEntry<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "time_entries.json".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&TimeEntryWrapper::<CreateTimeEntry> {
                time_entry: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to update an existing Redmine time entry
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
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
    spent_on: Option<time::Date>,
    /// the hours spent
    #[builder(default)]
    hours: Option<f64>,
    /// This is required unless there is a default activity defined in Redmine
    #[builder(default)]
    activity_id: Option<u64>,
    /// Short description for the entry (255 characters max)
    #[builder(default)]
    comments: Option<Cow<'a, str>>,
    /// User Id is only required when posting time on behalf of another user, defaults to current user
    #[builder(default)]
    user_id: Option<u64>,
}

impl<'a> UpdateTimeEntry<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> UpdateTimeEntryBuilder<'a> {
        UpdateTimeEntryBuilder::default()
    }
}

impl Endpoint for UpdateTimeEntry<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("time_entries/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&TimeEntryWrapper::<UpdateTimeEntry> {
                time_entry: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to delete a Redmine time entry
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteTimeEntry {
    /// the id of the time entry to delete
    id: u64,
}

impl DeleteTimeEntry {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> DeleteTimeEntryBuilder {
        DeleteTimeEntryBuilder::default()
    }
}

impl Endpoint for DeleteTimeEntry {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("time_entries/{}.json", &self.id).into()
    }
}

/// helper struct for outer layers with a time_entries field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct TimeEntriesWrapper<T> {
    /// to parse JSON with time_entries key
    pub time_entries: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a time_entry field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct TimeEntryWrapper<T> {
    /// to parse JSON with time_entry key
    pub time_entry: T,
}

#[cfg(test)]
mod test {
    use crate::api::ResponsePage;
    use crate::api::test_helpers::with_redmine;
    use crate::api::test_locking;

    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tokio::sync::RwLock;
    use tracing_test::traced_test;

    /// needed so we do not get 404s when listing while
    /// creating/deleting or creating/updating/deleting
    pub static TIME_ENTRY_LOCK: RwLock<()> = RwLock::const_new(());

    #[traced_test]
    #[test]
    fn test_list_time_entries_first_page() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _r_time_entries = test_locking::read_lock(&TIME_ENTRY_LOCK);
            let endpoint = ListTimeEntries::builder().build()?;
            redmine.json_response_body_page::<_, TimeEntry>(&endpoint, 0, 25)?;
            Ok(())
        })
    }

    /// this takes a long time and is not very useful given the relative uniformity of time entries
    // #[traced_test]
    // #[test]
    // #[ignore]
    // fn test_list_time_entries_all_pages() -> Result<(), Box<dyn Error>> {
    //     let _r_time_entries = TIME_ENTRY_LOCK.blocking_read();
    //     dotenvy::dotenv()?;
    //     let redmine = crate::api::Redmine::from_env()?;
    //     let endpoint = ListTimeEntries::builder().build()?;
    //     redmine.json_response_body_all_pages::<_, TimeEntry>(&endpoint)?;
    //     Ok(())
    // }

    #[traced_test]
    #[test]
    fn test_get_time_entry() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _r_time_entries = test_locking::read_lock(&TIME_ENTRY_LOCK);
            let endpoint = GetTimeEntry::builder().id(832).build()?;
            redmine.json_response_body::<_, TimeEntryWrapper<TimeEntry>>(&endpoint)?;
            Ok(())
        })
    }

    #[traced_test]
    #[test]
    fn test_create_time_entry() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _w_time_entries = test_locking::write_lock(&TIME_ENTRY_LOCK);
            let create_endpoint = super::CreateTimeEntry::builder()
                .issue_id(25095)
                .hours(1.0)
                .activity_id(8)
                .build()?;
            redmine.json_response_body::<_, TimeEntryWrapper<TimeEntry>>(&create_endpoint)?;
            Ok(())
        })
    }

    #[traced_test]
    #[test]
    fn test_update_time_entry() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _w_time_entries = test_locking::write_lock(&TIME_ENTRY_LOCK);
            let create_endpoint = super::CreateTimeEntry::builder()
                .issue_id(25095)
                .hours(1.0)
                .activity_id(8)
                .build()?;
            let TimeEntryWrapper { time_entry } =
                redmine.json_response_body::<_, TimeEntryWrapper<TimeEntry>>(&create_endpoint)?;
            let update_endpoint = super::UpdateTimeEntry::builder()
                .id(time_entry.id)
                .hours(2.0)
                .build()?;
            redmine.ignore_response_body::<_>(&update_endpoint)?;
            Ok(())
        })
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_time_entry_type_first_page() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _r_time_entries = test_locking::read_lock(&TIME_ENTRY_LOCK);
            let endpoint = ListTimeEntries::builder().build()?;
            let ResponsePage {
                values,
                total_count: _,
                offset: _,
                limit: _,
            } = redmine.json_response_body_page::<_, serde_json::Value>(&endpoint, 0, 100)?;
            for value in values {
                let o: TimeEntry = serde_json::from_value(value.clone())?;
                let reserialized = serde_json::to_value(o)?;
                assert_eq!(value, reserialized);
            }
            Ok(())
        })
    }
}
