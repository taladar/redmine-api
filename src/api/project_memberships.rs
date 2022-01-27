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

use crate::api::groups::GroupEssentials;
use crate::api::projects::ProjectEssentials;
use crate::api::roles::RoleEssentials;
use crate::api::users::UserEssentials;
use crate::api::{Endpoint, Pageable, ReturnsJsonResponse};
use serde::Serialize;

/// a minimal type for project memberships to be used in lists of memberships
/// returned as part of the user
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct UserProjectMembership {
    /// numeric id
    pub id: u64,
    /// the project
    pub project: ProjectEssentials,
    /// the roles the user has in the project
    pub roles: Vec<RoleEssentials>,
}

/// a minimal type for project memberships to be used in lists of memberships
/// returned as part of the group
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct GroupProjectMembership {
    /// numeric id
    pub id: u64,
    /// the project
    pub project: ProjectEssentials,
    /// the roles the group has in the project
    pub roles: Vec<RoleEssentials>,
}

/// a type for project memberships to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct ProjectMembership {
    /// numeric id
    pub id: u64,
    /// the project
    pub project: ProjectEssentials,
    /// the user (project member), optional because alternatively we could have a group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserEssentials>,
    /// the group (project member), optional because alternatively we could have a user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupEssentials>,
    /// the roles the user or group has in the project
    pub roles: Vec<RoleEssentials>,
}

/// The endpoint for all memberships in a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ListProjectMemberships<'a> {
    /// project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl<'a> ReturnsJsonResponse for ListProjectMemberships<'a> {}
impl<'a> Pageable for ListProjectMemberships<'a> {
    fn response_wrapper_key(&self) -> String {
        "memberships".to_string()
    }
}

impl<'a> ListProjectMemberships<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ListProjectMembershipsBuilder<'a> {
        ListProjectMembershipsBuilder::default()
    }
}

impl<'a> Endpoint for ListProjectMemberships<'a> {
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
pub struct GetProjectMembership {
    /// id of the project membership to retrieve
    id: u64,
}

impl ReturnsJsonResponse for GetProjectMembership {}

impl<'a> GetProjectMembership {
    /// Create a builder for the endpoint.
    pub fn builder() -> GetProjectMembershipBuilder {
        GetProjectMembershipBuilder::default()
    }
}

impl<'a> Endpoint for GetProjectMembership {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("memberships/{}.json", &self.id).into()
    }
}

/// The endpoint to create a Redmine project membership (add a user or group to a project)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
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

impl<'a> ReturnsJsonResponse for CreateProjectMembership<'a> {}

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
        Ok(Some((
            "application/json",
            serde_json::to_vec(&MembershipWrapper::<CreateProjectMembership> {
                membership: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to update an existing Redmine project membership (change roles)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
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
        Ok(Some((
            "application/json",
            serde_json::to_vec(&MembershipWrapper::<UpdateProjectMembership> {
                membership: (*self).to_owned(),
            })?,
        )))
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

/// helper struct for outer layers with a memberships field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct MembershipsWrapper<T> {
    /// to parse JSON with memberships key
    pub memberships: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a membership field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct MembershipWrapper<T> {
    /// to parse JSON with membership key
    pub membership: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::test_helpers::with_project;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_list_project_memberships_no_pagination() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjectMemberships::builder()
            .project_id_or_name("sandbox")
            .build()?;
        redmine.json_response_body::<_, MembershipsWrapper<ProjectMembership>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_project_memberships_first_page() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjectMemberships::builder()
            .project_id_or_name("sandbox")
            .build()?;
        redmine.json_response_body_page::<_, ProjectMembership>(&endpoint, 0, 25)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_project_memberships_all_pages() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjectMemberships::builder()
            .project_id_or_name("sandbox")
            .build()?;
        redmine.json_response_body_all_pages::<_, ProjectMembership>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_project_membership() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = GetProjectMembership::builder().id(238).build()?;
        redmine.json_response_body::<_, MembershipWrapper<ProjectMembership>>(&endpoint)?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_project_membership() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, project_id, _| {
            let create_endpoint = super::CreateProjectMembership::builder()
                .project_id_or_name(project_id.to_string())
                .user_id(1)
                .role_ids(vec![8])
                .build()?;
            redmine
                .json_response_body::<_, MembershipWrapper<ProjectMembership>>(&create_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_update_project_membership() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, project_id, _name| {
            let create_endpoint = super::CreateProjectMembership::builder()
                .project_id_or_name(project_id.to_string())
                .user_id(1)
                .role_ids(vec![8])
                .build()?;
            let MembershipWrapper { membership } = redmine
                .json_response_body::<_, MembershipWrapper<ProjectMembership>>(&create_endpoint)?;
            let update_endpoint = super::UpdateProjectMembership::builder()
                .id(membership.id)
                .role_ids(vec![9])
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
    fn test_completeness_project_membership_type() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListProjectMemberships::builder()
            .project_id_or_name("sandbox")
            .build()?;
        let MembershipsWrapper {
            memberships: values,
        } = redmine.json_response_body::<_, MembershipsWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: ProjectMembership = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
