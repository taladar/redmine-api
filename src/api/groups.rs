//! Groups Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Groups)
//!
//! - [x] all groups endpoint
//! - [x] specific group endpoint
//! - [x] create group endpoint
//! - [x] update group endpoint
//! - [x] delete group endpoint
//! - [x] add user to group endpoint
//! - [x] remove user from group endpoint
//! - [x] remove users from group endpoint (Redmine 7.0 and later)

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::custom_fields::CustomField;
use crate::api::custom_fields::CustomFieldEssentialsWithValue;
use crate::api::project_memberships::GroupProjectMembership;
use crate::api::users::UserEssentials;
use crate::api::{Endpoint, NoPagination, QueryParams, ReturnsJsonResponse};
use serde::Serialize;

/// a minimal type for Redmine groups used in lists of groups included in
/// other Redmine objects
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GroupEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

impl From<Group> for GroupEssentials {
    fn from(v: Group) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&Group> for GroupEssentials {
    fn from(v: &Group) -> Self {
        Self {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

/// a type for groups to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct Group {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// users (only with include parameter)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserEssentials>>,
    /// project memberships (only with include parameter)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<GroupProjectMembership>>,
    /// custom fields with values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldEssentialsWithValue>>,
}

/// The endpoint for all Redmine groups
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListGroups {
    /// filter for builtin groups
    #[builder(default)]
    builtin: Option<bool>,
}

impl ReturnsJsonResponse for ListGroups {}
impl NoPagination for ListGroups {}

impl ListGroups {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListGroupsBuilder {
        ListGroupsBuilder::default()
    }
}

impl Endpoint for ListGroups {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "groups.json".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("builtin", self.builtin);
        params
    }
}

/// The types of associated data which can be fetched along with a group
#[derive(Debug, Clone)]
pub enum GroupInclude {
    /// The group members
    Users,
    /// The project memberships for this group
    Memberships,
}

impl std::fmt::Display for GroupInclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Users => {
                write!(f, "users")
            }
            Self::Memberships => {
                write!(f, "memberships")
            }
        }
    }
}

/// The endpoint for a specific Redmine group
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetGroup {
    /// id of the group
    id: u64,
    /// associated data to include
    #[builder(default)]
    include: Option<Vec<GroupInclude>>,
}

impl ReturnsJsonResponse for GetGroup {}
impl NoPagination for GetGroup {}

impl GetGroup {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetGroupBuilder {
        GetGroupBuilder::default()
    }
}

impl Endpoint for GetGroup {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}.json", self.id).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("include", self.include.as_ref());
        params
    }
}

/// The endpoint to create a Redmine group
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateGroup<'a> {
    /// name of the group
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// whether two-factor authentication is required for this group
    #[builder(default)]
    twofa_required: Option<bool>,
    /// user ids of users to put in the group initially
    #[builder(default)]
    user_ids: Option<Vec<u64>>,
    /// custom field values
    #[builder(default)]
    custom_fields: Option<Vec<CustomField<'a>>>,
}

impl ReturnsJsonResponse for CreateGroup<'_> {}
impl NoPagination for CreateGroup<'_> {}

impl<'a> CreateGroup<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> CreateGroupBuilder<'a> {
        CreateGroupBuilder::default()
    }
}

impl Endpoint for CreateGroup<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "groups.json".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&GroupWrapper::<CreateGroup> {
                group: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to update an existing Redmine group
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateGroup<'a> {
    /// id of the group to update
    #[serde(skip_serializing)]
    id: u64,
    /// name of the group
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// whether two-factor authentication is required for this group
    #[builder(default)]
    twofa_required: Option<bool>,
    /// user ids of the group members
    #[builder(default)]
    user_ids: Option<Vec<u64>>,
    /// custom field values
    #[builder(default)]
    custom_fields: Option<Vec<CustomField<'a>>>,
}

impl<'a> UpdateGroup<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> UpdateGroupBuilder<'a> {
        UpdateGroupBuilder::default()
    }
}

impl Endpoint for UpdateGroup<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&GroupWrapper::<UpdateGroup> {
                group: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to delete a Redmine group
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteGroup {
    /// Id of the group to delete
    id: u64,
}

impl DeleteGroup {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> DeleteGroupBuilder {
        DeleteGroupBuilder::default()
    }
}

impl Endpoint for DeleteGroup {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}.json", self.id).into()
    }
}

/// The endpoint to add a Redmine user to a Redmine group
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct AddUserToGroup {
    /// Group Id to add the user to
    #[serde(skip_serializing)]
    group_id: u64,
    /// User to add to this group
    user_id: u64,
}

impl AddUserToGroup {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> AddUserToGroupBuilder {
        AddUserToGroupBuilder::default()
    }
}

