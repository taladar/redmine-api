//! Issue Categories Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_IssueCategories)
//!
//! - [x] project specific issue categories endpoint
//! - [x] specific issue category endpoint
//! - [x] create issue category endpoint
//! - [x] update issue category endpoint
//! - [x] delete issue category endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use serde::Serialize;
use crate::api::Endpoint;

/// The endpoint for all issue categories in a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct IssueCategories<'a> {
    /// the project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl<'a> IssueCategories<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueCategoriesBuilder<'a> {
        IssueCategoriesBuilder::default()
    }
}

impl<'a> Endpoint for IssueCategories<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/issue_categories.json", self.project_id_or_name).into()
    }
}

/// The endpoint for a specific issue category
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct IssueCategory {
    /// the id of the issue category to retrieve
    id: u64,
}

impl<'a> IssueCategory {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueCategoryBuilder {
        IssueCategoryBuilder::default()
    }
}

impl<'a> Endpoint for IssueCategory {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
       format!("issue_categories/{}.json", &self.id).into()
    }
}

/// The endpoint to create a Redmine issue category
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateIssueCategory<'a> {
    /// project id or name as it appears in the URL for the project where we want to create the new issue category
    #[serde(skip_serializing)]
    project_id_or_name: u64,
    /// the name of the new issue category
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// Issues in this issue category are assigned to this user by default
    #[builder(default)]
    assigned_to_id: Option<u64>,
}

impl<'a> CreateIssueCategory<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateIssueCategoryBuilder<'a> {
        CreateIssueCategoryBuilder::default()
    }
}

impl<'a> Endpoint for CreateIssueCategory<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}/issue_categories.json", self.project_id_or_name).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to update an existing Redmine issue category
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateIssueCategory<'a> {
    /// the id of the issue category to update
    #[serde(skip_serializing)]
    id: u64,
    /// the name of the issue category
    #[builder(setter(into), default)]
    name: Option<Cow<'a, str>>,
    /// Issues in this issue category are assigned to this user by default
    #[builder(default)]
    assigned_to_id: Option<u64>,
}

impl<'a> UpdateIssueCategory<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> UpdateIssueCategoryBuilder<'a> {
        UpdateIssueCategoryBuilder::default()
    }
}

impl<'a> Endpoint for UpdateIssueCategory<'a> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issue_categories/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to delete a Redmine issue category
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteIssueCategory {
    /// the id of the issue category to delete
    id: u64,
}

impl DeleteIssueCategory {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteIssueCategoryBuilder {
        DeleteIssueCategoryBuilder::default()
    }
}

impl<'a> Endpoint for DeleteIssueCategory {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
       format!("issue_categories/{}.json", &self.id).into()
    }
}
