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

use crate::api::enumerations::TimeEntryActivityEssentials;
use crate::api::issue_categories::IssueCategoryEssentials;
use crate::api::issues::AssigneeEssentials;
use crate::api::trackers::TrackerEssentials;
use crate::api::versions::VersionEssentials;
use crate::api::{Endpoint, Pageable, QueryParams, ReturnsJsonResponse};
use serde::Serialize;
use std::collections::HashMap;

/// a minimal type for Redmine modules used in lists enabled modules
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Module {
    /// numeric id
    pub id: u64,
    /// name (all lower-case and with underscores so probably not meant for display purposes)
    pub name: String,
}

/// a minimal type for Redmine projects used in lists of projects included in
/// other Redmine objects (e.g. custom fields)
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProjectEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

impl From<Project> for ProjectEssentials {
    fn from(v: Project) -> Self {
        ProjectEssentials {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&Project> for ProjectEssentials {
    fn from(v: &Project) -> Self {
        ProjectEssentials {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

/// a type for projects to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct Project {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// URL slug
    pub identifier: String,
    /// description
    pub description: Option<String>,
    /// the project homepage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    /// is the project public (visible to anonymous users)
    pub is_public: Option<bool>,
    /// the parent project (id and name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ProjectEssentials>,
    /// will the project inherit members from its ancestors
    pub inherit_members: Option<bool>,
    /// the default user/group issues in this project are assigned to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_assignee: Option<AssigneeEssentials>,
    /// the default version for issues in this project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<VersionEssentials>,
    /// ID of the default version. It works only with existing shared versions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<u64>,
    /// trackers to enable in the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_ids: Option<Vec<u64>>,
    /// modules to enable in the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_module_names: Option<Vec<String>>,
    /// custom issue fields to enable in the project
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_custom_field_id: Option<Vec<u64>>,
    /// values for custom fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_values: Option<HashMap<u64, String>>,
    /// archived or not?
    pub status: u64,
    /// The time when this project was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// The time when this project was last updated
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub updated_on: time::OffsetDateTime,
    /// issue categories (only with include parameter)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_categories: Option<Vec<IssueCategoryEssentials>>,
    /// time entry activities (only with include parameter)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_entry_activities: Option<Vec<TimeEntryActivityEssentials>>,
    /// enabled modules in this project (only with include parameter)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_modules: Option<Vec<Module>>,
    /// trackers in this project (only with include parameter)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trackers: Option<Vec<TrackerEssentials>>,
}

/// The types of associated data which can be fetched along with a project
#[derive(Debug, Clone)]
pub enum ProjectsInclude {
    /// Trackers enabled in the project
    Trackers,
    /// Issue categories in the project
    IssueCategories,
    /// Redmine Modules enabled in the project
    EnabledModules,
}

impl std::fmt::Display for ProjectsInclude {
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
    include: Option<Vec<ProjectsInclude>>,
}

impl ReturnsJsonResponse for ListProjects {}
impl Pageable for ListProjects {
    fn response_wrapper_key(&self) -> String {
        "projects".to_string()
    }
}

impl ListProjects {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListProjectsBuilder {
        ListProjectsBuilder::default()
    }
}

impl Endpoint for ListProjects {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "projects.json".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("include", self.include.as_ref());
        params
    }
}

/// The types of associated data which can be fetched along with a project
#[derive(Debug, Clone)]
pub enum ProjectInclude {
    /// Trackers enabled in the project
    Trackers,
    /// Issue categories in the project
    IssueCategories,
    /// Redmine Modules enabled in the project
    EnabledModules,
    /// Time Entry Activities enabled in the project
    TimeEntryActivities,
}

impl std::fmt::Display for ProjectInclude {
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
    include: Option<Vec<ProjectInclude>>,
}

impl<'a> ReturnsJsonResponse for GetProject<'a> {}

impl<'a> GetProject<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
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
        params.push_opt("include", self.include.as_ref());
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
#[derive(Debug, Clone, Builder, Serialize)]
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
    #[must_use]
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
        Ok(Some((
            "application/json",
            serde_json::to_vec(&ProjectWrapper::<UpdateProject> {
                project: (*self).to_owned(),
            })?,
        )))
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
    #[must_use]
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
    pub projects: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a project field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct ProjectWrapper<T> {
    /// to parse JSON with project key
    pub project: T,
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
    pub static PROJECT_LOCK: RwLock<()> = const_rwlock(());

    #[traced_test]
    #[test]
    fn test_list_projects_no_pagination() -> Result<(), Box<dyn Error>> {
        let _r_project = PROJECT_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjects::builder().build()?;
        redmine.json_response_body::<_, ProjectsWrapper<Project>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_projects_first_page() -> Result<(), Box<dyn Error>> {
        let _r_project = PROJECT_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjects::builder().build()?;
        redmine.json_response_body_page::<_, Project>(&endpoint, 0, 25)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_projects_all_pages() -> Result<(), Box<dyn Error>> {
        let _r_project = PROJECT_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjects::builder().build()?;
        redmine.json_response_body_all_pages::<_, Project>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_project() -> Result<(), Box<dyn Error>> {
        let _r_project = PROJECT_LOCK.read();
        dotenvy::dotenv()?;
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
        with_project(&name, |_, _, _| Ok(()))?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_update_project() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, _id, name| {
            let update_endpoint = super::UpdateProject::builder()
                .project_id_or_name(name)
                .description("Test-Description")
                .build()?;
            redmine.ignore_response_body::<_>(&update_endpoint)?;
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
    fn test_completeness_project_type() -> Result<(), Box<dyn Error>> {
        let _r_project = PROJECT_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjects::builder().build()?;
        let ProjectsWrapper { projects: values } =
            redmine.json_response_body::<_, ProjectsWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: Project = serde_json::from_value(value.clone())?;
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
    /// this version of the test will load all pages of projects and the individual
    /// projects for each via GetProject which means it
    /// can take a while so you need to use --include-ignored
    /// or --ignored to run it
    #[traced_test]
    #[test]
    fn test_completeness_project_type_all_pages_all_project_details() -> Result<(), Box<dyn Error>>
    {
        let _r_project = PROJECT_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjects::builder()
            .include(vec![
                ProjectsInclude::Trackers,
                ProjectsInclude::IssueCategories,
                ProjectsInclude::EnabledModules,
            ])
            .build()?;
        let projects = redmine.json_response_body_all_pages::<_, Project>(&endpoint)?;
        for project in projects {
            let get_endpoint = GetProject::builder()
                .project_id_or_name(project.id.to_string())
                .include(vec![
                    ProjectInclude::Trackers,
                    ProjectInclude::IssueCategories,
                    ProjectInclude::EnabledModules,
                    ProjectInclude::TimeEntryActivities,
                ])
                .build()?;
            let ProjectWrapper { project: value } = redmine
                .json_response_body::<_, ProjectWrapper<serde_json::Value>>(&get_endpoint)?;
            let o: Project = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
