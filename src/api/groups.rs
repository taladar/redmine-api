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

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::project_memberships::GroupProjectMembership;
use crate::api::users::UserEssentials;
use crate::api::{Endpoint, QueryParams, ReturnsJsonResponse};
use serde::Serialize;

/// a minimal type for Redmine groups used in lists of groups included in
/// other Redmine objects
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
pub struct GroupEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

impl From<Group> for GroupEssentials {
    fn from(v: Group) -> Self {
        GroupEssentials {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&Group> for GroupEssentials {
    fn from(v: &Group) -> Self {
        GroupEssentials {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

/// a type for groups to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
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
}

/// The endpoint for all Redmine groups
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ListGroups {}

impl ReturnsJsonResponse for ListGroups {}

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
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct GetGroup {
    /// id of the group
    id: u64,
    /// associated data to include
    #[builder(default)]
    include: Option<Vec<GroupInclude>>,
}

impl ReturnsJsonResponse for GetGroup {}

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
        format!("groups/{}.json", &self.id).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("include", self.include.as_ref());
        params
    }
}

/// The endpoint to create a Redmine group
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateGroup<'a> {
    /// name of the group
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// user ids of users to put in the group initially
    #[builder(default)]
    user_ids: Option<Vec<u64>>,
}

impl<'a> ReturnsJsonResponse for CreateGroup<'a> {}

impl<'a> CreateGroup<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> CreateGroupBuilder<'a> {
        CreateGroupBuilder::default()
    }
}

impl<'a> Endpoint for CreateGroup<'a> {
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
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateGroup<'a> {
    /// id of the group to update
    #[serde(skip_serializing)]
    id: u64,
    /// name of the group
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// user ids of the group members
    #[builder(default)]
    user_ids: Option<Vec<u64>>,
}

impl<'a> UpdateGroup<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> UpdateGroupBuilder<'a> {
        UpdateGroupBuilder::default()
    }
}

impl<'a> Endpoint for UpdateGroup<'a> {
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
#[derive(Debug, Builder)]
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
        format!("groups/{}.json", &self.id).into()
    }
}

/// The endpoint to add a Redmine user to a Redmine group
#[derive(Debug, Builder, Serialize)]
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
        format!("groups/{}/users.json", &self.group_id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to remove a Redmine user from a Redmine group
#[derive(Debug, Builder)]
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
        format!("groups/{}/users/{}.json", &self.group_id, &self.user_id).into()
    }
}

/// helper struct for outer layers with a groups field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct GroupsWrapper<T> {
    /// to parse JSON with groups key
    pub groups: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a group field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct GroupWrapper<T> {
    /// to parse JSON with group key
    pub group: T,
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::api::test_helpers::with_group;
    use parking_lot::{const_rwlock, RwLock};
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    /// needed so we do not get 404s when listing while
    /// creating/deleting or creating/updating/deleting
    pub static GROUP_LOCK: RwLock<()> = const_rwlock(());

    #[traced_test]
    #[test]
    fn test_list_groups_no_pagination() -> Result<(), Box<dyn Error>> {
        let _r_groups = GROUP_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListGroups::builder().build()?;
        redmine.json_response_body::<_, GroupsWrapper<Group>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_group() -> Result<(), Box<dyn Error>> {
        let _r_groups = GROUP_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = GetGroup::builder().id(338).build()?;
        redmine.json_response_body::<_, GroupWrapper<Group>>(&endpoint)?;
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
    fn test_update_project() -> Result<(), Box<dyn Error>> {
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

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_group_type() -> Result<(), Box<dyn Error>> {
        let _r_groups = GROUP_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
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
        let _r_groups = GROUP_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
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
