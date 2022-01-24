//! Users Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Users)
//!
//! - [x] all users endpoint
//!   - [x] status filter
//!   - [x] name filter
//!   - [x] group_id filter
//! - [x] specific user endpoint
//!   - [x] by user id
//!   - [x] current
//! - [x] create user endpoint
//! - [x] update user endpoint
//! - [x] delete user endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::{Endpoint, Pageable, QueryParams};
use serde::Serialize;

/// The user status values for filtering
#[derive(Debug, Clone)]
pub enum UserStatus {
    /// User can login and use their account (default)
    Active,
    /// User has registered but not yet confirmed their email address or was not yet activated by an administrator. User can not login
    Registered,
    /// User was once active and is now locked, User can not login
    Locked,
    /// Specify this to get users with any status
    AnyStatus,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => {
                write!(f, "Active")
            }
            Self::Registered => {
                write!(f, "Registered")
            }
            Self::Locked => {
                write!(f, "Locked")
            }
            Self::AnyStatus => {
                write!(f, "")
            }
        }
    }
}

/// The endpoint for all users
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Users<'a> {
    /// Filter by user status
    #[builder(default)]
    /// The status of the users (locked, registered but not confirmed yet,...)
    status: Option<UserStatus>,
    #[builder(default)]
    /// Filter by name, this matches login, firstname, lastname and if it contains a space also firstname and lastname
    name: Option<Cow<'a, str>>,
    /// Users need to be members of this group
    #[builder(default)]
    group_id: Option<u64>,
}

impl<'a> Pageable for Users<'a> {
    fn response_wrapper_key(&self) -> String {
        "users".to_string()
    }
}

impl<'a> Users<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UsersBuilder<'a> {
        UsersBuilder::default()
    }
}

impl<'a> Endpoint for Users<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "users.json".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("status", self.status.as_ref().map(|s| s.to_string()));
        params.push_opt("name", self.name.as_ref());
        params.push_opt("group_id", self.group_id);
        params
    }
}

/// The types of associated data which can be fetched along with a user
#[derive(Debug, Clone)]
pub enum UserInclude {
    /// The project memberships of this user
    Memberships,
    /// The groups where this user is a member
    Groups,
}

impl std::fmt::Display for UserInclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Memberships => {
                write!(f, "memberships")
            }
            Self::Groups => {
                write!(f, "groups")
            }
        }
    }
}

/// The endpoint for a specific user
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct User {
    /// User id to fetch, if not specified will fetch the current user
    id: Option<u64>,
    /// Include associated data
    #[builder(default)]
    include: Option<Vec<UserInclude>>,
}

impl User {
    /// Create a builder for the endpoint.
    pub fn builder() -> UserBuilder {
        UserBuilder::default()
    }
}

impl<'a> Endpoint for User {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        match self.id {
            Some(id) => format!("users/{}.json", id).into(),
            None => "users/current.json".into(),
        }
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("includes", self.include.as_ref());
        params
    }
}

/// Possible values for mail notification options for a user
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MailNotificationOptions {
    /// Get notified by all events (visible to user)
    All,
    /// This allows to be notified only by selected projects, not sure if those can be selected via the API
    Selected,
    /// Only get notifications for events caused by the user's own actions
    OnlyMyEvents,
    /// Only get notifications for events in issues assigned to the user
    OnlyAssigned,
    /// Only get notifications for events in issues owned by the user
    OnlyOwner,
    /// Do not get any notifications
    #[serde(rename = "none")]
    NoMailNotifications,
}

/// The endpoint to create a Redmine user
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateUser<'a> {
    /// The login for the user
    #[builder(setter(into))]
    login: Cow<'a, str>,
    /// The user's password
    ///
    /// It is recommended to use generate_password instead
    #[builder(setter(into), default)]
    password: Option<Cow<'a, str>>,
    /// The user's firstname
    #[builder(setter(into))]
    firstname: Cow<'a, str>,
    /// The user's lastname
    #[builder(setter(into))]
    lastname: Cow<'a, str>,
    /// The users primary email address
    #[builder(setter(into))]
    mail: Cow<'a, str>,
    /// allows setting users to be e.g. LDAP users
    #[builder(default)]
    auth_source_id: Option<u64>,
    /// what kind of mail notifications should be sent to the user
    #[builder(default)]
    mail_notification: Option<MailNotificationOptions>,
    /// if set the user must change their password after the next login
    #[builder(default)]
    must_change_passwd: Option<bool>,
    /// generate a random password
    #[builder(default)]
    generate_password: Option<bool>,
    /// Send account information to the user
    #[builder(default)]
    send_information: Option<bool>,
    /// Make the user a Redmine administrator
    #[builder(default)]
    admin: Option<bool>,
}

impl<'a> CreateUser<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateUserBuilder<'a> {
        CreateUserBuilder::default()
    }
}

impl<'a> Endpoint for CreateUser<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "users.json".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to update an existing Redmine user
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateUser<'a> {
    /// The id of the user to update
    #[serde(skip_serializing)]
    id: u64,
    /// The login for the user
    #[builder(setter(into))]
    login: Cow<'a, str>,
    /// The user's password
    ///
    /// It is recommended to use generate_password instead
    #[builder(setter(into), default)]
    password: Option<Cow<'a, str>>,
    /// The user's firstname
    #[builder(setter(into))]
    firstname: Cow<'a, str>,
    /// The user's lastname
    #[builder(setter(into))]
    lastname: Cow<'a, str>,
    /// The users primary email address
    #[builder(setter(into))]
    mail: Cow<'a, str>,
    /// allows setting users to be e.g. LDAP users
    #[builder(default)]
    auth_source_id: Option<u64>,
    /// what kind of mail notifications should be sent to the user
    #[builder(default)]
    mail_notification: Option<MailNotificationOptions>,
    /// if set the user must change their password after the next login
    #[builder(default)]
    must_change_passwd: Option<bool>,
    /// generate a random password
    #[builder(default)]
    generate_password: Option<bool>,
    /// Send account information to the user
    #[builder(default)]
    send_information: Option<bool>,
    /// Make the user a Redmine administrator
    #[builder(default)]
    admin: Option<bool>,
}

impl<'a> UpdateUser<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UpdateUserBuilder<'a> {
        UpdateUserBuilder::default()
    }
}

impl<'a> Endpoint for UpdateUser<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to delete a Redmine user
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteUser {
    /// The id of the user to delete
    id: u64,
}

impl DeleteUser {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteUserBuilder {
        DeleteUserBuilder::default()
    }
}

impl<'a> Endpoint for DeleteUser {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}.json", &self.id).into()
    }
}
