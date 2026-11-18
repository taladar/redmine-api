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
use reqwest::Method;
use std::borrow::Cow;

use crate::api::groups::GroupEssentials;
use crate::api::projects::ProjectEssentials;
use crate::api::roles::RoleEssentials;
use crate::api::users::UserEssentials;
use crate::api::{Endpoint, NoPagination, Pageable, ReturnsJsonResponse};
use serde::Serialize;

/// a minimal type for project memberships to be used in lists of memberships
/// returned as part of the user
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListProjectMemberships<'a> {
    /// project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl ReturnsJsonResponse for ListProjectMemberships<'_> {}
impl Pageable for ListProjectMemberships<'_> {
    fn response_wrapper_key(&self) -> String {
        "memberships".to_string()
    }
}

impl<'a> ListProjectMemberships<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListProjectMembershipsBuilder<'a> {
        ListProjectMembershipsBuilder::default()
    }
}

impl Endpoint for ListProjectMemberships<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/memberships.json", self.project_id_or_name).into()
    }
}

/// The endpoint for a specific Redmine project membership
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetProjectMembership {
    /// id of the project membership to retrieve
    id: u64,
}

impl ReturnsJsonResponse for GetProjectMembership {}
impl NoPagination for GetProjectMembership {}

impl GetProjectMembership {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetProjectMembershipBuilder {
        GetProjectMembershipBuilder::default()
    }
}

impl Endpoint for GetProjectMembership {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("memberships/{}.json", self.id).into()
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
    /// users to add to the project
    user_ids: Vec<u64>,
    /// roles for the user to add to the project
    role_ids: Vec<u64>,
}

impl ReturnsJsonResponse for CreateProjectMembership<'_> {}
impl NoPagination for CreateProjectMembership<'_> {}

impl<'a> CreateProjectMembership<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> CreateProjectMembershipBuilder<'a> {
        CreateProjectMembershipBuilder::default()
    }
}

impl Endpoint for CreateProjectMembership<'_> {
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
    #[must_use]
    pub fn builder() -> UpdateProjectMembershipBuilder {
        UpdateProjectMembershipBuilder::default()
    }
}

