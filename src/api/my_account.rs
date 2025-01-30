//! My Account Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_MyAccount)
//!
//! - [x] my account endpoint

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::custom_fields::CustomFieldEssentialsWithValue;
use crate::api::{Endpoint, ReturnsJsonResponse};

/// a type for my account to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MyAccount {
    /// numeric id
    pub id: u64,
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
    /// The time when this user was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// the time when this user last logged in
    #[serde(
        serialize_with = "crate::api::serialize_optional_rfc3339",
        deserialize_with = "crate::api::deserialize_optional_rfc3339"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_on: Option<time::OffsetDateTime>,
    /// the user's API key
    pub api_key: String,
    /// custom fields with values
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldEssentialsWithValue>>,
}

/// The endpoint to retrieve the current user's my account settings/data
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetMyAccount {}

impl ReturnsJsonResponse for GetMyAccount {}

impl GetMyAccount {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetMyAccountBuilder {
        GetMyAccountBuilder::default()
    }
}

impl Endpoint for GetMyAccount {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "my/account.json".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::users::UserWrapper;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_get_my_account() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = GetMyAccount::builder().build()?;
        redmine.json_response_body::<_, UserWrapper<MyAccount>>(&endpoint)?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_my_account_type() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = GetMyAccount::builder().build()?;
        let UserWrapper { user: value } =
            redmine.json_response_body::<_, UserWrapper<serde_json::Value>>(&endpoint)?;
        let o: MyAccount = serde_json::from_value(value.clone())?;
        let reserialized = serde_json::to_value(o)?;
        assert_eq!(value, reserialized);
        Ok(())
    }
}
