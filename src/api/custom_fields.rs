//! Custom Fields Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_CustomFields)
//!
//! - [x] all custom fields endpoint

use derive_builder::Builder;
use reqwest::Method;
use serde::Serialize;
use std::borrow::Cow;

use crate::api::issues::RoleFilter;
use crate::api::projects::ProjectEssentials;
use crate::api::roles::RoleEssentials;
use crate::api::trackers::TrackerEssentials;
use crate::api::versions::VersionStatusFilter;
use crate::api::{Endpoint, NoPagination, ReturnsJsonResponse};

/// Represents the types of objects that can be customized with customized types
/// in Redmine
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

/// The data type or format of a custom field.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldFormat {
    /// a Redmine user
    User,
    /// a Target version
    Version,
    /// a string
    String,
    /// a text block
    Text,
    /// a link
    Link,
    /// an integer
    Int,
    /// a floating point number
    Float,
    /// a date
    Date,
    /// a list of values
    List,
    /// a boolean
    Bool,
    /// an enumeration
    Enumeration,
    /// an attachment
    Attachment,
    /// a progress bar
    Progressbar,
}

/// The style of the edit tag for a custom field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditTagStyle {
    /// Dropdown list style.
    DropDown,
    /// Checkbox style.
    CheckBox,
    /// Radio button style.
    Radio,
}

impl serde::Serialize for EditTagStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            EditTagStyle::DropDown => serializer.serialize_str(""),
            EditTagStyle::CheckBox => serializer.serialize_str("check_box"),
            EditTagStyle::Radio => serializer.serialize_str("radio"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for EditTagStyle {
    fn deserialize<D>(deserializer: D) -> Result<EditTagStyle, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "" => Ok(EditTagStyle::DropDown),
            "check_box" => Ok(EditTagStyle::CheckBox),
            "radio" => Ok(EditTagStyle::Radio),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &["", "check_box", "radio"],
            )),
        }
    }
}

/// Possible values contain a value and a label
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PossibleValue {
    /// label for the value in a select box
    pub label: String,
    /// actual value
    pub value: String,
}

/// a type for custom fields to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CustomFieldDefinition {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// description
    pub description: Option<String>,
    /// is the field editable
    pub editable: bool,
    /// type of Redmine object this field is customizing
    pub customized_type: CustomizedType,
    /// data type of the field
    pub field_format: FieldFormat,
    /// a regular expression to constrain possible string values
    pub regexp: Option<String>,
    /// a minimum length for the field
    pub min_length: Option<usize>,
    /// a maximum length for the field
    pub max_length: Option<usize>,
    /// is this field required when creating/updating an object of the customized type
    pub is_required: Option<bool>,
    /// can this field be used as a filter
    pub is_filter: Option<bool>,
    /// will this field be indexed for the search
    pub searchable: bool,
    /// can this field be added more than once
    pub multiple: bool,
    /// default value for the field
    pub default_value: Option<String>,
    /// visibility of the custom field
    pub visible: bool,
    /// which roles can see the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleEssentials>>,
    /// limit possible values to an explicit list of values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_values: Option<Vec<PossibleValue>>,
    /// this field is useable in these trackers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trackers: Option<Vec<TrackerEssentials>>,
    /// this field is useable in these projects (None means all projects)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectEssentials>>,
    /// is the custom field for all projects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_for_all: Option<bool>,
    /// position of the custom field in the list
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,
    /// url pattern for the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url_pattern: Option<String>,
    /// text formatting for the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_formatting: Option<String>,
    /// edit tag style for the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_tag_style: Option<EditTagStyle>,
    /// user role for the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_role: Option<RoleFilter>,
    /// version status for the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_status: Option<VersionStatusFilter>,
    /// extensions allowed for the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions_allowed: Option<String>,
    /// full width layout for the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_width_layout: Option<bool>,
    /// thousands delimiter for the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thousands_delimiter: Option<bool>,
    /// ratio interval for the custom field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratio_interval: Option<f32>,
}

/// a type for custom field essentials with values used in other Redmine
/// objects (e.g. issues)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomFieldEssentialsWithValue {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// if this is true the value is serialized as an array
    pub multiple: Option<bool>,
    /// value
    pub value: Option<Vec<String>>,
}

