//! Issue Relations Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_IssueRelations)
//!
//! - [x] issue specific issue relations endpoint
//! - [ ] create issue relation endpoint
//!   - [x] normal relations
//!   - [ ] delay in precedes/follows
//! - [x] specific issue relation endpoint
//! - [x] delete issue relation endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use serde::Serialize;
use crate::api::Endpoint;

/// The endpoint for all issue relations in a Redmine issue
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct IssueRelations {
    /// the id of the issue for which we want to retrieve all issue relations
    issue_id: u64,
}

impl<'a> IssueRelations {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueRelationsBuilder {
        IssueRelationsBuilder::default()
    }
}

impl<'a> Endpoint for IssueRelations {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}/relations.json", self.issue_id).into()
    }
}

/// The endpoint for a specific issue relation
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct IssueRelation {
    /// the id of the issue relation to retrieve
    id: u64,
}

impl<'a> IssueRelation {
    /// Create a builder for the endpoint.
    pub fn builder() -> IssueRelationBuilder {
        IssueRelationBuilder::default()
    }
}

impl<'a> Endpoint for IssueRelation {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("relations/{}.json", self.id).into()
    }
}

/// Type of issue relation
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all="snake_case")]
pub enum IssueRelationType {
    /// The most general type of issue relation
    Relates,
    /// Indicates that the issue duplicates another issue
    Duplicates,
    /// Indicates that the issue is duplicated by another issue
    Duplicated,
    /// Indicates that the issue blocks another issue
    Blocks,
    /// Indicates that the issue is blocked by another issue
    Blocked,
    /// Indicates that the issue precedes another issue
    Precedes,
    /// Indicates that the issue follows another issue
    Follows,
    /// Indicates that the issue was copied to another issue
    CopiedTo,
    /// Indicates that the issue was copied from another issue
    CopiedFrom,
}

/// The endpoint to create an issue relation
#[derive(Debug, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateIssueRelation {
    /// id of the issue where the relation is created
    #[serde(skip_serializing)]
    issue_id: u64,
    /// id of the issue the relation is created to
    issue_to_id: u64,
    /// the type of issue relation to create
    relation_type: IssueRelationType,
}

impl<'a> CreateIssueRelation {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateIssueRelationBuilder {
        CreateIssueRelationBuilder::default()
    }
}

impl<'a> Endpoint for CreateIssueRelation {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}/relations.json", self.issue_id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some(("application/json", serde_json::to_vec(self)?)))
    }
}

/// The endpoint to delete an issue relation
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteIssueRelation {
    /// the id of the issue relation to delete
    id: u64,
}

impl<'a> DeleteIssueRelation {
    /// Create a builder for the endpoint.
    pub fn builder() -> CreateIssueRelationBuilder {
        CreateIssueRelationBuilder::default()
    }
}

impl<'a> Endpoint for DeleteIssueRelation {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("relations/{}.json", self.id).into()
    }
}
