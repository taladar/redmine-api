//! Versions Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Versions)
//!
//! - [x] project specific versions endpoint
//! - [x] specific version endpoint
//! - [x] create version endpoint
//! - [x] update version endpoint
//! - [x] delete version endpoint

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::projects::ProjectEssentials;
use crate::api::{Endpoint, ReturnsJsonResponse};
use serde::Serialize;

/// a minimal type for Redmine versions included in
/// other Redmine objects
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct VersionEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

impl From<Version> for VersionEssentials {
    fn from(v: Version) -> Self {
        VersionEssentials {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&Version> for VersionEssentials {
    fn from(v: &Version) -> Self {
        VersionEssentials {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

/// a type for version to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Version {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// project
    pub project: ProjectEssentials,
    /// description
    pub description: String,
    /// version status
    pub status: VersionStatus,
    /// version due date
    pub due_date: Option<time::Date>,
    /// version sharing between projects
    pub sharing: VersionSharing,
    /// The time when this version was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// The time when this version was last updated
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub updated_on: time::OffsetDateTime,
    /// The title of the wiki page for this version
    #[serde(default)]
    wiki_page_title: Option<String>,
}

/// The endpoint for all versions in a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ListVersions<'a> {
    /// The project Id or the project name as it appears in the URL for the project whose versions we want to list
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl ReturnsJsonResponse for ListVersions<'_> {}

impl<'a> ListVersions<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListVersionsBuilder<'a> {
        ListVersionsBuilder::default()
    }
}

impl Endpoint for ListVersions<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/versions.json", self.project_id_or_name).into()
    }
}

/// The endpoint for a specific Redmine project version
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct GetVersion {
    /// the id of the version to retrieve
    id: u64,
}

impl ReturnsJsonResponse for GetVersion {}

impl GetVersion {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetVersionBuilder {
        GetVersionBuilder::default()
    }
}

impl Endpoint for GetVersion {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("versions/{}.json", &self.id).into()
    }
}

/// The status of a version restricts if issues can be assigned to this
/// version and if assigned issues can be reopened
#[derive(Debug, Clone, serde::Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionStatus {
    /// no restrictions, default
    Open,
    /// can not assign new issues to the version
    Locked,
    /// can not assign new issues and can not reopen assigned issues
    Closed,
}

/// Version sharing determines the cross-project visibility of the version
#[derive(Debug, Clone, serde::Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionSharing {
    /// default
    None,
    /// only descendant projects in the hierarchy can see the project's version
    Descendants,
    /// descendant projects and ancestor projects in the hierarchy can see the project's version
    Hierarchy,
    /// descendant projects, ancestor projects and other projects in the same tree can see the project's version
    Tree,
    /// versions can be seen by all projects in the Redmine instance
    System,
}

/// The endpoint to create a Redmine project version
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateVersion<'a> {
    /// The project Id or the project name as it appears in the URL to add the version to
    #[builder(setter(into))]
    #[serde(skip_serializing)]
    project_id_or_name: Cow<'a, str>,
    /// display name
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// the status of the version
    #[builder(default)]
    status: Option<VersionStatus>,
    /// how the version is shared with other projects
    #[builder(default)]
    sharing: Option<VersionSharing>,
    /// when the version is due to be released
    #[builder(default)]
    due_date: Option<time::Date>,
    /// Description of the version
    #[builder(default)]
    description: Option<Cow<'a, str>>,
    /// The title of the wiki page for this version
    #[builder(default)]
    wiki_page_title: Option<Cow<'a, str>>,
}

impl ReturnsJsonResponse for CreateVersion<'_> {}

impl<'a> CreateVersion<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> CreateVersionBuilder<'a> {
        CreateVersionBuilder::default()
    }
}

impl Endpoint for CreateVersion<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/versions.json", self.project_id_or_name).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&VersionWrapper::<CreateVersion> {
                version: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to update an existing Redmine project version
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateVersion<'a> {
    /// The id of the version to update
    #[serde(skip_serializing)]
    id: u64,
    /// display name
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,
    /// the status of the version
    #[builder(default)]
    status: Option<VersionStatus>,
    /// how the version is shared with other projects
    #[builder(default)]
    sharing: Option<VersionSharing>,
    /// when the version is due to be released
    #[builder(default)]
    due_date: Option<time::Date>,
    /// Description of the version
    #[builder(default)]
    description: Option<Cow<'a, str>>,
    /// The title of the wiki page for this version
    #[builder(default)]
    wiki_page_title: Option<Cow<'a, str>>,
}

impl<'a> UpdateVersion<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> UpdateVersionBuilder<'a> {
        UpdateVersionBuilder::default()
    }
}

impl Endpoint for UpdateVersion<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("versions/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&VersionWrapper::<UpdateVersion> {
                version: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to delete a version in a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteVersion {
    /// The id of the version to delete
    id: u64,
}

impl DeleteVersion {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> DeleteVersionBuilder {
        DeleteVersionBuilder::default()
    }
}

impl Endpoint for DeleteVersion {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("versions/{}.json", &self.id).into()
    }
}

/// helper struct for outer layers with a versions field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct VersionsWrapper<T> {
    /// to parse JSON with versions key
    pub versions: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a version field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct VersionWrapper<T> {
    /// to parse JSON with version key
    pub version: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::test_helpers::with_project;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tokio::sync::RwLock;
    use tracing_test::traced_test;

    /// needed so we do not get 404s when listing while
    /// creating/deleting or creating/updating/deleting
    static VERSION_LOCK: RwLock<()> = RwLock::const_new(());

    #[traced_test]
    #[test]
    fn test_list_versions_no_pagination() -> Result<(), Box<dyn Error>> {
        let _r_versions = VERSION_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListVersions::builder().project_id_or_name("92").build()?;
        redmine.json_response_body::<_, VersionsWrapper<Version>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_version() -> Result<(), Box<dyn Error>> {
        let _r_versions = VERSION_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = GetVersion::builder().id(1182).build()?;
        redmine.json_response_body::<_, VersionWrapper<Version>>(&endpoint)?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_version() -> Result<(), Box<dyn Error>> {
        let _w_versions = VERSION_LOCK.write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, _, name| {
            let create_endpoint = CreateVersion::builder()
                .project_id_or_name(name)
                .name("Test Version")
                .build()?;
            redmine.json_response_body::<_, VersionWrapper<Version>>(&create_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_update_version() -> Result<(), Box<dyn Error>> {
        let _w_versions = VERSION_LOCK.write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, _, name| {
            let create_endpoint = CreateVersion::builder()
                .project_id_or_name(name)
                .name("Test Version")
                .build()?;
            let VersionWrapper { version } =
                redmine.json_response_body::<_, VersionWrapper<Version>>(&create_endpoint)?;
            let update_endpoint = super::UpdateVersion::builder()
                .id(version.id)
                .name("Neue Test-Version")
                .build()?;
            redmine.ignore_response_body::<_>(&update_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_version_type() -> Result<(), Box<dyn Error>> {
        let _r_versions = VERSION_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListVersions::builder().project_id_or_name("92").build()?;
        let VersionsWrapper { versions: values } =
            redmine.json_response_body::<_, VersionsWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: Version = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
