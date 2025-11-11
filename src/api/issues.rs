//! Issues Rest API Endpoint definitions
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Issues)
//!
//! [Redmine Documentation Journals](https://www.redmine.org/projects/redmine/wiki/Rest_IssueJournals)
//! (Journals in Redmine terminology are notes/comments and change histories for an issue)
//!
//! - [ ] all issues endpoint
//!   - [x] sort
//!     - [ ] limit sort to the existing columns only instead of a string value
//!   - [x] query_id parameter
//!   - [x] pagination
//!   - [x] issue_id filter
//!     - [x] issue id (multiple are possible, comma separated)
//!   - [x] project_id filter
//!     - [x] project id (multiple are possible, comma separated)
//!   - [x] subproject_id filter
//!     - [x] !* filter to only get parent project issues
//!   - [x] tracker_id filter
//!     - [x] tracker id (multiple are possible, comma separated)
//!   - [x] status_id filter
//!     - [x] open (default)
//!     - [x] closed
//!     - [x] for both
//!     - [x] status id (multiple are possible, comma separated)
//!   - [x] category_id filter
//!     - [x] category id (multiple are possible, comma separated)
//!   - [x] priority_id filter
//!     - [x] priority id (multiple are possible, comma separated)
//!   - [x] author_id filter
//!     - [x] any
//!     - [x] me
//!     - [x] !me
//!     - [x] user/group id (multiple are possible, comma separated)
//!     - [x] negation of list
//!   - [x] assigned_to_id filter
//!     - [x] any
//!     - [x] me
//!     - [x] !me
//!     - [x] user/group id (multiple are possible, comma separated)
//!     - [x] negation of list
//!     - [x] none (!*)
//!   - [x] fixed_version_id filter (Target version, API uses old name)
//!     - [x] version id (multiple are possible, comma separated)
//!   - [ ] is_private filter
//!   - [x] parent_id filter
//!     - [x] issue id (multiple are possible, comma separated)
//!   - [ ] custom field filter
//!     - [ ] exact match
//!     - [ ] substring match
//!     - [ ] what about multiple value custom fields?
//!   - [x] subject filter
//!     - [x] exact match
//!     - [x] substring match
//!   - [x] description filter
//!     - [x] exact match
//!     - [x] substring match
//!   - [ ] done_ratio filter
//!     - [ ] exact match
//!     - [ ] less than, greater than ?
//!     - [ ] range?
//!   - [ ] estimated_hours filter
//!     - [ ] exact match
//!     - [ ] less than, greater than ?
//!     - [ ] range?
//!   - [x] created_on filter
//!     - [x] exact match
//!     - [x] less than, greater than
//!     - [x] date range
//!   - [x] updated_on filter
//!     - [x] exact match
//!     - [x] less than, greater than
//!     - [x] date range
//!   - [x] start_date filter
//!     - [x] exact match
//!     - [x] less than, greater than
//!     - [x] date range
//!   - [x] due_date filter
//!     - [x] exact match
//!     - [x] less than, greater than
//!     - [x] date range
//! - [x] specific issue endpoint
//! - [x] create issue endpoint
//!   - [ ] attachments
//! - [x] update issue endpoint
//!   - [ ] attachments
//! - [x] delete issue endpoint
//! - [x] add watcher endpoint
//! - [x] remove watcher endpoint
//! - [ ] fields for issue changesets
//!
use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::attachments::Attachment;
use crate::api::custom_fields::CustomFieldEssentialsWithValue;
use crate::api::enumerations::IssuePriorityEssentials;
use crate::api::groups::{Group, GroupEssentials};
use crate::api::issue_categories::IssueCategoryEssentials;
use crate::api::issue_relations::IssueRelation;
use crate::api::issue_statuses::IssueStatusEssentials;
use crate::api::projects::ProjectEssentials;
use crate::api::trackers::TrackerEssentials;
use crate::api::users::UserEssentials;
use crate::api::versions::VersionEssentials;
use crate::api::{Endpoint, NoPagination, Pageable, QueryParams, ReturnsJsonResponse};
use serde::Serialize;

/// a minimal type for Redmine users or groups used in lists of assignees included in
/// other Redmine objects
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AssigneeEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

