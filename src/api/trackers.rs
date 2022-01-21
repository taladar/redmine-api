//! Trackers Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Trackers)
//!
//! - [x] all trackers endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;


/// The endpoint for all trackers
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Trackers {
}

impl Trackers {
    /// Create a builder for the endpoint.
    pub fn builder() -> TrackersBuilder {
        TrackersBuilder::default()
    }
}

impl<'a> Endpoint for Trackers {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "trackers.json".into()
    }
}
