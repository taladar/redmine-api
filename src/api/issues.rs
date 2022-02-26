//! Issues Rest API Endpoint definitions
//!
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
//!     - [x] * for both
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
//!
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::attachments::Attachment;
use crate::api::custom_fields::CustomFieldEssentialsWithValue;
use crate::api::enumerations::IssuePriorityEssentials;
use crate::api::issue_categories::IssueCategoryEssentials;
use crate::api::issue_relations::IssueRelation;
use crate::api::issue_statuses::IssueStatusEssentials;
use crate::api::projects::ProjectEssentials;
use crate::api::trackers::TrackerEssentials;
use crate::api::users::UserEssentials;
use crate::api::versions::VersionEssentials;
use crate::api::{Endpoint, Pageable, QueryParams, ReturnsJsonResponse};
use serde::Serialize;

/// a minimal type for Redmine users or groups used in lists of assignees included in
/// other Redmine objects
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AssigneeEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

/// a minimal type for Redmine issues included in
/// other Redmine objects
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IssueEssentials {
    /// numeric id
    pub id: u64,
}

/// the type of journal change
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ChangePropertyType {
    /// issue attribute change
    #[serde(rename = "attr")]
    Attr,
    /// TODO: not quite sure what cf stands for
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
#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
    /// changed issue attributes
    pub details: Vec<JournalChange>,
}

/// minimal issue used e.g. in child issues
#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Serialize, serde::Deserialize)]
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
    pub custom_fields: Vec<CustomFieldEssentialsWithValue>,
    /// estimated hours it will take to implement this isssue
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
                write!(f, "{}", s)
            }
            SubProjectFilter::NotTheseSubProjects(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| format!("!{}", e))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{}", s)
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
                write!(f, "{}", s)
            }
            StatusFilter::NotTheseStatuses(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| format!("!{}", e))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{}", s)
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
                write!(f, "{}", s)
            }
            AuthorFilter::NotTheseAuthors(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| format!("!{}", e))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{}", s)
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
                write!(f, "{}", s)
            }
            AssigneeFilter::NotTheseAssignees(ids) => {
                let s: String = ids
                    .iter()
                    .map(|e| format!("!{}", e))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{}", s)
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
                write!(f, "{}", s)
            }
            StringFieldFilter::SubStringMatch(s) => {
                write!(f, "~{}", s)
            }
        }
    }
}

/// a trait for comparable filter values, we do not just use Display because
/// one of our main application is dates and we need a specific format
pub trait ComparableFilterValue {
    /// returns a string representation of a single value (e.g. a date)
    /// to be combined with the comparison operators by the Display trait of
    /// [ComparableFilter]
    fn value_string(&self) -> Cow<'static, str>;
}

impl ComparableFilterValue for time::Date {
    fn value_string(&self) -> Cow<'static, str> {
        let format = time::format_description::parse("[year]-[month]-[day]").unwrap();
        self.format(&format).unwrap().into()
    }
}

impl ComparableFilterValue for time::OffsetDateTime {
    fn value_string(&self) -> Cow<'static, str> {
        self.format(&time::format_description::well_known::Rfc3339)
            .unwrap()
            .into()
    }
}

/// Filter for a comparable filter (those you can use ranges, less, greater,...) on
#[derive(Debug, Clone)]
pub enum ComparableFilter<V> {
    /// an exact match
    ExactMatch(V),
    /// a range match
    Range(V, V),
    /// we only want values less than the parameter
    LessThan(V),
    /// we only want values less than or equal to the parameter
    LessThanOrEqual(V),
    /// we only want values greater than the parameter
    GreaterThan(V),
    /// we only want values greater than or equal to the parameter
    GreaterThanOrEqual(V),
}

impl<V> std::fmt::Display for ComparableFilter<V>
where
    V: ComparableFilterValue,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparableFilter::ExactMatch(v) => {
                write!(f, "{}", v.value_string())
            }
            ComparableFilter::Range(v_start, v_end) => {
                write!(f, "><{}|{}", v_start.value_string(), v_end.value_string())
            }
            ComparableFilter::LessThan(v) => {
                write!(f, "<{}", v.value_string())
            }
            ComparableFilter::LessThanOrEqual(v) => {
                write!(f, "<={}", v.value_string())
            }
            ComparableFilter::GreaterThan(v) => {
                write!(f, ">{}", v.value_string())
            }
            ComparableFilter::GreaterThanOrEqual(v) => {
                write!(f, ">={}", v.value_string())
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
                write!(f, "{}", column_name)
            }
            SortByColumn::Reverse { column_name } => {
                write!(f, "{}:desc", column_name)
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
#[derive(Debug, Builder)]
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
    created_on: Option<ComparableFilter<time::OffsetDateTime>>,
    /// Filter by update time
    #[builder(default)]
    updated_on: Option<ComparableFilter<time::OffsetDateTime>>,
    /// Filter by start date
    #[builder(default)]
    start_date: Option<ComparableFilter<time::Date>>,
    /// Filter by due date
    #[builder(default)]
    due_date: Option<ComparableFilter<time::Date>>,
}