impl From<UserEssentials> for AssigneeEssentials {
    fn from(v: UserEssentials) -> Self {
        AssigneeEssentials {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&UserEssentials> for AssigneeEssentials {
    fn from(v: &UserEssentials) -> Self {
        AssigneeEssentials {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

impl From<GroupEssentials> for AssigneeEssentials {
    fn from(v: GroupEssentials) -> Self {
        AssigneeEssentials {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&GroupEssentials> for AssigneeEssentials {
    fn from(v: &GroupEssentials) -> Self {
        AssigneeEssentials {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

impl From<Group> for AssigneeEssentials {
    fn from(v: Group) -> Self {
        AssigneeEssentials {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&Group> for AssigneeEssentials {
    fn from(v: &Group) -> Self {
        AssigneeEssentials {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

/// a minimal type for Redmine issues included in
/// other Redmine objects
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IssueEssentials {
    /// numeric id
    pub id: u64,
}

impl From<Issue> for IssueEssentials {
    fn from(v: Issue) -> Self {
        IssueEssentials { id: v.id }
    }
}

impl From<&Issue> for IssueEssentials {
    fn from(v: &Issue) -> Self {
        IssueEssentials { id: v.id }
    }
}

/// the minimal data about a code repository included in other
/// redmine objects
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RepositoryEssentials {
    /// numeric id
    pub id: u64,
    /// the textual identifier
    pub identifier: String,
}

/// the type of issue changesets
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IssueChangeset {
    /// the revision of the changeset (e.g. commit id or number depending on VCS)
    revision: String,
    /// the committer
    user: UserEssentials,
    /// the commit message
    comments: String,
    /// the timestamp when this was committed
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    committed_on: time::OffsetDateTime,
}

/// the type of journal change
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ChangePropertyType {
    /// issue attribute change
    #[serde(rename = "attr")]
    Attr,
    /// Custom field change
    #[serde(rename = "cf")]
    Cf,
    /// change in issue relations
    #[serde(rename = "relation")]
    Relation,
    /// change in attachments
    #[serde(rename = "attachment")]
    Attachment,
}

/// a changed attribute entry in a journal entry
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JournalChange {
    /// name of the attribute
    pub name: String,
    /// old value
    pub old_value: Option<String>,
    /// new value
    pub new_value: Option<String>,
    /// what kind of property we are dealing with
    pub property: ChangePropertyType,
}

/// journals (issue comments and changes)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Journal {
    /// numeric id
    pub id: u64,
    /// the author of the journal entry
    pub user: UserEssentials,
    /// the comment content
    pub notes: Option<String>,
    /// is this a private comment
    pub private_notes: bool,
    /// The time when this comment/change was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// The time when this comment/change was last updated
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub updated_on: time::OffsetDateTime,
    /// The user who updated the comment/change if it differs from the author
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<UserEssentials>,
    /// changed issue attributes
    pub details: Vec<JournalChange>,
}

/// minimal issue used e.g. in child issues
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChildIssue {
    /// numeric id
    pub id: u64,
    /// subject
    pub subject: String,
    /// tracker
    pub tracker: TrackerEssentials,
    /// children
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ChildIssue>>,
}

/// a type for issue to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct Issue {
    /// numeric id
    pub id: u64,
    /// parent issue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<IssueEssentials>,
    /// the project of the issue
    pub project: ProjectEssentials,
    /// the tracker of the issue
    pub tracker: TrackerEssentials,
    /// the issue status
    pub status: IssueStatusEssentials,
    /// the issue priority
    pub priority: IssuePriorityEssentials,
    /// the issue author
    pub author: UserEssentials,
    /// the user or group the issue is assigned to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<AssigneeEssentials>,
    /// the issue category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<IssueCategoryEssentials>,
    /// the version the issue is assigned to
    #[serde(rename = "fixed_version", skip_serializing_if = "Option::is_none")]
    pub version: Option<VersionEssentials>,
    /// the issue subject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// the issue description
    pub description: Option<String>,
    /// is the issue private (only visible to roles that have the relevant permission enabled)
    is_private: Option<bool>,
    /// the start date for the issue
    pub start_date: Option<time::Date>,
    /// the due date for the issue
    pub due_date: Option<time::Date>,
    /// the time when the issue was closed
    #[serde(
        serialize_with = "crate::api::serialize_optional_rfc3339",
        deserialize_with = "crate::api::deserialize_optional_rfc3339"
    )]
    pub closed_on: Option<time::OffsetDateTime>,
    /// the percentage done
    pub done_ratio: u64,
    /// custom fields with values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldEssentialsWithValue>>,
    /// estimated hours it will take to implement this issue
    pub estimated_hours: Option<f64>,
    /// The time when this issue was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// The time when this issue was last updated
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub updated_on: time::OffsetDateTime,
    /// issue attachments (only when include parameter is used)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// issue relations (only when include parameter is used)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<IssueRelation>>,
    /// issue changesets (only when include parameter is used)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changesets: Option<Vec<IssueChangeset>>,
    /// journal entries (comments and changes), only when include parameter is used
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub journals: Option<Vec<Journal>>,
    /// child issues (only when include parameter is used)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ChildIssue>>,
    /// watchers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers: Option<Vec<UserEssentials>>,
    /// the hours spent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spent_hours: Option<f64>,
    /// the total hours spent on this and sub-tasks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_spent_hours: Option<f64>,
    /// the total hours estimated on this and sub-tasks
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_estimated_hours: Option<f64>,
}

/// ways to filter for subproject
#[derive(Debug, Clone)]
pub enum SubProjectFilter {
    /// return no issues from subjects
    OnlyParentProject,
    /// return issues from a specific list of sub project ids
    TheseSubProjects(Vec<u64>),
    /// return issues from any but a specific list of sub project ids
    NotTheseSubProjects(Vec<u64>),
}

impl std::fmt::Display for SubProjectFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubProjectFilter::OnlyParentProject => {
                write!(f, "!*")
            }
            SubProjectFilter::TheseSubProjects(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{s}")
            }
            SubProjectFilter::NotTheseSubProjects(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| format!("!{e}"))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{s}")
            }
        }
    }
}

/// ways to filter for issue status
#[derive(Debug, Clone)]
pub enum StatusFilter {
    /// match all open statuses (default if no status filter is specified
    Open,
    /// match all closed statuses
    Closed,
    /// match both open and closed statuses
    All,
    /// match a specific list of statuses
    TheseStatuses(Vec<u64>),
    /// match any status but a specific list of statuses
    NotTheseStatuses(Vec<u64>),
}

impl std::fmt::Display for StatusFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusFilter::Open => {
                write!(f, "open")
            }
            StatusFilter::Closed => {
                write!(f, "closed")
            }
            StatusFilter::All => {
                write!(f, "*")
            }
            StatusFilter::TheseStatuses(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{s}")
            }
            StatusFilter::NotTheseStatuses(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| format!("!{e}"))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{s}")
            }
        }
    }
}

/// ways to filter for users in author (always a user (not group), never !*)
#[derive(Debug, Clone)]
pub enum AuthorFilter {
    /// match any user
    AnyAuthor,
    /// match the current API user
    Me,
    /// match any author but the current API user
    NotMe,
    /// match a specific list of users
    TheseAuthors(Vec<u64>),
    /// match a negated specific list of users
    NotTheseAuthors(Vec<u64>),
}

impl std::fmt::Display for AuthorFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthorFilter::AnyAuthor => {
                write!(f, "*")
            }
            AuthorFilter::Me => {
                write!(f, "me")
            }
            AuthorFilter::NotMe => {
                write!(f, "!me")
            }
            AuthorFilter::TheseAuthors(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{s}")
            }
            AuthorFilter::NotTheseAuthors(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| format!("!{e}"))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{s}")
            }
        }
    }
}

/// ways to filter for users or groups in assignee
#[derive(Debug, Clone)]
pub enum AssigneeFilter {
    /// match any user or group
    AnyAssignee,
    /// match the current API user
    Me,
    /// match any assignee but the current API user
    NotMe,
    /// match a specific list of users or groups
    TheseAssignees(Vec<u64>),
    /// match a negated specific list of users or groups
    NotTheseAssignees(Vec<u64>),
    /// match unassigned
    NoAssignee,
}

impl std::fmt::Display for AssigneeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssigneeFilter::AnyAssignee => {
                write!(f, "*")
            }
            AssigneeFilter::Me => {
                write!(f, "me")
            }
            AssigneeFilter::NotMe => {
                write!(f, "!me")
            }
            AssigneeFilter::TheseAssignees(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{s}")
            }
            AssigneeFilter::NotTheseAssignees(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| format!("!{e}"))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{s}")
            }
            AssigneeFilter::NoAssignee => {
                write!(f, "!*")
            }
        }
    }
}

