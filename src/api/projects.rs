//! Projects Rest API Endpoint definitions
//!
//! [`Redmine Documentation`](https://www.redmine.org/projects/redmine/wiki/Rest_Projects)
//!
//! - [x] all projects endpoint
//! - [x] specific project endpoint
//! - [x] create project endpoint
//! - [x] update project endpoint
//! - [x] archive project endpoint
//! - [x] unarchive project endpoint
//! - [x] delete project endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::{Endpoint, Pageable, QueryParams, ReturnsJsonResponse};
use serde::Serialize;
use std::collections::HashMap;

/// a minimal type for Redmine projects used in lists of projects included in
/// other Redmine objects (e.g. custom fields)
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProjectEssentials {
    /// numeric id
    id: u64,
    /// display name
    name: String,
    /// URL slug
    identifier: String,
}

/// a type for projects to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct Project {
    /// numeric id
    id: u64,
    /// display name
    name: String,
    /// URL slug
    identifier: String,
    /// description
    description: Option<String>,
    /// the project homepage
    homepage: Option<String>,
    /// is the project public (visible to anonymous users)
    is_public: Option<bool>,
    /// the parent project id
    parent_id: Option<u64>,
    /// will the project inherit members from its ancestors
    inherit_members: Option<bool>,
    /// ID of the default user. It works only when the new project is a subproject and it inherits the members
    default_assigned_to_id: Option<u64>,
    /// ID of the default version. It works only with existing shared versions
    default_version_id: Option<u64>,
    /// trackers to enable in the project
    tracker_ids: Option<Vec<u64>>,
    /// modules to enable in the project
    enabled_module_names: Option<Vec<String>>,
    /// custom issue fields to enable in the project
    issue_custom_field_id: Option<Vec<u64>>,
    /// values for custom fields
    custom_field_values: Option<HashMap<u64, String>>,
}

/// The types of associated data which can be fetched along with a project
#[derive(Debug, Clone)]
pub enum ListProjectsInclude {
    /// Trackers enabled in the project
    Trackers,
    /// Issue categories in the project
    IssueCategories,
    /// Redmine Modules enabled in the project
    EnabledModules,
}

impl std::fmt::Display for ListProjectsInclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trackers => {
                write!(f, "trackers")
            }
            Self::IssueCategories => {
                write!(f, "issue_categories")
            }
            Self::EnabledModules => {
                write!(f, "enabled_modules")
            }
        }
    }
}

/// The endpoint for all Redmine projects
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ListProjects {
    /// the types of associate data to include
    #[builder(default)]
    include: Option<Vec<ListProjectsInclude>>,
}

impl ReturnsJsonResponse for ListProjects {}
impl Pageable for ListProjects {
    fn response_wrapper_key(&self) -> String {
        "projects".to_string()
    }
}

impl ListProjects {
    /// Create a builder for the endpoint.
    pub fn builder() -> ListProjectsBuilder {
        ListProjectsBuilder::default()
    }
}

impl<'a> Endpoint for ListProjects {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "projects.json".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("includes", self.include.as_ref());
        params
    }
}

/// The types of associated data which can be fetched along with a project
#[derive(Debug, Clone)]
pub enum GetProjectInclude {
    /// Trackers enabled in the project
    Trackers,
    /// Issue categories in the project
    IssueCategories,
    /// Redmine Modules enabled in the project
    EnabledModules,
    /// Time Entry Activities enabled in the project
    TimeEntryActivities,
}

impl std::fmt::Display for GetProjectInclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trackers => {
                write!(f, "trackers")
            }
            Self::IssueCategories => {
                write!(f, "issue_categories")
            }
            Self::EnabledModules => {
                write!(f, "enabled_modules")
            }
            Self::TimeEntryActivities => {
                write!(f, "time_entry_activities")
            }
        }
    }
}

/// The endpoint for a specific Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct GetProject<'a> {
    /// the project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// the types of associate data to include
    #[builder(default)]
    include: Option<Vec<GetProjectInclude>>,
}

impl<'a> ReturnsJsonResponse for GetProject<'a> {}

impl<'a> GetProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> GetProjectBuilder<'a> {
        GetProjectBuilder::default()
    }
}

impl<'a> Endpoint for GetProject<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}.json", &self.project_id_or_name).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("includes", self.include.as_ref());
        params
    }
}

/// The endpoint to archive a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ArchiveProject<'a> {
    /// the project id or name as it appears in the URL of the project to archive
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl<'a> ArchiveProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ArchiveProjectBuilder<'a> {
        ArchiveProjectBuilder::default()
    }
}

impl<'a> Endpoint for ArchiveProject<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/archive.json", &self.project_id_or_name).into()
    }
}

/// The endpoint to unarchive a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct UnarchiveProject<'a> {
    /// the project id or name as it appears in the URL of the project to unarchive
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl<'a> UnarchiveProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UnarchiveProjectBuilder<'a> {
        UnarchiveProjectBuilder::default()
    }
}

