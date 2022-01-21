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

use crate::api::{Endpoint, Pageable, QueryParams};
use serde::Serialize;
use std::collections::HashMap;

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

/// The endpoint for all Redmine projects
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Projects {}

impl Pageable for Projects {}

impl Projects {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectsBuilder {
        ProjectsBuilder::default()
    }
}

impl<'a> Endpoint for Projects {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "projects.json".into()
    }
}

/// The endpoint for a specific Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Project<'a> {
    /// the project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// the types of associate data to include
    #[builder(default)]
    include: Option<Vec<ProjectInclude>>,
}

impl<'a> Project<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectBuilder<'a> {
        ProjectBuilder::default()
    }
}

impl<'a> Endpoint for Project<'a> {
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
pub struct ArchiveProject {
    /// the id of the project to archive
    id: u64,
}

impl ArchiveProject {
    /// Create a builder for the endpoint.
    pub fn builder() -> ArchiveProjectBuilder {
        ArchiveProjectBuilder::default()
    }
}

impl<'a> Endpoint for ArchiveProject {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/archive.json", &self.id).into()
    }
}

/// The endpoint to unarchive a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct UnarchiveProject {
    /// the id of the project to unarchive
    id: u64,
}

impl UnarchiveProject {
    /// Create a builder for the endpoint.
    pub fn builder() -> UnarchiveProjectBuilder {
        UnarchiveProjectBuilder::default()
    }
}

impl<'a> Endpoint for UnarchiveProject {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/unarchive.json", &self.id).into()
    }
}

/// The endpoint to create a Redmine project
#[derive(Debug, Builder, Serialize)]
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
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to update an existing Redmine project
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateProject<'a> {
    /// the id of the project to update
    #[serde(skip_serializing)]
    id: u64,
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
        format!("projects/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to delete a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteProject {
    /// the id of the project to delete
    id: u64,
}

impl DeleteProject {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteProjectBuilder {
        DeleteProjectBuilder::default()
    }
}

impl<'a> Endpoint for DeleteProject {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}.json", &self.id).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;
    //use pretty_assertions::{assert_eq,assert_ne};
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_list_projects_no_pagination() -> Result<(), Box<dyn Error>> {
        #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
        struct Project {
            id: u64,
        }
        #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
        struct ProjectsWrapper {
            projects: Vec<Project>,
        }
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = Projects::builder().build()?;
        redmine.rest::<_, ProjectsWrapper>(&endpoint)?;
        Ok(())
    }
}