/// Filter options for subject and description
#[derive(Debug, Clone)]
pub enum StringFieldFilter {
    /// match exactly this value
    ExactMatch(String),
    /// match this substring of the actual value
    SubStringMatch(String),
}

impl std::fmt::Display for StringFieldFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringFieldFilter::ExactMatch(s) => {
                write!(f, "{s}")
            }
            StringFieldFilter::SubStringMatch(s) => {
                write!(f, "~{s}")
            }
        }
    }
}

/// Filter for a comparable date time filters for past
/// used for filters on created_on, updated_on fields
#[derive(Debug, Clone)]
pub enum DateTimeFilterPast {
    /// an exact match
    ExactMatch(time::OffsetDateTime),
    /// a range match (inclusive)
    Range(time::OffsetDateTime, time::OffsetDateTime),
    /// we only want values less than or equal to the parameter
    LessThanOrEqual(time::OffsetDateTime),
    /// we only want values greater than or equal to the parameter
    GreaterThanOrEqual(time::OffsetDateTime),
    /// less than n days ago
    LessThanDaysAgo(u32),
    /// more than n days ago
    MoreThanDaysAgo(u32),
    /// within the past n days
    WithinPastDays(u32),
    /// exactly n days ago
    ExactDaysAgo(u32),
    /// today
    Today,
    /// yesterday
    Yesterday,
    /// this week
    ThisWeek,
    /// last week
    LastWeek,
    /// last 2 weeks
    LastTwoWeeks,
    /// this month
    ThisMonth,
    /// last month
    LastMonth,
    /// this year
    ThisYear,
    /// unset value (NULL in DB)
    Unset,
    /// any value (NOT NULL in DB)
    Any,
}

impl std::fmt::Display for DateTimeFilterPast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format =
            time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");
        match self {
            DateTimeFilterPast::ExactMatch(v) => {
                write!(
                    f,
                    "{}",
                    v.format(&format).expect(
                        "Error formatting OffsetDateTime in DateTimeFilterPast::ExactMatch"
                    )
                )
            }
            DateTimeFilterPast::Range(v_start, v_end) => {
                write!(
                    f,
                    "><{}|{}",
                    v_start.format(&format).expect(
                        "Error formatting first OffsetDateTime in DateTimeFilterPast::Range"
                    ),
                    v_end.format(&format).expect(
                        "Error formatting second OffsetDateTime in DateTimeFilterPast::Range"
                    ),
                )
            }
            DateTimeFilterPast::LessThanOrEqual(v) => {
                write!(
                    f,
                    "<={}",
                    v.format(&format).expect(
                        "Error formatting OffsetDateTime in DateTimeFilterPast::LessThanOrEqual"
                    )
                )
            }
            DateTimeFilterPast::GreaterThanOrEqual(v) => {
                write!(
                    f,
                    ">={}",
                    v.format(&format).expect(
                        "Error formatting OffsetDateTime in DateTimeFilterPast::GreaterThanOrEqual"
                    )
                )
            }
            DateTimeFilterPast::LessThanDaysAgo(d) => {
                write!(f, ">t-{}", d)
            }
            DateTimeFilterPast::MoreThanDaysAgo(d) => {
                write!(f, "<t-{}", d)
            }
            DateTimeFilterPast::WithinPastDays(d) => {
                write!(f, "><t-{}", d)
            }
            DateTimeFilterPast::ExactDaysAgo(d) => {
                write!(f, "t-{}", d)
            }
            DateTimeFilterPast::Today => {
                write!(f, "t")
            }
            DateTimeFilterPast::Yesterday => {
                write!(f, "ld")
            }
            DateTimeFilterPast::ThisWeek => {
                write!(f, "w")
            }
            DateTimeFilterPast::LastWeek => {
                write!(f, "lw")
            }
            DateTimeFilterPast::LastTwoWeeks => {
                write!(f, "l2w")
            }
            DateTimeFilterPast::ThisMonth => {
                write!(f, "m")
            }
            DateTimeFilterPast::LastMonth => {
                write!(f, "lm")
            }
            DateTimeFilterPast::ThisYear => {
                write!(f, "y")
            }
            DateTimeFilterPast::Unset => {
                write!(f, "!*")
            }
            DateTimeFilterPast::Any => {
                write!(f, "*")
            }
        }
    }
}
/// Filter for a comparable date filters for past or
/// future dates used for filters on start_date and due_date
#[derive(Debug, Clone)]
pub enum DateFilter {
    /// an exact match
    ExactMatch(time::Date),
    /// a range match (inclusive)
    Range(time::Date, time::Date),
    /// we only want values less than or equal to the parameter
    LessThanOrEqual(time::Date),
    /// we only want values greater than or equal to the parameter
    GreaterThanOrEqual(time::Date),
    /// less than n days ago
    LessThanDaysAgo(u32),
    /// more than n days ago
    MoreThanDaysAgo(u32),
    /// within the past n days
    WithinPastDays(u32),
    /// exactly n days ago
    ExactDaysAgo(u32),
    /// in less than n days
    InLessThanDays(u32),
    /// in more than n days
    InMoreThanDays(u32),
    /// in the next n days
    WithinFutureDays(u32),
    /// in exactly n days
    InExactDays(u32),
    /// today
    Today,
    /// yesterday
    Yesterday,
    /// tomorrow
    Tomorrow,
    /// this week
    ThisWeek,
    /// last week
    LastWeek,
    /// last 2 weeks
    LastTwoWeeks,
    /// next week
    NextWeek,
    /// this month
    ThisMonth,
    /// last month
    LastMonth,
    /// next month
    NextMonth,
    /// this year
    ThisYear,
    /// unset value (NULL in DB)
    Unset,
    /// any value (NOT NULL in DB)
    Any,
}

