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
use reqwest::Method;
use std::borrow::Cow;

use crate::api::custom_fields::CustomFieldEssentialsWithValue;
use crate::api::groups::GroupEssentials;
use crate::api::project_memberships::UserProjectMembership;
use crate::api::{
    CustomFieldFilter, DateTimeFilterPast, Endpoint, NoPagination, Pageable, QueryParams,
    ReturnsJsonResponse,
};
use serde::Serialize;

/// a minimal type for Redmine users used in
/// other Redmine objects (e.g. issue author)
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UserEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

/// a type for user to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    /// numeric id
    pub id: u64,
    /// user status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// login name
    pub login: String,
    /// is this user an admin
    pub admin: bool,
    /// user's firstname
    pub firstname: String,
    /// user's lastname
    pub lastname: String,
    /// primary email of the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    /// the user's API key
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// user's 2FA scheme
    #[serde(default)]
    pub twofa_scheme: Option<String>,
    /// allows setting users to be e.g. LDAP users
    #[serde(default, skip_serializing_if = "Option::is_none")]
    auth_source_id: Option<u64>,
    /// The time when this user was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// The time when this user was last updated
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub updated_on: time::OffsetDateTime,
    /// The time when this user's password was last changed
    #[serde(
        serialize_with = "crate::api::serialize_optional_rfc3339",
        deserialize_with = "crate::api::deserialize_optional_rfc3339"
    )]
    pub passwd_changed_on: Option<time::OffsetDateTime>,
    /// the time when this user last logged in
    #[serde(
        serialize_with = "crate::api::serialize_optional_rfc3339",
        deserialize_with = "crate::api::deserialize_optional_rfc3339"
    )]
    pub last_login_on: Option<time::OffsetDateTime>,
    /// custom fields with values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldEssentialsWithValue>>,
    /// groups (only if include is specified)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupEssentials>>,
    /// memberships (only if include is specified)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<UserProjectMembership>>,
}

/// The user status values
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserStatus {
    /// User can login and use their account (default)
    Active,
    /// User has registered but not yet confirmed their email address or was not yet activated by an administrator. User can not login
    Registered,
    /// User was once active and is now locked, User can not login
    Locked,
}

impl serde::Serialize for UserStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Active => serializer.serialize_u64(1),
            Self::Registered => serializer.serialize_u64(2),
            Self::Locked => serializer.serialize_u64(3),
        }
    }
}

impl<'de> serde::Deserialize<'de> for UserStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let status_code = u64::deserialize(deserializer)?;
        match status_code {
            1 => Ok(Self::Active),
            2 => Ok(Self::Registered),
            3 => Ok(Self::Locked),
            _ => Err(serde::de::Error::custom(format!(
                "unknown user status code: {status_code}"
            ))),
        }
    }
}

/// The user status values for filtering
#[derive(Debug, Clone)]
pub enum UserStatusFilter {
    /// User can login and use their account (default)
    Active,
    /// User has registered but not yet confirmed their email address or was not yet activated by an administrator. User can not login
    Registered,
    /// User was once active and is now locked, User can not login
    Locked,
    /// Specify this to get users with any status
    Any,
}

impl std::fmt::Display for UserStatusFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => {
                write!(f, "1")
            }
            Self::Registered => {
                write!(f, "2")
            }
            Self::Locked => {
                write!(f, "3")
            }
            Self::Any => {
                write!(f, "*")
            }
        }
    }
}

impl From<UserStatus> for UserStatusFilter {
    fn from(value: UserStatus) -> Self {
        match value {
            UserStatus::Active => Self::Active,
            UserStatus::Registered => Self::Registered,
            UserStatus::Locked => Self::Locked,
        }
    }
}

/// The endpoint for all users
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListUsers<'a> {
    /// Filter by user status
    #[builder(default)]
    status: Option<UserStatusFilter>,
    #[builder(default)]
    /// Filter by name, this matches login, firstname, lastname, mail and if it contains a space also firstname and lastname
    #[builder(setter(into))]
    name: Option<Cow<'a, str>>,
    /// Users need to be members of this group
    #[builder(default)]
    group_id: Option<u64>,
    /// Filter by authentication source
    #[builder(default)]
    auth_source_id: Option<u64>,
    /// Filter by the two-factor authentication scheme
    #[builder(default)]
    #[builder(setter(into))]
    twofa_scheme: Option<Cow<'a, str>>,
    /// A boolean filter to find only administrators
    #[builder(default)]
    admin: Option<bool>,
    /// Filter by creation time
    #[builder(default)]
    created_on: Option<DateTimeFilterPast>,
    /// Filter by last login time
    #[builder(default)]
    last_login_on: Option<DateTimeFilterPast>,
    /// Filter by login
    #[builder(default)]
    #[builder(setter(into))]
    login: Option<Cow<'a, str>>,
    /// Filter by firstname
    #[builder(default)]
    #[builder(setter(into))]
    firstname: Option<Cow<'a, str>>,
    /// Filter by lastname
    #[builder(default)]
    #[builder(setter(into))]
    lastname: Option<Cow<'a, str>>,
    /// Filter by mail
    #[builder(default)]
    #[builder(setter(into))]
    mail: Option<Cow<'a, str>>,
    /// Filter by custom fields
    #[builder(default)]
    custom_field_filters: Option<Vec<CustomFieldFilter>>,
}