impl Endpoint for AddUserToGroup {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/users.json", self.group_id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to remove a Redmine user from a Redmine group
///
/// This works with all supported Redmine versions but is deprecated in
/// Redmine 7.0 and scheduled for removal in Redmine 8.0. When targeting only
/// Redmine 7.0 or later prefer [`RemoveUsersFromGroup`].
///
/// Starting with Redmine 7.0 this returns a 404 if the user is not a member
/// of the group.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct RemoveUserFromGroup {
    /// Group Id to remove the user from
    group_id: u64,
    /// User to remove from the group
    user_id: u64,
}

impl RemoveUserFromGroup {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> RemoveUserFromGroupBuilder {
        RemoveUserFromGroupBuilder::default()
    }
}

impl Endpoint for RemoveUserFromGroup {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/users/{}.json", self.group_id, self.user_id).into()
    }
}

/// The endpoint to remove one or more Redmine users from a Redmine group
///
/// This endpoint requires Redmine 7.0 or later, older versions return a 404.
/// Use [`RemoveUserFromGroup`] for older versions.
///
/// Redmine returns a 404 if none of the given users are members of the group.
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct RemoveUsersFromGroup {
    /// Group Id to remove the users from
    #[serde(skip_serializing)]
    group_id: u64,
    /// Users to remove from the group
    user_ids: Vec<u64>,
}

impl RemoveUsersFromGroup {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> RemoveUsersFromGroupBuilder {
        RemoveUsersFromGroupBuilder::default()
    }
}