impl std::fmt::Display for DateFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let format = time::macros::format_description!("[year]-[month]-[day]");
        match self {
            DateFilter::ExactMatch(v) => {
                write!(
                    f,
                    "{}",
                    v.format(&format)
                        .expect("Error formatting Date in DateFilter::ExactMatch")
                )
            }
            DateFilter::Range(v_start, v_end) => {
                write!(
                    f,
                    "><{}|{}",
                    v_start
                        .format(&format)
                        .expect("Error formatting first Date in DateFilter::Range"),
                    v_end
                        .format(&format)
                        .expect("Error formatting second Date in DateFilter::Range"),
                )
            }
            DateFilter::LessThanOrEqual(v) => {
                write!(
                    f,
                    "<={}",
                    v.format(&format)
                        .expect("Error formatting Date in DateFilter::LessThanOrEqual")
                )
            }
            DateFilter::GreaterThanOrEqual(v) => {
                write!(
                    f,
                    ">={}",
                    v.format(&format)
                        .expect("Error formatting Date in DateFilter::GreaterThanOrEqual")
                )
            }
            DateFilter::LessThanDaysAgo(d) => {
                write!(f, ">t-{}", d)
            }
            DateFilter::MoreThanDaysAgo(d) => {
                write!(f, "<t-{}", d)
            }
            DateFilter::WithinPastDays(d) => {
                write!(f, "><t-{}", d)
            }
            DateFilter::ExactDaysAgo(d) => {
                write!(f, "t-{}", d)
            }
            DateFilter::InLessThanDays(d) => {
                write!(f, "<t+{}", d)
            }
            DateFilter::InMoreThanDays(d) => {
                write!(f, ">t+{}", d)
            }
            DateFilter::WithinFutureDays(d) => {
                write!(f, "><t+{}", d)
            }
            DateFilter::InExactDays(d) => {
                write!(f, "t+{}", d)
            }
            DateFilter::Today => {
                write!(f, "t")
            }
            DateFilter::Yesterday => {
                write!(f, "ld")
            }
            DateFilter::Tomorrow => {
                write!(f, "nd")
            }
            DateFilter::ThisWeek => {
                write!(f, "w")
            }
            DateFilter::LastWeek => {
                write!(f, "lw")
            }
            DateFilter::LastTwoWeeks => {
                write!(f, "l2w")
            }
            DateFilter::NextWeek => {
                write!(f, "nw")
            }
            DateFilter::ThisMonth => {
                write!(f, "m")
            }
            DateFilter::LastMonth => {
                write!(f, "lm")
            }
            DateFilter::NextMonth => {
                write!(f, "nm")
            }
            DateFilter::ThisYear => {
                write!(f, "y")
            }
            DateFilter::Unset => {
                write!(f, "!*")
            }
            DateFilter::Any => {
                write!(f, "*")
            }
        }
    }
}

/// Sort by this column
#[derive(Debug, Clone)]
pub enum SortByColumn {
    /// Sort in an ascending direction
    Forward {
        /// the column to sort by
        column_name: String,
    },
    /// Sort in a descending direction
    Reverse {
        /// the column to sort by
        column_name: String,
    },
}

impl std::fmt::Display for SortByColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortByColumn::Forward { column_name } => {
                write!(f, "{column_name}")
            }
            SortByColumn::Reverse { column_name } => {
                write!(f, "{column_name}:desc")
            }
        }
    }
}

/// The types of associated data which can be fetched along with a issue
#[derive(Debug, Clone)]
pub enum IssueListInclude {
    /// Issue Attachments
    Attachments,
    /// Issue relations
    Relations,
}

impl std::fmt::Display for IssueListInclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attachments => {
                write!(f, "attachments")
            }
            Self::Relations => {
                write!(f, "relations")
            }
        }
    }
}

/// The endpoint for all Redmine issues
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListIssues {
    /// Include associated data
    #[builder(default)]
    include: Option<Vec<IssueListInclude>>,
    /// Sort by column
    #[builder(default)]
    sort: Option<Vec<SortByColumn>>,
    /// Filter by issue id(s)
    #[builder(default)]
    issue_id: Option<Vec<u64>>,
    /// Filter by project id
    #[builder(default)]
    project_id: Option<Vec<u64>>,
    /// Filter by subproject
    #[builder(default)]
    subproject_id: Option<SubProjectFilter>,
    /// Filter by tracker id
    #[builder(default)]
    tracker_id: Option<Vec<u64>>,
    /// Filter by priority id
    #[builder(default)]
    priority_id: Option<Vec<u64>>,
    /// Filter by parent issue id
    #[builder(default)]
    parent_id: Option<Vec<u64>>,
    /// Filter by issue category id
    #[builder(default)]
    category_id: Option<Vec<u64>>,
    /// Filter by issue status
    #[builder(default)]
    status_id: Option<StatusFilter>,
    /// Filter by subject
    #[builder(default)]
    subject: Option<StringFieldFilter>,
    /// Filter by description
    #[builder(default)]
    description: Option<StringFieldFilter>,
    /// Filter by author
    #[builder(default)]
    author: Option<AuthorFilter>,
    /// Filter by assignee
    #[builder(default)]
    assignee: Option<AssigneeFilter>,
    /// Filter by a saved query
    #[builder(default)]
    query_id: Option<u64>,
    /// Filter by target version
    #[builder(default)]
    version_id: Option<Vec<u64>>,
    /// Filter by creation time
    #[builder(default)]
    created_on: Option<DateTimeFilterPast>,
    /// Filter by update time
    #[builder(default)]
    updated_on: Option<DateTimeFilterPast>,
    /// Filter by start date
    #[builder(default)]
    start_date: Option<DateFilter>,
    /// Filter by due date
    #[builder(default)]
    due_date: Option<DateFilter>,
    /// Filter by child issue id
    #[builder(default)]
    child_id: Option<Vec<u64>>,
}

