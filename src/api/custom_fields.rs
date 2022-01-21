//! Custom Fields Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_CustomFields)
//!
//! - [x] all custom fields endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;


/// The endpoint for all custom fields
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct CustomFields {
}

impl CustomFields {
    /// Create a builder for the endpoint.
    pub fn builder() -> CustomFieldsBuilder {
        CustomFieldsBuilder::default()
    }
}

impl<'a> Endpoint for CustomFields {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "trackers.json".into()
    }
}