impl<'a> ReturnsJsonResponse for ListIssues {}

impl<'a> Pageable for ListIssues {
    fn response_wrapper_key(&self) -> String {
        "issues".to_string()
    }
}

impl<'a> ListIssues {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListIssuesBuilder {
        ListIssuesBuilder::default()
    }
}

impl<'a> Endpoint for ListIssues {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "issues.json".into()
    }

    fn parameters(&self) -> QueryParams {
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
                write!(f, "relations")
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
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct GetIssue {
    /// id of the issue to retrieve
    id: u64,
    /// associated data to include
    #[builder(default)]
    include: Option<Vec<IssueInclude>>,
}

impl<'a> ReturnsJsonResponse for GetIssue {}

impl<'a> GetIssue {
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

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("include", self.include.as_ref());
        params
    }
}

/// a custom field
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct CustomField<'a> {
    /// the custom field's id
    id: u64,
    /// is usually present in contexts where it is returned by Redmine but can be ommitted when it is sent by the client
    name: Option<Cow<'a, str>>,
    /// the custom field's value
    value: Cow<'a, str>,
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
    /// estimated hours it will take to implement this isssue
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

impl<'a> ReturnsJsonResponse for CreateIssue<'a> {}

impl<'a> Endpoint for CreateIssue<'a> {
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
    /// estimated hours it will take to implement this isssue
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

impl<'a> Endpoint for UpdateIssue<'a> {
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
#[derive(Debug, Builder)]
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

impl<'a> Endpoint for DeleteIssue {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}.json", &self.id).into()
    }
}

/// The endpoint to add a Redmine user as a watcher on a Redmine issue
#[derive(Debug, Builder, Serialize)]
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

impl<'a> Endpoint for AddWatcher {
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
#[derive(Debug, Builder)]
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

impl<'a> Endpoint for RemoveWatcher {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}/watchers/{}.json", &self.issue_id, &self.user_id).into()
    }
}

/// helper struct for outer layers with a issues field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct IssuesWrapper<T> {
    /// to parse JSON with issues key
    pub issues: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a issue field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct IssueWrapper<T> {
    /// to parse JSON with an issue key
    pub issue: T,
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::api::test_helpers::with_project;
    use parking_lot::{const_rwlock, RwLock};
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    /// needed so we do not get 404s when listing while
    /// creating/deleting or creating/updating/deleting
    pub static ISSUES_LOCK: RwLock<()> = const_rwlock(());

    #[traced_test]
    #[test]
    fn test_list_issues_no_pagination() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.read();
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListIssues::builder().build()?;
        redmine.json_response_body::<_, IssuesWrapper<Issue>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_issues_first_page() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.read();
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
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
        let _r_issues = ISSUES_LOCK.read();
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListIssues::builder().build()?;
        redmine.json_response_body_all_pages::<_, Issue>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_issue() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.read();
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = GetIssue::builder().id(40000).build()?;
        redmine.json_response_body::<_, IssueWrapper<Issue>>(&endpoint)?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_issue() -> Result<(), Box<dyn Error>> {
        let _w_issues = ISSUES_LOCK.write();
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
        let _w_issues = ISSUES_LOCK.write();
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
        let _w_issues = ISSUES_LOCK.write();
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
    fn test_completeness_issue_type() -> Result<(), Box<dyn Error>> {
        let _r_issues = ISSUES_LOCK.read();
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListIssues::builder()
            .include(vec![
                IssueListInclude::Attachments,
                IssueListInclude::Relations,
            ])
            .build()?;
        let IssuesWrapper { issues: values } =
            redmine.json_response_body::<_, IssuesWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: Issue = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
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
        let _r_issues = ISSUES_LOCK.read();
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
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
            assert_eq!(value, reserialized);
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
        let _r_issues = ISSUES_LOCK.read();
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
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
}