impl ReturnsJsonResponse for ListIssues {}

impl Pageable for ListIssues {
    fn response_wrapper_key(&self) -> String {
        "issues".to_string()
    }
}

impl ListIssues {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListIssuesBuilder {
        ListIssuesBuilder::default()
    }
}

impl Endpoint for ListIssues {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "issues.json".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("include", self.include.as_ref());
        params.push_opt("sort", self.sort.as_ref());
        params.push_opt("issue_id", self.issue_id.as_ref());
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt(
            "subproject_id",
            self.subproject_id.as_ref().map(|s| s.to_string()),
        );
        params.push_opt("tracker_id", self.tracker_id.as_ref());
        params.push_opt("priority_id", self.priority_id.as_ref());
        params.push_opt("parent_id", self.parent_id.as_ref());
        params.push_opt("category_id", self.category_id.as_ref());
        params.push_opt("status_id", self.status_id.as_ref().map(|s| s.to_string()));
        params.push_opt("subject", self.subject.as_ref().map(|s| s.to_string()));
        params.push_opt(
            "description",
            self.description.as_ref().map(|s| s.to_string()),
        );
        params.push_opt("author_id", self.author.as_ref().map(|s| s.to_string()));
        params.push_opt(
            "assigned_to_id",
            self.assignee.as_ref().map(|s| s.to_string()),
        );
        params.push_opt("query_id", self.query_id);
        params.push_opt("fixed_version_id", self.version_id.as_ref());
        params.push_opt(
            "created_on",
            self.created_on.as_ref().map(|s| s.to_string()),
        );
        params.push_opt(
            "updated_on",
            self.updated_on.as_ref().map(|s| s.to_string()),
        );
        params.push_opt(
            "start_date",
            self.start_date.as_ref().map(|s| s.to_string()),
        );
        params.push_opt("due_date", self.due_date.as_ref().map(|s| s.to_string()));
        params.push_opt("child_id", self.child_id.as_ref());
        params
    }
}

/// The types of associated data which can be fetched along with a issue
#[derive(Debug, Clone)]
pub enum IssueInclude {
    /// Child issues
    Children,
    /// Issue attachments
    Attachments,
    /// Issue relations
    Relations,
    /// VCS changesets
    Changesets,
    /// the notes and changes to the issue
    Journals,
    /// Users watching the issue
    Watchers,
    /// The statuses this issue can transition to
    ///
    /// This can be influenced by
    ///
    /// - the defined workflow
    ///   - the issue's current tracker
    ///   - the issue's current status
    ///   - the member's role
    /// - the existence of any open subtask(s)
    /// - the existence of any open blocking issue(s)
    /// - the existence of a closed parent issue
    ///
    AllowedStatuses,
}

impl std::fmt::Display for IssueInclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Children => {
                write!(f, "children")
            }
            Self::Attachments => {
                write!(f, "attachments")
            }
            Self::Relations => {
                write!(f, "relations")
            }
            Self::Changesets => {
                write!(f, "changesets")
            }
            Self::Journals => {
                write!(f, "journals")
            }
            Self::Watchers => {
                write!(f, "watchers")
            }
            Self::AllowedStatuses => {
                write!(f, "allowed_statuses")
            }
        }
    }
}

/// The endpoint for a specific Redmine issue
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetIssue {
    /// id of the issue to retrieve
    id: u64,
    /// associated data to include
    #[builder(default)]
    include: Option<Vec<IssueInclude>>,
}

impl ReturnsJsonResponse for GetIssue {}
impl NoPagination for GetIssue {}

impl GetIssue {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetIssueBuilder {
        GetIssueBuilder::default()
    }
}

impl Endpoint for GetIssue {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}.json", &self.id).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("include", self.include.as_ref());
        params
    }
}

/// a custom field
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct CustomField<'a> {
    /// the custom field's id
    pub id: u64,
    /// is usually present in contexts where it is returned by Redmine but can be omitted when it is sent by the client
    pub name: Option<Cow<'a, str>>,
    /// the custom field's value
    pub value: Cow<'a, str>,
}

/// the information the uploader needs to supply for an attachment
/// in [CreateIssue] or [UpdateIssue]
#[derive(Debug, Clone, Serialize)]
pub struct UploadedAttachment<'a> {
    /// the upload token from [UploadFile|crate::api::uploads::UploadFile]
    pub token: Cow<'a, str>,
    /// the filename
    pub filename: Cow<'a, str>,
    /// a description for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    /// the MIME content type of the file
    pub content_type: Cow<'a, str>,
}

/// The endpoint to create a Redmine issue
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateIssue<'a> {
    /// project for the issue
    project_id: u64,
    /// tracker for the issue
    #[builder(default)]
    tracker_id: Option<u64>,
    /// status of the issue
    #[builder(default)]
    status_id: Option<u64>,
    /// issue priority
    #[builder(default)]
    priority_id: Option<u64>,
    /// issue subject
    #[builder(setter(into), default)]
    subject: Option<Cow<'a, str>>,
    /// issue description
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// issue category
    #[builder(default)]
    category_id: Option<u64>,
    /// ID of the Target Versions (previously called 'Fixed Version' and still referred to as such in the API)
    #[builder(default, setter(name = "version"))]
    fixed_version_id: Option<u64>,
    /// user/group id the issue will be assigned to
    #[builder(default)]
    assigned_to_id: Option<u64>,
    /// Id of the parent issue
    #[builder(default)]
    parent_issue_id: Option<u64>,
    /// custom field values
    #[builder(default)]
    custom_fields: Option<Vec<CustomField<'a>>>,
    /// user ids of watchers of the issue
    #[builder(default)]
    watcher_user_ids: Option<Vec<u64>>,
    /// is the issue private (only visible to roles that have the relevant permission enabled)
    #[builder(default)]
    is_private: Option<bool>,
    /// estimated hours it will take to implement this issue
    #[builder(default)]
    estimated_hours: Option<f64>,
    /// attachments (files)
    #[builder(default)]
    uploads: Option<Vec<UploadedAttachment<'a>>>,
}

