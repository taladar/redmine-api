//! Trackers Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Trackers)
//!
//! - [x] all trackers endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;

/// a minimal type for Redmine trackers used in lists of trackers included in
/// other Redmine objects (e.g. custom fields)
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TrackerEssentials {
    /// numeric id
    id: u64,
    /// display name
    name: String,
}

/// The endpoint for all trackers
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Trackers {}

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
