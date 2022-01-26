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
use http::Method;
use std::borrow::Cow;

use crate::api::Endpoint;
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

/// The endpoint for all versions in a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Versions<'a> {
    /// The project Id or the project name as it appears in the URL for the project whose versions we want to list
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl<'a> Versions<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> VersionsBuilder<'a> {
        VersionsBuilder::default()
    }
}

impl<'a> Endpoint for Versions<'a> {
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
pub struct Version {
    /// the id of the version to retrieve
    id: u64,
}

impl<'a> Version {
    /// Create a builder for the endpoint.
    pub fn builder() -> VersionBuilder {
        VersionBuilder::default()
    }
}

impl<'a> Endpoint for Version {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("versions/{}.json", &self.id).into()
    }
}

/// The status of a version restricts if issues can be assigned to this
/// version and if assigned issues can be reopened
#[derive(Debug, Clone, Serialize)]
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
#[derive(Debug, Clone, Serialize)]
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
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateVersion<'a> {
    /// The project Id or the project name as it appears in the URL to add the version to
    #[builder(setter(into))]
    #[serde(skip_serializing)]
    project_id_or_name: Cow<'a, str>,
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

impl<'a> CreateVersion<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateVersionBuilder<'a> {
        CreateVersionBuilder::default()
    }
}

impl<'a> Endpoint for CreateVersion<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/versions.json", self.project_id_or_name).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to update an existing Redmine project version
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateVersion<'a> {
    /// The id of the version to update
    #[serde(skip_serializing)]
    id: u64,
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
    pub fn builder() -> UpdateVersionBuilder<'a> {
        UpdateVersionBuilder::default()
    }
}

impl<'a> Endpoint for UpdateVersion<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("versions/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to delete a versionp in a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteVersion {
    /// The id of the version to delete
    id: u64,
}

impl DeleteVersion {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteVersionBuilder {
        DeleteVersionBuilder::default()
    }
}

impl<'a> Endpoint for DeleteVersion {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("versions/{}.json", &self.id).into()
    }
}