impl<'a> Endpoint for UnarchiveProject<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/unarchive.json", &self.project_id_or_name).into()
    }
}

/// The endpoint to create a Redmine project
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateProject<'a> {
    /// the name of the project
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// the identifier of the project as it appears in the URL
    #[builder(setter(into))]
    identifier: Cow<'a, str>,
    /// the project description
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// the project homepage
    #[builder(setter(into), default)]
    homepage: Option<Cow<'a, str>>,
    /// is the project public (visible to anonymous users)
    #[builder(default)]
    is_public: Option<bool>,
    /// the parent project id
    #[builder(default)]
    parent_id: Option<u64>,
    /// will the project inherit members from its ancestors
    #[builder(default)]
    inherit_members: Option<bool>,
    /// ID of the default user. It works only when the new project is a subproject and it inherits the members
    #[builder(default)]
    default_assigned_to_id: Option<u64>,
    /// ID of the default version. It works only with existing shared versions
    #[builder(default)]
    default_version_id: Option<u64>,
    /// trackers to enable in the project
    #[builder(default)]
    tracker_ids: Option<Vec<u64>>,
    /// modules to enable in the project
    #[builder(default)]
    enabled_module_names: Option<Vec<Cow<'a, str>>>,
    /// custom issue fields to enable in the project
    #[builder(default)]
    issue_custom_field_id: Option<Vec<u64>>,
    /// values for custom fields
    #[builder(default)]
    custom_field_values: Option<HashMap<u64, Cow<'a, str>>>,
}

impl<'a> ReturnsJsonResponse for CreateProject<'a> {}

impl<'a> CreateProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateProjectBuilder<'a> {
        CreateProjectBuilder::default()
    }
}

impl<'a> Endpoint for CreateProject<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "projects.json".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&ProjectWrapper::<CreateProject> {
                project: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to update an existing Redmine project
#[serde_with::skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateProject<'a> {
    /// the project id or name as it appears in the URL of the project to update
    #[serde(skip_serializing)]
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// the name of the project
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    /// the identifier of the project as it appears in the URL
    #[builder(setter(into), default)]
    identifier: Option<Cow<'a, str>>,
    /// the project description
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
    /// the project homepage
    #[builder(setter(into), default)]
    homepage: Option<Cow<'a, str>>,
    /// is the project public (visible to anonymous users)
    #[builder(default)]
    is_public: Option<bool>,
    /// the parent project id
    #[builder(default)]
    parent_id: Option<u64>,
    /// will the project inherit members from its ancestors
    #[builder(default)]
    inherit_members: Option<bool>,
    /// ID of the default user. It works only when the new project is a subproject and it inherits the members
    #[builder(default)]
    default_assigned_to_id: Option<u64>,
    /// ID of the default version. It works only with existing shared versions
    #[builder(default)]
    default_version_id: Option<u64>,
    /// trackers to enable in the project
    #[builder(default)]
    tracker_ids: Option<Vec<u64>>,
    /// modules to enable in the project
    #[builder(default)]
    enabled_module_names: Option<Vec<Cow<'a, str>>>,
    /// custom issue fields to enable in the project
    #[builder(default)]
    issue_custom_field_id: Option<Vec<u64>>,
    /// values for custom fields
    #[builder(default)]
    custom_field_values: Option<HashMap<u64, Cow<'a, str>>>,
}

impl<'a> UpdateProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UpdateProjectBuilder<'a> {
        UpdateProjectBuilder::default()
    }
}

impl<'a> Endpoint for UpdateProject<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}.json", self.project_id_or_name).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to delete a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteProject<'a> {
    /// the project id or name as it appears in the URL of the project to delete
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl<'a> DeleteProject<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteProjectBuilder<'a> {
        DeleteProjectBuilder::default()
    }
}

impl<'a> Endpoint for DeleteProject<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}.json", &self.project_id_or_name).into()
    }
}

/// helper struct for outer layers with a projects field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct ProjectsWrapper<T> {
    /// to parse JSON with projects key
    projects: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a project field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct ProjectWrapper<T> {
    /// to parse JSON with project key
    project: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;
    //use pretty_assertions::{assert_eq,assert_ne};
    use crate::api::test_helpers::with_project;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_list_projects_no_pagination() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjects::builder().build()?;
        redmine.json_response_body::<_, ProjectsWrapper<Project>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_projects_first_page() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjects::builder().build()?;
        redmine.json_response_body_page::<_, Project>(&endpoint, 0, 25)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_projects_all_pages() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjects::builder().build()?;
        redmine.json_response_body_all_pages::<_, Project>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_project() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = GetProject::builder()
            .project_id_or_name("sandbox")
            .build()?;
        redmine.json_response_body::<_, ProjectWrapper<Project>>(&endpoint)?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_project() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |_, _| Ok(()))?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_update_project() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, name| {
            let update_endpoint = super::UpdateProject::builder()
                .project_id_or_name(name)
                .description("Test-Description")
                .build()?;
            redmine.ignore_response_body::<_>(&update_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }
}
