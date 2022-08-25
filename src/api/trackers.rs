//! Trackers Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Trackers)
//!
//! - [x] all trackers endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::issue_statuses::IssueStatusEssentials;
use crate::api::{Endpoint, ReturnsJsonResponse};

/// a minimal type for Redmine trackers used in lists of trackers included in
/// other Redmine objects (e.g. custom fields)
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TrackerEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

impl From<Tracker> for TrackerEssentials {
    fn from(v: Tracker) -> Self {
        TrackerEssentials {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&Tracker> for TrackerEssentials {
    fn from(v: &Tracker) -> Self {
        TrackerEssentials {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

/// a type for tracker to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Tracker {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// default issue status
    pub default_status: IssueStatusEssentials,
    /// description
    pub description: Option<String>,
    /// standard fields enabled in this tracker (available in Redmine 5.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_standard_fields: Option<Vec<String>>,
}

/// The endpoint for all trackers
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ListTrackers {}

impl ReturnsJsonResponse for ListTrackers {}

impl ListTrackers {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListTrackersBuilder {
        ListTrackersBuilder::default()
    }
}

impl Endpoint for ListTrackers {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "trackers.json".into()
    }
}

/// helper struct for outer layers with a trackers field holding the inner data
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TrackersWrapper<T> {
    /// to parse JSON with trackers key
    pub trackers: Vec<T>,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_list_trackers_no_pagination() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListTrackers::builder().build()?;
        redmine.json_response_body::<_, TrackersWrapper<Tracker>>(&endpoint)?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_tracker_type() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListTrackers::builder().build()?;
        let TrackersWrapper { trackers: values } =
            redmine.json_response_body::<_, TrackersWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: Tracker = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