impl ReturnsJsonResponse for ListUsers<'_> {}
impl Pageable for ListUsers<'_> {
    fn response_wrapper_key(&self) -> String {
        "users".to_string()
    }
}

impl<'a> ListUsers<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListUsersBuilder<'a> {
        ListUsersBuilder::default()
    }
}

impl Endpoint for ListUsers<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "users.json".into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("status", self.status.as_ref().map(|s| s.to_string()));
        params.push_opt("name", self.name.as_ref());
        params.push_opt("group_id", self.group_id);
        params.push_opt("auth_source_id", self.auth_source_id);
        params.push_opt("twofa_scheme", self.twofa_scheme.as_ref());
        params.push_opt("admin", self.admin);
        params.push_opt(
            "created_on",
            self.created_on.as_ref().map(|s| s.to_string()),
        );
        params.push_opt(
            "last_login_on",
            self.last_login_on.as_ref().map(|s| s.to_string()),
        );
        params.push_opt("login", self.login.as_ref());
        params.push_opt("firstname", self.firstname.as_ref());
        params.push_opt("lastname", self.lastname.as_ref());
        params.push_opt("mail", self.mail.as_ref());
        if let Some(filters) = self.custom_field_filters.as_ref() {
            for filter in filters {
                params.push(format!("cf_{}", filter.id), filter.value.to_string());
            }
        }
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
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetUser {
    /// User id to fetch, if not specified will fetch the current user
    #[builder(default)]
    id: Option<u64>,
    /// Include associated data
    #[builder(default)]
    include: Option<Vec<UserInclude>>,
}

impl ReturnsJsonResponse for GetUser {}
impl NoPagination for GetUser {}

impl GetUser {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetUserBuilder {
        GetUserBuilder::default()
    }
}

impl Endpoint for GetUser {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        match self.id {
            Some(id) => format!("users/{id}.json").into(),
            None => "users/current.json".into(),
        }
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("include", self.include.as_ref());
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
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
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
    #[serde(skip_serializing)]
    send_information: Option<bool>,
    /// Make the user a Redmine administrator
    #[builder(default)]
    admin: Option<bool>,
}

impl ReturnsJsonResponse for CreateUser<'_> {}
impl NoPagination for CreateUser<'_> {}

impl<'a> CreateUser<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> CreateUserBuilder<'a> {
        CreateUserBuilder::default()
    }
}

impl Endpoint for CreateUser<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "users.json".into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&UserWrapperWithSendInformation::<CreateUser> {
                user: (*self).to_owned(),
                send_information: self.send_information,
            })?,
        )))
    }
}

/// The endpoint to update an existing Redmine user
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
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
    #[builder(default, setter(into))]
    firstname: Option<Cow<'a, str>>,
    /// The user's lastname
    #[builder(default, setter(into))]
    lastname: Option<Cow<'a, str>>,
    /// The users primary email address
    #[builder(default, setter(into))]
    mail: Option<Cow<'a, str>>,
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
    #[serde(skip_serializing)]
    send_information: Option<bool>,
    /// Make the user a Redmine administrator
    #[builder(default)]
    admin: Option<bool>,
}

impl<'a> UpdateUser<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> UpdateUserBuilder<'a> {
        UpdateUserBuilder::default()
    }
}

impl Endpoint for UpdateUser<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&UserWrapperWithSendInformation::<UpdateUser> {
                user: (*self).to_owned(),
                send_information: self.send_information,
            })?,
        )))
    }
}

/// The endpoint to delete a Redmine user
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteUser {
    /// The id of the user to delete
    id: u64,
}

impl DeleteUser {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> DeleteUserBuilder {
        DeleteUserBuilder::default()
    }
}

impl Endpoint for DeleteUser {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{}.json", &self.id).into()
    }
}

