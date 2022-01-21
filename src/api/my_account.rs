//! My Account Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_MyAccount)
//!
//! - [x] my account endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;

/// The endpoint to retrieve the current user's my account settings/data
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct MyAccount {}

impl MyAccount {
    /// Create a builder for the endpoint.
    pub fn builder() -> MyAccountBuilder {
        MyAccountBuilder::default()
    }
}

impl<'a> Endpoint for MyAccount {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "my/account.json".into()
    }
}
