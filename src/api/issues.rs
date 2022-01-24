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
//!   - [ ] pagination
//!   - [ ] issue_id filter
//!     - [x] issue id (multiple are possible, comma separated)
//!   - [ ] project_id filter
//!     - [x] project id (multiple are possible, comma separated)
//!   - [ ] subproject_id filter
//!     - [ ] !* filter to only get parent project issues
//!   - [ ] tracker_id filter
//!     - [x] tracker id (multiple are possible, comma separated)
//!   - [ ] status_id filter
//!     - [ ] open (default)
//!     - [ ] closed
//!     - [ ] * for both
//!     - [x] status id (multiple are possible, comma separated)
//!   - [ ] category_id filter
//!     - [x] category id (multiple are possible, comma separated)
//!   - [ ] priority_id filter
//!     - [ ] less than, greater than
//!   - [ ] author_id filter
//!     - [ ] me
//!     - [ ] user/group id (multiple are possible, comma separated)
//!     - [ ] negation
//!   - [ ] assigned_to_id filter
//!     - [ ] me
//!     - [ ] user/group id (multiple are possible, comma separated)
//!     - [ ] negation
//!   - [ ] fixed_version_id filter (Target version, API uses old name)
//!     - [ ] version id (multiple are possible, comma separated)
//!   - [ ] is_private filter
//!   - [ ] parent_id filter
//!     - [ ] issue id (multiple are possible, comma separated)
//!   - [ ] custom field filter
//!     - [ ] exact match
//!     - [ ] substring match
//!   - [ ] subject filter
//!     - [ ] exact match
//!     - [ ] substring match
//!   - [ ] description filter
//!     - [ ] exact match
//!     - [ ] substring match
//!   - [ ] done_ratio filter
//!     - [ ] exact match
//!     - [ ] less than, greater than
//!   - [ ] estimated_hours filter
//!     - [ ] exact match
//!     - [ ] less than, greater than
//!   - [ ] created_on filter
//!     - [ ] exact match
//!     - [ ] less than, greater than
//!   - [ ] updated_on filter
//!     - [ ] exact match
//!     - [ ] less than, greater than
//!   - [ ] start_date filter
//!     - [ ] exact match
//!     - [ ] less than, greater than
//!   - [ ] due_date filter
//!     - [ ] exact match
//!     - [ ] less than, greater than
//! - [x] specific issue endpoint
//! - [x] create issue endpoint
//! - [x] update issue endpoint
//! - [x] delete issue endpoint
//! - [x] add watcher endpoint
//! - [x] remove watcher endpoint
//!
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::{Endpoint, Pageable, QueryParams};
use serde::Serialize;

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
                write!(f, "{}:rev", column_name)
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
pub struct Issues {
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
    /// Filter by tracker id
    #[builder(default)]
    tracker_id: Option<Vec<u64>>,
    /// Filter by issue category id
    #[builder(default)]
    category_id: Option<Vec<u64>>,
    /// Filter by issue status
    #[builder(default)]
    status_id: Option<Vec<u64>>,
}

impl Pageable for Issues {
    fn response_wrapper_key(&self) -> String {
        "issues".to_string()
    }
}

impl<'a> Issues {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssuesBuilder {
        IssuesBuilder::default()
    }
}

impl<'a> Endpoint for Issues {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "issues.json".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("includes", self.include.as_ref());
        params.push_opt("sort", self.sort.as_ref());
        params.push_opt("issue_id", self.issue_id.as_ref());
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt("tracker_id", self.tracker_id.as_ref());
        params.push_opt("category_id", self.category_id.as_ref());
        params.push_opt("status_id", self.status_id.as_ref());
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
pub struct Issue {
    /// id of the issue to retrieve
    id: u64,
    /// associated data to include
    #[builder(default)]
    include: Option<Vec<IssueInclude>>,
}

impl<'a> Issue {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueBuilder {
        IssueBuilder::default()
    }
}

impl Endpoint for Issue {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}.json", &self.id).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("includes", self.include.as_ref());
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

/// The endpoint to create a Redmine issue
#[derive(Debug, Builder, Serialize)]
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
}

impl<'a> CreateIssue<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateIssueBuilder<'a> {
        CreateIssueBuilder::default()
    }
}

impl<'a> Endpoint for CreateIssue<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "issues.json".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to update an existing Redmine issue
#[derive(Debug, Builder, Serialize)]
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
}

impl<'a> UpdateIssue<'a> {
    /// Create a builder for the endpoint.
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
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
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