impl<'a> CreateIssue<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> CreateIssueBuilder<'a> {
        CreateIssueBuilder::default()
    }
}

impl ReturnsJsonResponse for CreateIssue<'_> {}
impl NoPagination for CreateIssue<'_> {}

impl Endpoint for CreateIssue<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "issues.json".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&IssueWrapper::<CreateIssue> {
                issue: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to update an existing Redmine issue
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateIssue<'a> {
    /// id of the issue to update
    #[serde(skip_serializing)]
    id: u64,
    /// project for the issue
    #[builder(default)]
    project_id: Option<u64>,
    /// tracker for the issue
    #[builder(default)]
    tracker_id: Option<u64>,
    /// status of the issue
    #[builder(default)]
    status_id: Option<u64>,
    /// issue priority
    #[builder(default)]
    priority_id: Option<u64>,
    /// issue subject
    #[builder(setter(into), default)]
    subject: Option<Cow<'a, str>>,
    /// issue description
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// issue category
    #[builder(default)]
    category_id: Option<u64>,
    /// ID of the Target Versions (previously called 'Fixed Version' and still referred to as such in the API)
    #[builder(default, setter(name = "version"))]
    fixed_version_id: Option<u64>,
    /// user/group id the issue will be assigned to
    #[builder(default)]
    assigned_to_id: Option<u64>,
    /// Id of the parent issue
    #[builder(default)]
    parent_issue_id: Option<u64>,
    /// custom field values
    #[builder(default)]
    custom_fields: Option<Vec<CustomField<'a>>>,
    /// user ids of watchers of the issue
    #[builder(default)]
    watcher_user_ids: Option<Vec<u64>>,
    /// is the issue private (only visible to roles that have the relevant permission enabled)
    #[builder(default)]
    is_private: Option<bool>,
    /// estimated hours it will take to implement this issue
    #[builder(default)]
    estimated_hours: Option<f64>,
    /// add a comment (note)
    #[builder(default)]
    notes: Option<Cow<'a, str>>,
    /// is the added comment/note private
    #[builder(default)]
    private_notes: Option<bool>,
    /// attachments (files)
    #[builder(default)]
    uploads: Option<Vec<UploadedAttachment<'a>>>,
}

impl<'a> UpdateIssue<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> UpdateIssueBuilder<'a> {
        UpdateIssueBuilder::default()
    }
}

impl Endpoint for UpdateIssue<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&IssueWrapper::<UpdateIssue> {
                issue: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to delete a Redmine issue
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteIssue {
    /// id of the issue to delete
    id: u64,
}

impl DeleteIssue {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> DeleteIssueBuilder {
        DeleteIssueBuilder::default()
    }
}

impl Endpoint for DeleteIssue {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}.json", &self.id).into()
    }
}

/// The endpoint to add a Redmine user as a watcher on a Redmine issue
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct AddWatcher {
    /// id of the issue to add the watcher to
    #[serde(skip_serializing)]
    issue_id: u64,
    /// id of the user to add as a watcher
    user_id: u64,
}

impl AddWatcher {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> AddWatcherBuilder {
        AddWatcherBuilder::default()
    }
}

impl Endpoint for AddWatcher {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}/watchers.json", &self.issue_id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to remove a Redmine user from a Redmine issue as a watcher
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct RemoveWatcher {
    /// id of the issue to remove the watcher from
    issue_id: u64,
    /// id of the user to remove as a watcher
    user_id: u64,
}

impl RemoveWatcher {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> RemoveWatcherBuilder {
        RemoveWatcherBuilder::default()
    }
}

impl Endpoint for RemoveWatcher {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}/watchers/{}.json", &self.issue_id, &self.user_id).into()
    }
}