impl Endpoint for UpdateProjectMembership {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("memberships/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&MembershipWrapper::<Self> {
                membership: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to delete a membership in a Redmine project
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteProjectMembership {
    /// id of the project membership to delete
    id: u64,
}

impl DeleteProjectMembership {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> DeleteProjectMembershipBuilder {
        DeleteProjectMembershipBuilder::default()
    }
}

impl Endpoint for DeleteProjectMembership {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("memberships/{}.json", self.id).into()
    }
}

/// helper struct for outer layers with a memberships field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct MembershipsWrapper<T> {
    /// to parse JSON with memberships key
    pub memberships: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a membership field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct MembershipWrapper<T> {
    /// to parse JSON with membership key
    pub membership: T,
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::api::test_helpers::{current_user_id, with_project};
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tokio::sync::RwLock;
    use tracing_test::traced_test;

    /// needed so we do not get 404s when listing while
    /// creating/deleting or creating/updating/deleting
    pub static PROJECT_MEMBERSHIP_LOCK: RwLock<()> = RwLock::const_new(());

    /// returns the ids of all roles which can be given to project members
    fn role_ids(redmine: &crate::api::Redmine) -> Result<Vec<u64>, Box<dyn Error>> {
        use crate::api::roles::{ListRoles, RolesWrapper};
        let endpoint = ListRoles::builder().build()?;
        let RolesWrapper { roles } =
            redmine.json_response_body::<_, RolesWrapper<RoleEssentials>>(&endpoint)?;
        if roles.is_empty() {
            return Err("no roles found".into());
        }
        Ok(roles.into_iter().map(|r| r.id).collect())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_list_project_memberships_first_page() -> Result<(), Box<dyn Error>> {
        let _r_project_memberships = PROJECT_MEMBERSHIP_LOCK.blocking_read();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, _project_id, name| {
            let endpoint = ListProjectMemberships::builder()
                .project_id_or_name(name)
                .build()?;
            let page = redmine.json_response_body_page::<_, ProjectMembership>(&endpoint, 0, 25)?;
            assert!(!page.values.is_empty());
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_list_project_memberships_all_pages() -> Result<(), Box<dyn Error>> {
        let _r_project_memberships = PROJECT_MEMBERSHIP_LOCK.blocking_read();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, _project_id, name| {
            let endpoint = ListProjectMemberships::builder()
                .project_id_or_name(name)
                .build()?;
            let memberships =
                redmine.json_response_body_all_pages::<_, ProjectMembership>(&endpoint)?;
            assert!(!memberships.is_empty());
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_get_project_membership() -> Result<(), Box<dyn Error>> {
        let _r_project_memberships = PROJECT_MEMBERSHIP_LOCK.blocking_read();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, _project_id, name| {
            let list_endpoint = ListProjectMemberships::builder()
                .project_id_or_name(name)
                .build()?;
            let memberships =
                redmine.json_response_body_all_pages::<_, ProjectMembership>(&list_endpoint)?;
            let membership = memberships.first().ok_or("no project membership found")?;
            let endpoint = GetProjectMembership::builder().id(membership.id).build()?;
            redmine.json_response_body::<_, MembershipWrapper<ProjectMembership>>(&endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    /// returns the membership of the current user in the project created by
    /// [`with_project`]
    fn current_user_membership(
        redmine: &crate::api::Redmine,
        project_id: u64,
    ) -> Result<ProjectMembership, Box<dyn Error>> {
        let user_id = current_user_id(redmine)?;
        let endpoint = ListProjectMemberships::builder()
            .project_id_or_name(project_id.to_string())
            .build()?;
        let memberships =
            redmine.json_response_body_all_pages::<_, ProjectMembership>(&endpoint)?;
        memberships
            .into_iter()
            .find(|m| m.user.as_ref().is_some_and(|u| u.id == user_id))
            .ok_or_else(|| "current user is not a member of the test project".into())
    }

    /// the project created by [`with_project`] already has the current user as
    /// a member so we remove that membership first and then create it again
    /// to avoid having to create a user (which is slow to delete on large
    /// Redmine instances)
    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_project_membership() -> Result<(), Box<dyn Error>> {
        let _w_project_memberships = PROJECT_MEMBERSHIP_LOCK.blocking_write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, project_id, _name| {
            let membership = current_user_membership(redmine, project_id)?;
            let delete_endpoint = super::DeleteProjectMembership::builder()
                .id(membership.id)
                .build()?;
            redmine.ignore_response_body::<_>(&delete_endpoint)?;
            let create_endpoint = super::CreateProjectMembership::builder()
                .project_id_or_name(project_id.to_string())
                .user_ids(vec![current_user_id(redmine)?])
                .role_ids(membership.roles.iter().map(|r| r.id).collect())
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
        let _w_project_memberships = PROJECT_MEMBERSHIP_LOCK.blocking_write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, project_id, _name| {
            let membership = current_user_membership(redmine, project_id)?;
            let mut new_role_ids: Vec<u64> = membership.roles.iter().map(|r| r.id).collect();
            if let Some(extra_role_id) = role_ids(redmine)?
                .into_iter()
                .find(|id| !new_role_ids.contains(id))
            {
                new_role_ids.push(extra_role_id);
            }
            let update_endpoint = super::UpdateProjectMembership::builder()
                .id(membership.id)
                .role_ids(new_role_ids.clone())
                .build()?;
            redmine.ignore_response_body::<_>(&update_endpoint)?;
            let get_endpoint = GetProjectMembership::builder().id(membership.id).build()?;
            let MembershipWrapper { membership } = redmine
                .json_response_body::<_, MembershipWrapper<ProjectMembership>>(&get_endpoint)?;
            for role_id in new_role_ids {
                assert!(membership.roles.iter().any(|r| r.id == role_id));
            }
            Ok(())
        })?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_completeness_project_membership_type() -> Result<(), Box<dyn Error>> {
        let _r_project_memberships = PROJECT_MEMBERSHIP_LOCK.blocking_read();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, _project_id, name| {
            let endpoint = ListProjectMemberships::builder()
                .project_id_or_name(name)
                .build()?;
            let values: Vec<serde_json::Value> = redmine.json_response_body_all_pages(&endpoint)?;
            assert!(!values.is_empty());
            for value in values {
                let o: ProjectMembership = serde_json::from_value(value.clone())?;
                let reserialized = serde_json::to_value(o)?;
                assert_eq!(value, reserialized);
            }
            Ok(())
        })?;
        Ok(())
    }
}
