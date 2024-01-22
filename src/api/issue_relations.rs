//! Issue Relations Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_IssueRelations)
//!
//! - [x] issue specific issue relations endpoint
//! - [x] create issue relation endpoint
//!   - [x] normal relations
//!   - [x] delay in precedes/follows
//! - [x] specific issue relation endpoint
//! - [x] delete issue relation endpoint

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::{Endpoint, ReturnsJsonResponse};
use serde::Serialize;

/// a type for issue relations to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct IssueRelation {
    /// numeric id
    pub id: u64,
    /// issue on which this relation is created
    pub issue_id: u64,
    /// issue to which it is related
    pub issue_to_id: u64,
    /// type of relation
    pub relation_type: IssueRelationType,
    /// Delay in days for the precedes and follows relation types
    pub delay: Option<u64>,
}

/// The endpoint for all issue relations in a Redmine issue
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ListIssueRelations {
    /// the id of the issue for which we want to retrieve all issue relations
    issue_id: u64,
}

impl ReturnsJsonResponse for ListIssueRelations {}

impl ListIssueRelations {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListIssueRelationsBuilder {
        ListIssueRelationsBuilder::default()
    }
}

impl Endpoint for ListIssueRelations {
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
pub struct GetIssueRelation {
    /// the id of the issue relation to retrieve
    id: u64,
}

impl ReturnsJsonResponse for GetIssueRelation {}

impl GetIssueRelation {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetIssueRelationBuilder {
        GetIssueRelationBuilder::default()
    }
}

impl Endpoint for GetIssueRelation {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("relations/{}.json", self.id).into()
    }
}

/// Type of issue relation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
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
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateIssueRelation {
    /// id of the issue where the relation is created
    #[serde(skip_serializing)]
    issue_id: u64,
    /// id of the issue the relation is created to
    issue_to_id: u64,
    /// the type of issue relation to create
    relation_type: IssueRelationType,
    /// Delay in days for the precedes and follows relation types
    #[builder(default)]
    delay: Option<u64>,
}

impl ReturnsJsonResponse for CreateIssueRelation {}

impl CreateIssueRelation {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> CreateIssueRelationBuilder {
        CreateIssueRelationBuilder::default()
    }
}

impl Endpoint for CreateIssueRelation {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issues/{}/relations.json", self.issue_id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&RelationWrapper::<CreateIssueRelation> {
                relation: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to delete an issue relation
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteIssueRelation {
    /// the id of the issue relation to delete
    id: u64,
}

impl DeleteIssueRelation {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> DeleteIssueRelationBuilder {
        DeleteIssueRelationBuilder::default()
    }
}

impl Endpoint for DeleteIssueRelation {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("relations/{}.json", self.id).into()
    }
}

/// helper struct for outer layers with a relations field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct RelationsWrapper<T> {
    /// to parse JSON with relations key
    pub relations: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a relation field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct RelationWrapper<T> {
    /// to parse JSON with an relation key
    pub relation: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::issues::test::ISSUES_LOCK;
    use crate::api::issues::{CreateIssue, Issue, IssueWrapper};
    use crate::api::test_helpers::with_project;
    use parking_lot::{const_rwlock, RwLock};
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    /// needed so we do not get 404s when listing while
    /// creating/deleting or creating/updating/deleting
    static ISSUE_RELATION_LOCK: RwLock<()> = const_rwlock(());

    #[traced_test]
    #[test]
    fn test_list_issue_relations_no_pagination() -> Result<(), Box<dyn Error>> {
        let _r_issue_relation = ISSUE_RELATION_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListIssueRelations::builder().issue_id(50017).build()?;
        redmine.json_response_body::<_, RelationsWrapper<IssueRelation>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_issue_relation() -> Result<(), Box<dyn Error>> {
        let _r_issue_relation = ISSUE_RELATION_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = GetIssueRelation::builder().id(10).build()?;
        redmine.json_response_body::<_, RelationWrapper<IssueRelation>>(&endpoint)?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_issue_relation() -> Result<(), Box<dyn Error>> {
        let _w_issues = ISSUES_LOCK.write();
        let _w_issue_relation = ISSUE_RELATION_LOCK.write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, project_id, _name| {
            let create_issue1_endpoint = CreateIssue::builder()
                .project_id(project_id)
                .subject("Test issue 1")
                .build()?;
            let IssueWrapper { issue: issue1 }: IssueWrapper<Issue> =
                redmine.json_response_body::<_, _>(&create_issue1_endpoint)?;
            let create_issue2_endpoint = CreateIssue::builder()
                .project_id(project_id)
                .subject("Test issue 2")
                .build()?;
            let IssueWrapper { issue: issue2 }: IssueWrapper<Issue> =
                redmine.json_response_body::<_, _>(&create_issue2_endpoint)?;
            let create_endpoint = super::CreateIssueRelation::builder()
                .issue_id(issue1.id)
                .issue_to_id(issue2.id)
                .relation_type(IssueRelationType::Relates)
                .build()?;
            redmine.json_response_body::<_, RelationWrapper<IssueRelation>>(&create_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_delete_issue_relation() -> Result<(), Box<dyn Error>> {
        let _w_issues = ISSUES_LOCK.write();
        let _w_issue_relation = ISSUE_RELATION_LOCK.write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, project_id, _name| {
            let create_issue1_endpoint = CreateIssue::builder()
                .project_id(project_id)
                .subject("Test issue 1")
                .build()?;
            let IssueWrapper { issue: issue1 }: IssueWrapper<Issue> =
                redmine.json_response_body::<_, _>(&create_issue1_endpoint)?;
            let create_issue2_endpoint = CreateIssue::builder()
                .project_id(project_id)
                .subject("Test issue 2")
                .build()?;
            let IssueWrapper { issue: issue2 }: IssueWrapper<Issue> =
                redmine.json_response_body::<_, _>(&create_issue2_endpoint)?;
            let create_endpoint = super::CreateIssueRelation::builder()
                .issue_id(issue1.id)
                .issue_to_id(issue2.id)
                .relation_type(IssueRelationType::Relates)
                .build()?;
            let RelationWrapper { relation }: RelationWrapper<IssueRelation> =
                redmine.json_response_body::<_, _>(&create_endpoint)?;
            let id = relation.id;
            let delete_endpoint = super::DeleteIssueRelation::builder().id(id).build()?;
            redmine.ignore_response_body::<_>(&delete_endpoint)?;
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
    fn test_completeness_issue_relation_type() -> Result<(), Box<dyn Error>> {
        let _r_issue_relation = ISSUE_RELATION_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListIssueRelations::builder().issue_id(50017).build()?;
        let RelationsWrapper { relations: values } =
            redmine.json_response_body::<_, RelationsWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: IssueRelation = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