/// helper struct for outer layers with a issues field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct IssuesWrapper<T> {
    /// to parse JSON with issues key
    pub issues: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a issue field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct IssueWrapper<T> {
    /// to parse JSON with an issue key
    pub issue: T,
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::api::ResponsePage;
    use crate::api::test_helpers::with_project;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tokio::sync::RwLock;
    use tracing_test::traced_test;

    /// needed so we do not get 404s when listing while
    /// creating/deleting or creating/updating/deleting
    pub static ISSUES_LOCK: RwLock<()> = RwLock::const_new(());

    #[traced_test]
    #[test]
    fn test_list_issues_first_page() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;
        let endpoint = ListIssues::builder().build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;
        Ok(())
    }

    /// this version of the test will load all pages of issues which means it
    /// can take a while (a minute or more) so you need to use --include-ignored
    /// or --ignored to run it
    #[traced_test]
    #[test]
    #[ignore]
    fn test_list_issues_all_pages() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;
        let endpoint = ListIssues::builder().build()?;
        redmine.json_response_body_all_pages::<_, Issue>(&endpoint)?;
        Ok(())
    }

    /// this version of the test will load all pages of issues which means it
    /// can take a while (a minute or more) so you need to use --include-ignored
    /// or --ignored to run it
    #[traced_test]
    #[test]
    #[ignore]
    fn test_list_issues_all_pages_iter() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;
        let endpoint = ListIssues::builder().build()?;
        let mut i = 0;
        for issue in redmine.json_response_body_all_pages_iter::<_, Issue>(&endpoint) {
            let _issue = issue?;
            i += 1;
        }
        assert!(i > 0);

        Ok(())
    }

    /// this version of the test will load all pages of issues which means it
    /// can take a while (a minute or more) so you need to use --include-ignored
    /// or --ignored to run it
    #[traced_test]
    #[tokio::test]
    #[ignore]
    async fn test_list_issues_all_pages_stream() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.read().await;
        dotenvy::dotenv()?;
        let redmine = crate::api::RedmineAsync::from_env(
            reqwest::Client::builder().use_rustls_tls().build()?,
        )?;
        let endpoint = ListIssues::builder().build()?;
        let mut i = 0;
        let mut stream = redmine.json_response_body_all_pages_stream::<_, Issue>(&endpoint);
        while let Some(issue) = <_ as futures::stream::StreamExt>::next(&mut stream).await {
            let _issue = issue?;
            i += 1;
        }
        assert!(i > 0);

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_issue() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;
        let endpoint = GetIssue::builder().id(40000).build()?;
        redmine.json_response_body::<_, IssueWrapper<Issue>>(&endpoint)?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_issue() -> Result<(), Box<dyn Error>> {
        let _w_issues = ISSUES_LOCK.blocking_write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, project_id, _| {
            let create_endpoint = super::CreateIssue::builder()
                .project_id(project_id)
                .subject("old test subject")
                .build()?;
            redmine.json_response_body::<_, IssueWrapper<Issue>>(&create_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_update_issue() -> Result<(), Box<dyn Error>> {
        let _w_issues = ISSUES_LOCK.blocking_write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, project_id, _name| {
            let create_endpoint = super::CreateIssue::builder()
                .project_id(project_id)
                .subject("old test subject")
                .build()?;
            let IssueWrapper { issue }: IssueWrapper<Issue> =
                redmine.json_response_body::<_, _>(&create_endpoint)?;
            let update_endpoint = super::UpdateIssue::builder()
                .id(issue.id)
                .subject("New test subject")
                .build()?;
            redmine.ignore_response_body::<_>(&update_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_delete_issue() -> Result<(), Box<dyn Error>> {
        let _w_issues = ISSUES_LOCK.blocking_write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, project_id, _name| {
            let create_endpoint = super::CreateIssue::builder()
                .project_id(project_id)
                .subject("test subject")
                .build()?;
            let IssueWrapper { issue }: IssueWrapper<Issue> =
                redmine.json_response_body::<_, _>(&create_endpoint)?;
            let delete_endpoint = super::DeleteIssue::builder().id(issue.id).build()?;
            redmine.ignore_response_body::<_>(&delete_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_issue_type_first_page() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;
        let endpoint = ListIssues::builder()
            .include(vec![
                IssueListInclude::Attachments,
                IssueListInclude::Relations,
            ])
            .build()?;
        let ResponsePage {
            values,
            total_count: _,
            offset: _,
            limit: _,
        } = redmine.json_response_body_page::<_, serde_json::Value>(&endpoint, 0, 100)?;
        for value in values {
            let o: Issue = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            let expected_value = if let serde_json::Value::Object(obj) = value {
                let mut expected_obj = obj.clone();
                if obj
                    .get("total_estimated_hours")
                    .is_some_and(|v| *v == serde_json::Value::Null)
                {
                    expected_obj.remove("total_estimated_hours");
                }
                serde_json::Value::Object(expected_obj)
            } else {
                value
            };
            assert_eq!(expected_value, reserialized);
        }
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    ///
    /// this version of the test will load all pages of issues which means it
    /// can take a while (a minute or more) so you need to use --include-ignored
    /// or --ignored to run it
    #[traced_test]
    #[test]
    #[ignore]
    fn test_completeness_issue_type_all_pages() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;
        let endpoint = ListIssues::builder()
            .include(vec![
                IssueListInclude::Attachments,
                IssueListInclude::Relations,
            ])
            .build()?;
        let values = redmine.json_response_body_all_pages::<_, serde_json::Value>(&endpoint)?;
        for value in values {
            let o: Issue = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            let expected_value = if let serde_json::Value::Object(obj) = value {
                let mut expected_obj = obj.clone();
                if obj
                    .get("total_estimated_hours")
                    .is_some_and(|v| *v == serde_json::Value::Null)
                {
                    expected_obj.remove("total_estimated_hours");
                }
                serde_json::Value::Object(expected_obj)
            } else {
                value
            };
            assert_eq!(expected_value, reserialized);
        }
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    ///
    /// this version of the test will load all pages of issues and the individual
    /// issues for each via GetIssue which means it
    /// can take a while (about 400 seconds) so you need to use --include-ignored
    /// or --ignored to run it
    #[traced_test]
    #[test]
    #[ignore]
    fn test_completeness_issue_type_all_pages_all_issue_details() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;
        let endpoint = ListIssues::builder()
            .include(vec![
                IssueListInclude::Attachments,
                IssueListInclude::Relations,
            ])
            .build()?;
        let issues = redmine.json_response_body_all_pages::<_, Issue>(&endpoint)?;
        for issue in issues {
            let get_endpoint = GetIssue::builder()
                .id(issue.id)
                .include(vec![
                    IssueInclude::Attachments,
                    IssueInclude::Children,
                    IssueInclude::Changesets,
                    IssueInclude::Relations,
                    IssueInclude::Journals,
                    IssueInclude::Watchers,
                ])
                .build()?;
            let IssueWrapper { issue: mut value } =
                redmine.json_response_body::<_, IssueWrapper<serde_json::Value>>(&get_endpoint)?;
            let o: Issue = serde_json::from_value(value.clone())?;
            // workaround for the fact that the field total_estimated_hours is put into the result
            // when its null in the GetIssue endpoint but not in the ListIssues one
            // we can only do one or the other in our JSON serialization unless we want to add
            // extra fields just to keep track of the missing field vs. field with null value
            // difference
            let value_object = value.as_object_mut().unwrap();
            if value_object.get("total_estimated_hours") == Some(&serde_json::Value::Null) {
                value_object.remove("total_estimated_hours");
            }
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_exact_match() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::datetime!(2023-01-15 10:30:00 UTC);
        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::ExactMatch(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_range() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt_start = time::macros::datetime!(2023-01-01 00:00:00 UTC);
        let dt_end = time::macros::datetime!(2023-01-31 23:59:59 UTC);
        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::Range(dt_start, dt_end))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_less_than_or_equal() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::datetime!(2023-01-15 10:30:00 UTC);
        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::LessThanOrEqual(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_greater_than_or_equal() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::datetime!(2023-01-15 10:30:00 UTC);
        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::GreaterThanOrEqual(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_less_than_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::LessThanDaysAgo(5))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_more_than_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::MoreThanDaysAgo(10))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_within_past_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::WithinPastDays(7))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_exact_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::ExactDaysAgo(3))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_today() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::Today)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_yesterday() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::Yesterday)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_this_week() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::ThisWeek)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_last_week() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::LastWeek)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_last_two_weeks() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::LastTwoWeeks)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_this_month() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::ThisMonth)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_last_month() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::LastMonth)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_this_year() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::ThisYear)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_unset() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::Unset)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_created_on_filter_any() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .created_on(DateTimeFilterPast::Any)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_exact_match() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::datetime!(2023-01-15 10:30:00 UTC);
        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::ExactMatch(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_range() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt_start = time::macros::datetime!(2023-01-01 00:00:00 UTC);
        let dt_end = time::macros::datetime!(2023-01-31 23:59:59 UTC);
        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::Range(dt_start, dt_end))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_less_than_or_equal() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::datetime!(2023-01-15 10:30:00 UTC);
        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::LessThanOrEqual(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_greater_than_or_equal() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::datetime!(2023-01-15 10:30:00 UTC);
        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::GreaterThanOrEqual(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_less_than_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::LessThanDaysAgo(5))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_more_than_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::MoreThanDaysAgo(10))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_within_past_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::WithinPastDays(7))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_exact_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::ExactDaysAgo(3))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_today() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::Today)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_yesterday() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::Yesterday)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_this_week() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::ThisWeek)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_last_week() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::LastWeek)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_last_two_weeks() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::LastTwoWeeks)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_this_month() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::ThisMonth)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_last_month() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::LastMonth)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_this_year() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::ThisYear)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_unset() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::Unset)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_updated_on_filter_any() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .updated_on(DateTimeFilterPast::Any)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_exact_match() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::date!(2023 - 01 - 15);
        let endpoint = ListIssues::builder()
            .start_date(DateFilter::ExactMatch(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_range() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt_start = time::macros::date!(2023 - 01 - 01);
        let dt_end = time::macros::date!(2023 - 01 - 31);
        let endpoint = ListIssues::builder()
            .start_date(DateFilter::Range(dt_start, dt_end))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_less_than_or_equal() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::date!(2023 - 01 - 15);
        let endpoint = ListIssues::builder()
            .start_date(DateFilter::LessThanOrEqual(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_greater_than_or_equal() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::date!(2023 - 01 - 15);
        let endpoint = ListIssues::builder()
            .start_date(DateFilter::GreaterThanOrEqual(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_less_than_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::LessThanDaysAgo(5))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_more_than_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::MoreThanDaysAgo(10))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_within_past_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::WithinPastDays(7))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_exact_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::ExactDaysAgo(3))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_in_less_than_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::InLessThanDays(5))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_in_more_than_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::InMoreThanDays(10))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_within_future_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::WithinFutureDays(7))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_in_exact_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::InExactDays(3))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_today() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::Today)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_yesterday() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::Yesterday)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_tomorrow() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::Tomorrow)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_this_week() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::ThisWeek)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_last_week() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::LastWeek)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_last_two_weeks() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::LastTwoWeeks)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_next_week() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::NextWeek)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_this_month() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::ThisMonth)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_last_month() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::LastMonth)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_next_month() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::NextMonth)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_this_year() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::ThisYear)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_unset() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .start_date(DateFilter::Unset)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_start_date_filter_any() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder().start_date(DateFilter::Any).build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_exact_match() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::date!(2023 - 01 - 15);
        let endpoint = ListIssues::builder()
            .due_date(DateFilter::ExactMatch(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_range() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt_start = time::macros::date!(2023 - 01 - 01);
        let dt_end = time::macros::date!(2023 - 01 - 31);
        let endpoint = ListIssues::builder()
            .due_date(DateFilter::Range(dt_start, dt_end))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_less_than_or_equal() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::date!(2023 - 01 - 15);
        let endpoint = ListIssues::builder()
            .due_date(DateFilter::LessThanOrEqual(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_greater_than_or_equal() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let dt = time::macros::date!(2023 - 01 - 15);
        let endpoint = ListIssues::builder()
            .due_date(DateFilter::GreaterThanOrEqual(dt))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_less_than_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::LessThanDaysAgo(5))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_more_than_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::MoreThanDaysAgo(10))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_within_past_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::WithinPastDays(7))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_exact_days_ago() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::ExactDaysAgo(3))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_in_less_than_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::InLessThanDays(5))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_in_more_than_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::InMoreThanDays(10))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_within_future_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::WithinFutureDays(7))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_in_exact_days() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::InExactDays(3))
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_today() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder().due_date(DateFilter::Today).build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_yesterday() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::Yesterday)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_tomorrow() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::Tomorrow)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_this_week() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::ThisWeek)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_last_week() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::LastWeek)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_last_two_weeks() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::LastTwoWeeks)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_next_week() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::NextWeek)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_this_month() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::ThisMonth)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_last_month() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::LastMonth)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_next_month() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::NextMonth)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_this_year() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder()
            .due_date(DateFilter::ThisYear)
            .build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_unset() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder().due_date(DateFilter::Unset).build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_due_date_filter_any() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder().due_date(DateFilter::Any).build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_child_id_filter() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;

        let endpoint = ListIssues::builder().child_id(vec![123, 456]).build()?;
        redmine.json_response_body_page::<_, Issue>(&endpoint, 0, 25)?;

        Ok(())
    }
}