impl Endpoint for RemoveUsersFromGroup {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/users.json", self.group_id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// helper struct for outer layers with a groups field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct GroupsWrapper<T> {
    /// to parse JSON with groups key
    pub groups: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a group field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct GroupWrapper<T> {
    /// to parse JSON with group key
    pub group: T,
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::api::project_memberships::test::PROJECT_MEMBERSHIP_LOCK;
    use crate::api::test_helpers::{RedmineVersion, current_user_id, redmine_version, with_group};
    use crate::api::users::test::USER_LOCK;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tokio::sync::RwLock;
    use tracing::trace;
    use tracing_test::traced_test;

    /// needed so we do not get 404s when listing while
    /// creating/deleting or creating/updating/deleting
    pub static GROUP_LOCK: RwLock<()> = RwLock::const_new(());

    #[traced_test]
    #[test]
    fn test_list_groups_no_pagination() -> Result<(), Box<dyn Error>> {
        let _r_groups = GROUP_LOCK.blocking_read();
        crate::api::test_helpers::setup()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .tls_backend_rustls()
                .build()?,
        )?;
        let endpoint = ListGroups::builder().build()?;
        redmine.json_response_body::<_, GroupsWrapper<Group>>(&endpoint)?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_get_group() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        with_group(&name, |redmine, id, name| {
            let endpoint = GetGroup::builder().id(id).build()?;
            let GroupWrapper { group } =
                redmine.json_response_body::<_, GroupWrapper<Group>>(&endpoint)?;
            assert_eq!(group.name, name);
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_group() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        with_group(&name, |_, _, _| Ok(()))?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_update_group() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        with_group(&name, |redmine, id, _name| {
            let update_endpoint = super::UpdateGroup::builder()
                .id(id)
                .name("unittest_rename_test")
                .build()?;
            redmine.ignore_response_body::<_>(&update_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    /// returns the ids of the users in the group
    fn group_user_ids(
        redmine: &crate::api::Redmine,
        group_id: u64,
    ) -> Result<Vec<u64>, Box<dyn Error>> {
        let endpoint = GetGroup::builder()
            .id(group_id)
            .include(vec![GroupInclude::Users])
            .build()?;
        let GroupWrapper { group } =
            redmine.json_response_body::<_, GroupWrapper<Group>>(&endpoint)?;
        Ok(group
            .users
            .unwrap_or_default()
            .into_iter()
            .map(|u| u.id)
            .collect())
    }

    /// creates a group, adds the current user to it, calls the function with
    /// the group id and the user id and deletes the group afterwards
    ///
    /// this uses the current user instead of creating a user because deleting
    /// users can take a very long time on large Redmine instances
    fn with_current_user_in_group<F>(name: &str, f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce(&crate::api::Redmine, u64, u64) -> Result<(), Box<dyn Error>>,
    {
        with_group(name, |redmine, group_id, _name| {
            let user_id = current_user_id(redmine)?;
            let add_endpoint = AddUserToGroup::builder()
                .group_id(group_id)
                .user_id(user_id)
                .build()?;
            redmine.ignore_response_body::<_>(&add_endpoint)?;
            assert_eq!(group_user_ids(redmine, group_id)?, vec![user_id]);
            f(redmine, group_id, user_id)
        })
    }

    /// checks whether the Redmine instance has the route used by
    /// [`RemoveUsersFromGroup`] (Redmine 7.0 and later)
    ///
    /// sends the request without any users to an empty group, Redmine 7.0 and
    /// later answers this with an empty API 404 response while older versions
    /// do not have the route at all and answer with the HTML 404 page
    fn has_remove_users_from_group_route(group_id: u64) -> Result<bool, Box<dyn Error>> {
        let url = std::env::var("REDMINE_URL")?;
        let api_key = std::env::var("REDMINE_API_KEY")?;
        let response = reqwest::blocking::Client::builder()
            .tls_backend_rustls()
            .build()?
            .delete(format!(
                "{}/groups/{group_id}/users.json",
                url.trim_end_matches('/')
            ))
            .header("X-Redmine-API-Key", api_key)
            .send()?;
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_owned();
        trace!(status = %response.status(), %content_type, "RemoveUsersFromGroup probe");
        Ok(response.status() == reqwest::StatusCode::NOT_FOUND
            && content_type.starts_with("application/json"))
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_add_and_remove_user_from_group() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        with_current_user_in_group(&name, |redmine, group_id, user_id| {
            let remove_endpoint = RemoveUserFromGroup::builder()
                .group_id(group_id)
                .user_id(user_id)
                .build()?;
            redmine.ignore_response_body::<_>(&remove_endpoint)?;
            assert_eq!(group_user_ids(redmine, group_id)?, Vec::<u64>::new());
            Ok(())
        })
    }

    /// the endpoint was added in Redmine 7.0, on older versions (according to
    /// `REDMINE_VERSION`) this only checks that the endpoint does not exist yet
    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_add_and_remove_users_from_group() -> Result<(), Box<dyn Error>> {
        let version = redmine_version()?;
        let name = format!("unittest_{}", function_name!());
        with_group(&name, |redmine, group_id, _name| {
            let has_route = has_remove_users_from_group_route(group_id)?;
            if version < RedmineVersion::new(7, 0, 0) {
                assert!(
                    !has_route,
                    "REDMINE_VERSION is {version} but the Redmine instance has the \
                     RemoveUsersFromGroup endpoint added in Redmine 7.0, \
                     is REDMINE_VERSION correct?"
                );
                return Ok(());
            }
            assert!(
                has_route,
                "REDMINE_VERSION is {version} but the Redmine instance does not have the \
                 RemoveUsersFromGroup endpoint added in Redmine 7.0, \
                 is REDMINE_VERSION correct?"
            );
            let user_id = current_user_id(redmine)?;
            let add_endpoint = AddUserToGroup::builder()
                .group_id(group_id)
                .user_id(user_id)
                .build()?;
            redmine.ignore_response_body::<_>(&add_endpoint)?;
            assert_eq!(group_user_ids(redmine, group_id)?, vec![user_id]);
            let remove_endpoint = RemoveUsersFromGroup::builder()
                .group_id(group_id)
                .user_ids(vec![user_id])
                .build()?;
            redmine.ignore_response_body::<_>(&remove_endpoint)?;
            assert_eq!(group_user_ids(redmine, group_id)?, Vec::<u64>::new());
            Ok(())
        })
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_group_type() -> Result<(), Box<dyn Error>> {
        let _r_groups = GROUP_LOCK.blocking_read();
        crate::api::test_helpers::setup()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .tls_backend_rustls()
                .build()?,
        )?;
        let endpoint = ListGroups::builder().build()?;
        let GroupsWrapper { groups: values } =
            redmine.json_response_body::<_, GroupsWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: Group = serde_json::from_value(value.clone())?;
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
    /// this version of the test will load all groups and the individual
    /// groups for each via GetGroup
    #[traced_test]
    #[test]
    fn test_completeness_group_type_all_group_details() -> Result<(), Box<dyn Error>> {
        let _r_user = USER_LOCK.blocking_read();
        let _r_groups = GROUP_LOCK.blocking_read();
        let _r_project_memberships = PROJECT_MEMBERSHIP_LOCK.blocking_read();
        crate::api::test_helpers::setup()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .tls_backend_rustls()
                .build()?,
        )?;
        let endpoint = ListGroups::builder().build()?;
        let GroupsWrapper { groups } =
            redmine.json_response_body::<_, GroupsWrapper<Group>>(&endpoint)?;
        for group in groups {
            let get_endpoint = GetGroup::builder()
                .id(group.id)
                .include(vec![GroupInclude::Users, GroupInclude::Memberships])
                .build()?;
            let GroupWrapper { group: value } =
                redmine.json_response_body::<_, GroupWrapper<serde_json::Value>>(&get_endpoint)?;
            let o: Group = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
