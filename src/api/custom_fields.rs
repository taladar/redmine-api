//! Custom Fields Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_CustomFields)
//!
//! - [x] all custom fields endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::projects::ProjectEssentials;
use crate::api::roles::RoleEssentials;
use crate::api::trackers::TrackerEssentials;
use crate::api::{Endpoint, ReturnsJsonResponse};

/// Represents the types of objects that can be customized with customized types
/// in Redmine
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomizedType {
    /// Redmine Issues
    Issue,
    /// Redmine Time Entries
    TimeEntry,
    /// Redmine Projects
    Project,
    /// Redmine Target Versions
    Version,
    /// Redmine Users
    User,
    /// Redmine Groups
    Group,
    /// Redmine Activities (in time tracking)
    Activity,
    /// Redmine Issue Priorities
    IssuePriority,
    /// Redmine Document Categories
    DocumentCategory,
}

/// Describes the format (data type) of a field
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldFormat {
    /// true or false
    Bool,
    /// a calendar date
    Date,
    /// an uploaded file
    File,
    /// a floating point number
    Float,
    /// a whole number
    Integer,
    /// a list of key/value pairs
    KeyValueList,
    /// a hyperlink
    Link,
    /// a list of strings
    List,
    /// a long text (multi-line)
    Text,
    /// a short text
    String,
    /// a Redmine user
    User,
    /// a Target version
    Version,
}

/// Possible values contain a value and a label
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PossibleValue {
    /// label for the value in a select box
    label: String,
    /// actual value
    value: String,
}

/// a type for custom fields to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CustomField {
    /// numeric id
    id: u64,
    /// display name
    name: String,
    /// type of Redmine object this field is customizing
    customized_type: CustomizedType,
    /// data type of the field
    field_format: FieldFormat,
    /// a regular expression to constrain possible string values
    regexp: Option<String>,
    /// a minimum length for the field
    min_length: Option<usize>,
    /// a maximum length for the field
    max_length: Option<usize>,
    /// is this field required when creating/updating an object of the customized type
    is_required: Option<bool>,
    /// can this field be used as a filter
    is_filter: Option<bool>,
    /// will this field be indexed for the search
    searchable: bool,
    /// can this field be added more than once
    multiple: bool,
    /// default value for the field
    default_value: Option<String>,
    /// visibility of the custom field
    visible: bool,
    /// which roles can see the custom field
    roles: Vec<RoleEssentials>,
    /// limit possible values to an explicit list of values
    #[serde(skip_serializing_if = "Option::is_none")]
    possible_values: Option<Vec<PossibleValue>>,
    /// this field is useable in these trackers
    trackers: Vec<TrackerEssentials>,
    /// this field is useable in these projects (None means all projects)
    #[serde(skip_serializing_if = "Option::is_none")]
    projects: Option<Vec<ProjectEssentials>>,
}

/// The endpoint for all custom fields
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ListCustomFields {}

impl ReturnsJsonResponse for ListCustomFields {}

impl ListCustomFields {
    /// Create a builder for the endpoint.
    pub fn builder() -> ListCustomFieldsBuilder {
        ListCustomFieldsBuilder::default()
    }
}

impl<'a> Endpoint for ListCustomFields {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "custom_fields.json".into()
    }
}

/// helper struct for outer layers with a custom_fields field holding the inner data
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CustomFieldsWrapper<T> {
    /// to parse JSON with custom_fields key
    custom_fields: Vec<T>,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_list_custom_fields_no_pagination() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListCustomFields::builder().build()?;
        redmine.json_response_body::<_, CustomFieldsWrapper<CustomField>>(&endpoint)?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_custom_fields_type() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListCustomFields::builder().build()?;
        let CustomFieldsWrapper {
            custom_fields: values,
        } = redmine.json_response_body::<_, CustomFieldsWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: CustomField = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