/// a type used to list all the custom field ids and names
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct CustomFieldName {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

impl serde::Serialize for CustomFieldEssentialsWithValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 2;
        if self.multiple.is_some() {
            len += 1;
        };
        if self.value.is_some() {
            len += 1;
        }
        let mut state = serializer.serialize_struct("CustomFieldEssentialsWithValue", len)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        if let Some(ref multiple) = self.multiple {
            state.serialize_field("multiple", &multiple)?;
            if let Some(ref value) = self.value {
                state.serialize_field("value", &value)?;
            } else {
                let s: Option<Vec<String>> = None;
                state.serialize_field("value", &s)?;
            }
        } else if let Some(ref value) = self.value {
            match value.as_slice() {
                [] => {
                    let s: Option<String> = None;
                    state.serialize_field("value", &s)?;
                }
                [s] => {
                    state.serialize_field("value", &s)?;
                }
                values => {
                    return Err(serde::ser::Error::custom(format!(
                        "CustomFieldEssentialsWithValue multiple was set to false but value contained more than one value: {values:?}"
                    )));
                }
            }
        } else {
            let s: Option<String> = None;
            state.serialize_field("value", &s)?;
        }
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for CustomFieldEssentialsWithValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// the fields in the CustomFieldEssentialsWithValue type
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            /// the id field
            Id,
            /// the name field
            Name,
            /// the multiple field
            Multiple,
            /// the value field
            Value,
        }

        /// visitor to deserialize CustomFieldEssentialsWithValue
        struct CustomFieldVisitor;

        impl<'de> serde::de::Visitor<'de> for CustomFieldVisitor {
            type Value = CustomFieldEssentialsWithValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CustomFieldEssentialsWithValue")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CustomFieldEssentialsWithValue, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut multiple = None;
                let mut string_value: Option<String> = None;
                let mut vec_string_value: Option<Vec<String>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Multiple => {
                            if multiple.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiple"));
                            }
                            multiple = Some(map.next_value()?);
                        }
                        Field::Value => {
                            if string_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            if vec_string_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            if let Some(true) = multiple {
                                vec_string_value = Some(map.next_value()?);
                            } else {
                                string_value = map.next_value()?;
                            }
                        }
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                match (multiple, string_value, vec_string_value) {
                    (None, None, None) => Ok(CustomFieldEssentialsWithValue {
                        id,
                        name,
                        multiple: None,
                        value: None,
                    }),
                    (None, Some(s), None) => Ok(CustomFieldEssentialsWithValue {
                        id,
                        name,
                        multiple: None,
                        value: Some(vec![s]),
                    }),
                    (Some(true), None, Some(v)) => Ok(CustomFieldEssentialsWithValue {
                        id,
                        name,
                        multiple: Some(true),
                        value: Some(v),
                    }),
                    _ => Err(serde::de::Error::custom(
                        "invalid combination of multiple and value",
                    )),
                }
            }
        }

        /// list of fields of CustomFieldEssentialsWithValue to pass to deserialize_struct
        const FIELDS: &[&str] = &["id", "name", "multiple", "value"];
        deserializer.deserialize_struct(
            "CustomFieldEssentialsWithValue",
            FIELDS,
            CustomFieldVisitor,
        )
    }
}

/// The endpoint for all custom fields
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListCustomFields {}

impl ReturnsJsonResponse for ListCustomFields {}
impl NoPagination for ListCustomFields {}

impl ListCustomFields {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListCustomFieldsBuilder {
        ListCustomFieldsBuilder::default()
    }
}

impl Endpoint for ListCustomFields {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "custom_fields.json".into()
    }
}

/// a custom field
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct CustomField<'a> {
    /// the custom field's id
    pub id: u64,
    /// is usually present in contexts where it is returned by Redmine but can be omitted when it is sent by the client
    pub name: Option<Cow<'a, str>>,
    /// the custom field's value
    pub value: Cow<'a, str>,
}

/// helper struct for outer layers with a custom_fields field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CustomFieldsWrapper<T> {
    /// to parse JSON with custom_fields key
    pub custom_fields: Vec<T>,
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
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;
        let endpoint = ListCustomFields::builder().build()?;
        redmine.json_response_body::<_, CustomFieldsWrapper<CustomFieldDefinition>>(&endpoint)?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_custom_fields_type() -> Result<(), Box<dyn Error>> {
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;
        let endpoint = ListCustomFields::builder().build()?;
        let CustomFieldsWrapper {
            custom_fields: values,
        } = redmine.json_response_body::<_, CustomFieldsWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: CustomFieldDefinition = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
