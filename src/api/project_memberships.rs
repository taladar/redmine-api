//! Project Memberships Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Memberships)
//!
//! - [x] list of project memberships endpoint
//! - [x] get specific membership endpoint
//! - [x] create project membership endpoint
//! - [x] update specific membership endpoint
//! - [x] delete specific membership endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;
use serde::Serialize;

/// The endpoint for all memberships in a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ProjectMemberships<'a> {
    /// project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl<'a> ProjectMemberships<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectMembershipsBuilder<'a> {
        ProjectMembershipsBuilder::default()
    }
}

impl<'a> Endpoint for ProjectMemberships<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/memberships.json", self.project_id_or_name).into()
    }
}

/// The endpoint for a specific Redmine project membership
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ProjectMembership {
    /// id of the project membership to retrieve
    id: u64,
}

impl<'a> ProjectMembership {
    /// Create a builder for the endpoint.
    pub fn builder() -> ProjectMembershipBuilder {
        ProjectMembershipBuilder::default()
    }
}

impl<'a> Endpoint for ProjectMembership {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("memberships/{}.json", &self.id).into()
    }
}

/// The endpoint to create a Redmine project membership (add a user or group to a project)
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateProjectMembership<'a> {
    /// project id or name as it appears in the URL
    #[builder(setter(into))]
    #[serde(skip_serializing)]
    project_id_or_name: Cow<'a, str>,
    /// user to add to the project
    user_id: u64,
    /// roles for the user to add to the project
    role_ids: Vec<u64>,
}

impl<'a> CreateProjectMembership<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateProjectMembershipBuilder<'a> {
        CreateProjectMembershipBuilder::default()
    }
}

impl<'a> Endpoint for CreateProjectMembership<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/memberships.json", self.project_id_or_name).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to update an existing Redmine project membership (change roles)
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateProjectMembership {
    /// id of the project membership to update
    #[serde(skip_serializing)]
    id: u64,
    /// roles for the user to add to the project
    role_ids: Vec<u64>,
}

impl UpdateProjectMembership {
    /// Create a builder for the endpoint.
    pub fn builder() -> UpdateProjectMembershipBuilder {
        UpdateProjectMembershipBuilder::default()
    }
}

impl<'a> Endpoint for UpdateProjectMembership {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("memberships/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to delete a membership in a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteProjectMembership {
    /// id of the project membership to delete
    id: u64,
}

impl DeleteProjectMembership {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteProjectMembershipBuilder {
        DeleteProjectMembershipBuilder::default()
    }
}

impl<'a> Endpoint for DeleteProjectMembership {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("memberships/{}.json", &self.id).into()
    }
}
