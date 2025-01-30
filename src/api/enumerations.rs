//! Enumerations Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Enumerations)
//!
//! - [x] all issue priorities endpoint
//! - [x] all time entry activities endpoint
//! - [x] all document categories endpoint

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::{Endpoint, ReturnsJsonResponse};

/// a minimal type for Redmine issue priorities included in
/// other Redmine objects
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IssuePriorityEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

impl From<IssuePriority> for IssuePriorityEssentials {
    fn from(v: IssuePriority) -> Self {
        IssuePriorityEssentials {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&IssuePriority> for IssuePriorityEssentials {
    fn from(v: &IssuePriority) -> Self {
        IssuePriorityEssentials {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

/// a type for issue priority to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IssuePriority {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// whether this value is the default value
    pub is_default: bool,
}

/// The endpoint for all issue priorities
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListIssuePriorities {}

impl ReturnsJsonResponse for ListIssuePriorities {}

impl ListIssuePriorities {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListIssuePrioritiesBuilder {
        ListIssuePrioritiesBuilder::default()
    }
}

impl Endpoint for ListIssuePriorities {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "enumerations/issue_priorities.json".into()
    }
}

/// helper struct for outer layers with a issue_priorities field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IssuePrioritiesWrapper<T> {
    /// to parse JSON with issue_priorities key
    pub issue_priorities: Vec<T>,
}

/// a minimal type for Redmine time entry activities included in
/// other Redmine objects
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TimeEntryActivityEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

impl From<TimeEntryActivity> for TimeEntryActivityEssentials {
    fn from(v: TimeEntryActivity) -> Self {
        TimeEntryActivityEssentials {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&TimeEntryActivity> for TimeEntryActivityEssentials {
    fn from(v: &TimeEntryActivity) -> Self {
        TimeEntryActivityEssentials {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

/// a type for time entry activity to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TimeEntryActivity {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// whether this value is the default value
    pub is_default: bool,
}

/// The endpoint for all time entry activities
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListTimeEntryActivities {}

impl ReturnsJsonResponse for ListTimeEntryActivities {}

impl ListTimeEntryActivities {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListTimeEntryActivitiesBuilder {
        ListTimeEntryActivitiesBuilder::default()
    }
}

impl Endpoint for ListTimeEntryActivities {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "enumerations/time_entry_activities.json".into()
    }
}

/// helper struct for outer layers with a time_entry_activities field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TimeEntryActivitiesWrapper<T> {
    /// to parse JSON with time_entry_activities key
    pub time_entry_activities: Vec<T>,
}

/// a minimal type for Redmine document categories included in
/// other Redmine objects
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DocumentCategoryEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

impl From<DocumentCategory> for DocumentCategoryEssentials {
    fn from(v: DocumentCategory) -> Self {
        DocumentCategoryEssentials {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&DocumentCategory> for DocumentCategoryEssentials {
    fn from(v: &DocumentCategory) -> Self {
        DocumentCategoryEssentials {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

/// a type for document category to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DocumentCategory {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// whether this value is the default value
    pub is_default: bool,
}

/// The endpoint for all document categories
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListDocumentCategories {}

impl ReturnsJsonResponse for ListDocumentCategories {}

impl ListDocumentCategories {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListDocumentCategoriesBuilder {
        ListDocumentCategoriesBuilder::default()
    }
}

impl Endpoint for ListDocumentCategories {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "enumerations/document_categories.json".into()
    }
}

/// helper struct for outer layers with a document_categories field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DocumentCategoriesWrapper<T> {
    /// to parse JSON with document_categories key
    pub document_categories: Vec<T>,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_list_issue_priorities_no_pagination() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListIssuePriorities::builder().build()?;
        redmine.json_response_body::<_, IssuePrioritiesWrapper<IssuePriority>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_time_entry_activities_no_pagination() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListTimeEntryActivities::builder().build()?;
        redmine
            .json_response_body::<_, TimeEntryActivitiesWrapper<TimeEntryActivity>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_document_categories_no_pagination() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListDocumentCategories::builder().build()?;
        redmine.json_response_body::<_, DocumentCategoriesWrapper<DocumentCategory>>(&endpoint)?;
        Ok(())
    }
}
