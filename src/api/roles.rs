//! Roles Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Roles)
//!
//! - [x] all roles endpoint
//! - [x] specific role endpoint

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::{Endpoint, NoPagination, ReturnsJsonResponse};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// a minimal type for Redmine roles used in lists of roles included in
/// other Redmine objects (e.g. custom fields) and also in the global ListRoles
/// endpoint (unlike most other Redmine API objects)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RoleEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// true if this role is inherited from a parent project, used e.g. in project memberships
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
}

/// determines which issues are visible to users/group with a role
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IssuesVisibility {
    /// a user/group with the role can see all issues (in visible projects)
    #[serde(rename = "all")]
    All,
    /// a user/group with the role can see all non-private issues (in visible projects)
    #[serde(rename = "default")]
    AllNonPrivate,
    /// a user/group with the role can see only issues created by or assigned to them
    #[serde(rename = "own")]
    Own,
}

/// determines which time entries are visible to users/group with a role
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TimeEntriesVisibility {
    /// a user/group with the role can see all time entries (in visible projects)
    #[serde(rename = "all")]
    All,
    /// a user/group with the role can see only time entries created by them
    #[serde(rename = "own")]
    Own,
}

/// determines which users are visible to users/group with a role
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UsersVisibility {
    /// a user/group with the role can see all active users
    #[serde(rename = "all")]
    All,
    /// a user/group with the role can only see users which are members of the project
    #[serde(rename = "members_of_visible_projects")]
    MembersOfVisibleProjects,
}

/// The type of a built-in role
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum BuiltinRole {
    /// custom role
    Custom = 0,
    /// non-member role
    NonMember = 1,
    /// anonymous role
    Anonymous = 2,
}

/// a type for roles to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Role {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// if this is true users/groups with this role can be assignees for issues
    pub assignable: bool,
    /// the issues that can be seen by users/groups with this role
    pub issues_visibility: IssuesVisibility,
    /// the time entries that can be seen by users/groups with this role
    pub time_entries_visibility: TimeEntriesVisibility,
    /// the users that can be seen by users/groups with this role
    pub users_visibility: UsersVisibility,
    /// list of permissions, this can contain core Redmine permissions
    /// and those provided by plugins
    pub permissions: Vec<String>,
    /// the position of the role in the list
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    /// the type of built-in role
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builtin: Option<BuiltinRole>,
    /// settings for the role
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    /// the default time entry activity for this role
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_time_entry_activity_id: Option<u64>,
}

/// The endpoint for all roles
///
/// unlike most other Redmine objects this only returns a RoleEssentials like
/// minimal object
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListRoles {
    /// filter for givable roles
    #[builder(default)]
    givable: Option<bool>,
}

impl ReturnsJsonResponse for ListRoles {}
impl NoPagination for ListRoles {}

impl ListRoles {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListRolesBuilder {
        ListRolesBuilder::default()
    }
}

impl Endpoint for ListRoles {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "roles.json".into()
    }

    fn parameters(&self) -> crate::api::QueryParams<'_> {
        let mut params = crate::api::QueryParams::default();
        params.push_opt("givable", self.givable);
        params
    }
}

/// The endpoint for a specific role
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetRole {
    /// the id of the role to retrieve
    id: u64,
}

impl ReturnsJsonResponse for GetRole {}
impl NoPagination for GetRole {}

impl GetRole {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetRoleBuilder {
        GetRoleBuilder::default()
    }
}

impl Endpoint for GetRole {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("roles/{}.json", self.id).into()
    }
}

/// helper struct for outer layers with a roles field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RolesWrapper<T> {
    /// to parse JSON with roles key
    pub roles: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a role field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RoleWrapper<T> {
    /// to parse JSON with role key
    pub role: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::test_helpers::with_redmine;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_list_roles_no_pagination() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let endpoint = ListRoles::builder().build()?;
            redmine.json_response_body::<_, RolesWrapper<RoleEssentials>>(&endpoint)?;
            Ok(())
        })
    }

    #[traced_test]
    #[test]
    fn test_list_roles_givable_filter() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let endpoint = ListRoles::builder().givable(true).build()?;
            redmine.json_response_body::<_, RolesWrapper<RoleEssentials>>(&endpoint)?;
            Ok(())
        })
    }

    #[test]
    fn test_get_role() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let endpoint = GetRole::builder().id(8).build()?;
            redmine.json_response_body::<_, RoleWrapper<Role>>(&endpoint)?;
            Ok(())
        })
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_role_type() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let list_endpoint = ListRoles::builder().build()?;
            let RolesWrapper { roles } =
                redmine.json_response_body::<_, RolesWrapper<RoleEssentials>>(&list_endpoint)?;
            for role in roles {
                let endpoint = GetRole::builder().id(role.id).build()?;
                let RoleWrapper { role: value } =
                    redmine.json_response_body::<_, RoleWrapper<serde_json::Value>>(&endpoint)?;
                let o: Role = serde_json::from_value(value.clone())?;
                let reserialized = serde_json::to_value(o)?;
                assert_eq!(value, reserialized);
            }
            Ok(())
        })
    }
}
