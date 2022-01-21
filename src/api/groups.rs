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
use http::Method;
use std::borrow::Cow;

use crate::api::{Endpoint, QueryParams};
use serde::Serialize;

/// The endpoint for all Redmine groups
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Groups {}

impl Groups {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupsBuilder {
        GroupsBuilder::default()
    }
}

impl<'a> Endpoint for Groups {
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
pub struct Group {
    /// id of the group
    id: u64,
    /// associated data to include
    #[builder(default)]
    include: Option<Vec<GroupInclude>>,
}

impl Group {
    /// Create a builder for the endpoint.
    pub fn builder() -> GroupBuilder {
        GroupBuilder::default()
    }
}

impl<'a> Endpoint for Group {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}.json", &self.id).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("includes", self.include.as_ref());
        params
    }
}

/// The endpoint to create a Redmine group
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateGroup<'a> {
    /// name of the group
    name: Cow<'a, str>,
    /// user ids of users to put in the group initially
    #[builder(default)]
    user_ids: Option<Vec<u64>>,
}

impl<'a> CreateGroup<'a> {
    /// Create a builder for the endpoint.
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
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to update an existing Redmine group
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateGroup<'a> {
    /// id of the group to update
    #[serde(skip_serializing)]
    id: u64,
    /// name of the group
    name: Cow<'a, str>,
    /// user ids of the group members
    #[builder(default)]
    user_ids: Option<Vec<u64>>,
}

impl<'a> UpdateGroup<'a> {
    /// Create a builder for the endpoint.
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
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
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
    pub fn builder() -> DeleteGroupBuilder {
        DeleteGroupBuilder::default()
    }
}

impl<'a> Endpoint for DeleteGroup {
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
    pub fn builder() -> AddUserToGroupBuilder {
        AddUserToGroupBuilder::default()
    }
}

impl<'a> Endpoint for AddUserToGroup {
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
    pub fn builder() -> RemoveUserFromGroupBuilder {
        RemoveUserFromGroupBuilder::default()
    }
}

impl<'a> Endpoint for RemoveUserFromGroup {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("groups/{}/users/{}.json", &self.group_id, &self.user_id).into()
    }
}
