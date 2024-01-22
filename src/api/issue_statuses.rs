//! Issue Statuses Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_IssueStatuses)
//!
//! - [x] all issue statuses endpoint

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::{Endpoint, ReturnsJsonResponse};

/// a minimal type for Redmine issue status used in
/// other Redmine objects (e.g. issue)
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IssueStatusEssentials {
    /// numeric id
    pub id: u64,
    /// is this status consided closed, only included in recent Redmine versions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    /// display name
    pub name: String,
}

impl From<IssueStatus> for IssueStatusEssentials {
    fn from(v: IssueStatus) -> Self {
        IssueStatusEssentials {
            id: v.id,
            is_closed: Some(v.is_closed),
            name: v.name,
        }
    }
}

impl From<&IssueStatus> for IssueStatusEssentials {
    fn from(v: &IssueStatus) -> Self {
        IssueStatusEssentials {
            id: v.id,
            is_closed: Some(v.is_closed),
            name: v.name.to_owned(),
        }
    }
}

/// a type for issue status to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IssueStatus {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// is this status considered closed
    pub is_closed: bool,
}

/// The endpoint for all issue statuses
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ListIssueStatuses {}

impl ReturnsJsonResponse for ListIssueStatuses {}

impl ListIssueStatuses {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListIssueStatusesBuilder {
        ListIssueStatusesBuilder::default()
    }
}

impl Endpoint for ListIssueStatuses {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "issue_statuses.json".into()
    }
}

/// helper struct for outer layers with a issue_statuses field holding the inner data
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IssueStatusesWrapper<T> {
    /// to parse JSON with issue_statuses key
    pub issue_statuses: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a issue_status field holding the inner data
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IssueStatusWrapper<T> {
    /// to parse JSON with an issue_status key
    pub issue_status: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_list_issue_statuses_no_pagination() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListIssueStatuses::builder().build()?;
        redmine.json_response_body::<_, IssueStatusesWrapper<IssueStatus>>(&endpoint)?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_issue_status_type() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListIssueStatuses::builder().build()?;
        let IssueStatusesWrapper {
            issue_statuses: values,
        } = redmine.json_response_body::<_, IssueStatusesWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: IssueStatus = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