/// helper struct for outer layers with a users field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct UsersWrapper<T> {
    /// to parse JSON with users key
    pub users: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a user field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct UserWrapper<T> {
    /// to parse JSON with user key
    pub user: T,
}

/// a special version of the UserWrapper to use with [CreateUser] and [UpdateUser]
/// because Redmine puts the send_information flag outside the user object for
/// some reason
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserWrapperWithSendInformation<T> {
    /// to parse JSON with user key
    pub user: T,
    /// send information flag in [CreateUser] and [UpdateUser]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_information: Option<bool>,
}

#[cfg(test)]
pub(crate) mod test {
    use crate::api::{
        ResponsePage, groups::test::GROUP_LOCK, project_memberships::test::PROJECT_MEMBERSHIP_LOCK,
        test_helpers::with_redmine, test_locking,
    };

    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tokio::sync::RwLock;
    use tracing_test::traced_test;

    /// needed so we do not get 404s when listing while
    /// creating/deleting or creating/updating/deleting
    pub static USER_LOCK: RwLock<()> = RwLock::const_new(());

    #[traced_test]
    #[test]
    fn test_list_users_first_page() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _r_user = test_locking::read_lock(&USER_LOCK);
            let endpoint = ListUsers::builder().build()?;
            redmine.json_response_body_page::<_, User>(&endpoint, 0, 25)?;
            Ok(())
        })
    }

    #[traced_test]
    #[test]
    fn test_list_users_all_pages() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _r_user = test_locking::read_lock(&USER_LOCK);
            let endpoint = ListUsers::builder().build()?;
            redmine.json_response_body_all_pages::<_, User>(&endpoint)?;
            Ok(())
        })
    }

    #[traced_test]
    #[test]
    fn test_get_user() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _r_user = test_locking::read_lock(&USER_LOCK);
            let endpoint = GetUser::builder().id(1).build()?;
            redmine.json_response_body::<_, UserWrapper<User>>(&endpoint)?;
            Ok(())
        })
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_user() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _w_user = test_locking::write_lock(&USER_LOCK);
            let name = format!("unittest_{}", function_name!());
            let list_endpoint = ListUsers::builder().name(name.clone()).build()?;
            let users: Vec<User> = redmine.json_response_body_all_pages(&list_endpoint)?;
            for user in users {
                let delete_endpoint = DeleteUser::builder().id(user.id).build()?;
                redmine.ignore_response_body::<_>(&delete_endpoint)?;
            }
            let create_endpoint = CreateUser::builder()
                .login(name.clone())
                .firstname("Unit")
                .lastname("Test")
                .mail(format!("unit-test_{name}@example.org"))
                .build()?;
            let UserWrapper { user } =
                redmine.json_response_body::<_, UserWrapper<User>>(&create_endpoint)?;
            let delete_endpoint = DeleteUser::builder().id(user.id).build()?;
            redmine.ignore_response_body::<_>(&delete_endpoint)?;
            Ok(())
        })
    }

    // this test causes emails to be sent so we comment it out, mainly it was
    // meant to check if the send_information attribute is inside or outside the
    // user object in CreateUser (the docs in the wiki say outside and that really
    // seems to be the case)
    // #[function_name::named]
    // #[traced_test]
    // #[test]
    // fn test_create_user_send_account_info() -> Result<(), Box<dyn Error>> {
    //     let _w_user = test_locking::write_lock(&USER_LOCK);
    //     let name = format!("unittest_{}", function_name!());
    //     dotenvy::dotenv()?;
    //     let redmine = crate::api::Redmine::from_env()?;
    //     let list_endpoint = ListUsers::builder().name(name.clone()).build()?;
    //     let UsersWrapper { users } =
    //         redmine.json_response_body::<_, UsersWrapper<User>>(&list_endpoint)?;
    //     for user in users {
    //         let delete_endpoint = DeleteUser::builder().id(user.id).build()?;
    //         redmine.ignore_response_body::<_>(&delete_endpoint)?;
    //     }
    //     let create_endpoint = CreateUser::builder()
    //         .login(name.clone())
    //         .firstname("Unit")
    //         .lastname("Test Send Account Info")
    //         .mail(format!("{}@example.org", name)) // apparently there is a 60 character limit on the email in Redmine
    //         .send_information(true)
    //         .build()?;
    //     let UserWrapper { user } =
    //         redmine.json_response_body::<_, UserWrapper<User>>(&create_endpoint)?;
    //     let delete_endpoint = DeleteUser::builder().id(user.id).build()?;
    //     redmine.ignore_response_body::<_>(&delete_endpoint)?;
    //     Ok(())
    // }

    // this test causes emails to be sent so we comment it out, mainly it was
    // meant to check if the admin attribute is inside or outside the user object
    // in CreateUser (the docs on the wiki say outside but inside seems
    // to be correct)
    // #[function_name::named]
    // #[traced_test]
    // #[test]
    // fn test_create_admin_user() -> Result<(), Box<dyn Error>> {
    //     let _w_user = test_locking::write_lock(&USER_LOCK);
    //     let name = format!("unittest_{}", function_name!());
    //     dotenvy::dotenv()?;
    //     let redmine = crate::api::Redmine::from_env()?;
    //     let list_endpoint = ListUsers::builder().name(name.clone()).build()?;
    //     let UsersWrapper { users } =
    //         redmine.json_response_body::<_, UsersWrapper<User>>(&list_endpoint)?;
    //     for user in users {
    //         let delete_endpoint = DeleteUser::builder().id(user.id).build()?;
    //         redmine.ignore_response_body::<_>(&delete_endpoint)?;
    //     }
    //     let create_endpoint = CreateUser::builder()
    //         .login(name.clone())
    //         .firstname("Unit")
    //         .lastname("Test Admin")
    //         .mail(format!("unit-test_{}@example.org", name))
    //         .admin(true)
    //         .build()?;
    //     let UserWrapper { user } =
    //         redmine.json_response_body::<_, UserWrapper<User>>(&create_endpoint)?;
    //     let delete_endpoint = DeleteUser::builder().id(user.id).build()?;
    //     redmine.ignore_response_body::<_>(&delete_endpoint)?;
    //     Ok(())
    // }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_update_user() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _w_user = test_locking::write_lock(&USER_LOCK);
            let name = format!("unittest_{}", function_name!());
            let list_endpoint = ListUsers::builder().name(name.clone()).build()?;
            let users: Vec<User> = redmine.json_response_body_all_pages(&list_endpoint)?;
            for user in users {
                let delete_endpoint = DeleteUser::builder().id(user.id).build()?;
                redmine.ignore_response_body::<_>(&delete_endpoint)?;
            }
            let create_endpoint = CreateUser::builder()
                .login(name.clone())
                .firstname("Unit")
                .lastname("Test")
                .mail(format!("unit-test_{name}@example.org"))
                .build()?;
            let UserWrapper { user } =
                redmine.json_response_body::<_, UserWrapper<User>>(&create_endpoint)?;
            let update_endpoint = super::UpdateUser::builder()
                .id(user.id)
                .login(format!("new_{name}"))
                .build()?;
            redmine.ignore_response_body::<_>(&update_endpoint)?;
            let delete_endpoint = DeleteUser::builder().id(user.id).build()?;
            redmine.ignore_response_body::<_>(&delete_endpoint)?;
            Ok(())
        })
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_user_type_first_page() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _r_user = test_locking::read_lock(&USER_LOCK);
            let endpoint = ListUsers::builder().build()?;
            let ResponsePage {
                values,
                total_count: _,
                offset: _,
                limit: _,
            } = redmine.json_response_body_page::<_, serde_json::Value>(&endpoint, 0, 100)?;
            for value in values {
                let o: User = serde_json::from_value(value.clone())?;
                let reserialized = serde_json::to_value(o)?;
                assert_eq!(value, reserialized);
            }
            Ok(())
        })
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    ///
    /// this version of the test will load all pages of users and the individual
    /// users for each via GetUser
    #[traced_test]
    #[test]
    fn test_completeness_user_type_all_pages_all_user_details() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        with_redmine(current_span, |redmine| {
            let _r_user = test_locking::read_lock(&USER_LOCK);
            let _r_groups = test_locking::read_lock(&GROUP_LOCK);
            let _r_project_memberships = test_locking::read_lock(&PROJECT_MEMBERSHIP_LOCK);
            let endpoint = ListUsers::builder().build()?;
            let users = redmine.json_response_body_all_pages::<_, User>(&endpoint)?;
            for user in users {
                let get_endpoint = GetUser::builder()
                    .id(user.id)
                    .include(vec![UserInclude::Memberships, UserInclude::Groups])
                    .build()?;
                let UserWrapper { user: value } = redmine
                    .json_response_body::<_, UserWrapper<serde_json::Value>>(&get_endpoint)?;
                let o: User = serde_json::from_value(value.clone())?;
                let reserialized = serde_json::to_value(o)?;
                assert_eq!(value, reserialized);
            }
            Ok(())
        })
    }
}
