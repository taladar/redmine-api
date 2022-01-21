//! Roles Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Roles)
//!
//! - [x] all roles endpoint
//! - [x] specific role endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;

/// The endpoint for all roles
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Roles {}

impl Roles {
    /// Create a builder for the endpoint.
    pub fn builder() -> RolesBuilder {
        RolesBuilder::default()
    }
}

impl<'a> Endpoint for Roles {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "roles.json".into()
    }
}

/// The endpoint for a specific role
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Role {
    /// the id of the role to retrieve
    id: u64,
}

impl Role {
    /// Create a builder for the endpoint.
    pub fn builder() -> RoleBuilder {
        RoleBuilder::default()
    }
}

impl<'a> Endpoint for Role {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("roles/{}.json", self.id).into()
    }
}
