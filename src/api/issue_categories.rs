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
use reqwest::Method;
use std::borrow::Cow;

use crate::api::issues::AssigneeEssentials;
use crate::api::projects::ProjectEssentials;
use crate::api::{Endpoint, ReturnsJsonResponse};
use serde::Serialize;

/// a minimal type for Redmine issue categories used in
/// other Redmine objects (e.g. issue)
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IssueCategoryEssentials {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
}

impl From<IssueCategory> for IssueCategoryEssentials {
    fn from(v: IssueCategory) -> Self {
        IssueCategoryEssentials {
            id: v.id,
            name: v.name,
        }
    }
}

impl From<&IssueCategory> for IssueCategoryEssentials {
    fn from(v: &IssueCategory) -> Self {
        IssueCategoryEssentials {
            id: v.id,
            name: v.name.to_owned(),
        }
    }
}

/// a type for issue categories to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct IssueCategory {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// project
    pub project: ProjectEssentials,
    /// issues in this category are assigned to this user or group by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<AssigneeEssentials>,
}

/// The endpoint for all issue categories in a Redmine project
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ListIssueCategories<'a> {
    /// the project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl ReturnsJsonResponse for ListIssueCategories<'_> {}

impl<'a> ListIssueCategories<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListIssueCategoriesBuilder<'a> {
        ListIssueCategoriesBuilder::default()
    }
}

impl Endpoint for ListIssueCategories<'_> {
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
pub struct GetIssueCategory {
    /// the id of the issue category to retrieve
    id: u64,
}

impl ReturnsJsonResponse for GetIssueCategory {}

impl GetIssueCategory {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetIssueCategoryBuilder {
        GetIssueCategoryBuilder::default()
    }
}

impl Endpoint for GetIssueCategory {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issue_categories/{}.json", &self.id).into()
    }
}

/// The endpoint to create a Redmine issue category
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateIssueCategory<'a> {
    /// project id or name as it appears in the URL for the project where we want to create the new issue category
    #[serde(skip_serializing)]
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// the name of the new issue category
    #[builder(setter(into))]
    name: Cow<'a, str>,
    /// Issues in this issue category are assigned to this user by default
    #[builder(default)]
    assigned_to_id: Option<u64>,
}

impl ReturnsJsonResponse for CreateIssueCategory<'_> {}

impl<'a> CreateIssueCategory<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> CreateIssueCategoryBuilder<'a> {
        CreateIssueCategoryBuilder::default()
    }
}

impl Endpoint for CreateIssueCategory<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/issue_categories.json", self.project_id_or_name).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&IssueCategoryWrapper::<CreateIssueCategory> {
                issue_category: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to update an existing Redmine issue category
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Builder, Serialize)]
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
    #[must_use]
    pub fn builder() -> UpdateIssueCategoryBuilder<'a> {
        UpdateIssueCategoryBuilder::default()
    }
}

impl Endpoint for UpdateIssueCategory<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issue_categories/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&IssueCategoryWrapper::<UpdateIssueCategory> {
                issue_category: (*self).to_owned(),
            })?,
        )))
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
    #[must_use]
    pub fn builder() -> DeleteIssueCategoryBuilder {
        DeleteIssueCategoryBuilder::default()
    }
}

impl Endpoint for DeleteIssueCategory {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("issue_categories/{}.json", &self.id).into()
    }
}

/// helper struct for outer layers with a issue_categories field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct IssueCategoriesWrapper<T> {
    /// to parse JSON with issue_categories key
    pub issue_categories: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a issue_category field holding the inner data
#[derive(Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct IssueCategoryWrapper<T> {
    /// to parse JSON with an issue_category key
    pub issue_category: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::test_helpers::with_project;
    use parking_lot::{const_rwlock, RwLock};
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    /// needed so we do not get 404s when listing while
    /// creating/deleting or creating/updating/deleting
    static ISSUE_CATEGORY_LOCK: RwLock<()> = const_rwlock(());

    #[traced_test]
    #[test]
    fn test_list_issue_categories_no_pagination() -> Result<(), Box<dyn Error>> {
        let _r_issue_category = ISSUE_CATEGORY_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListIssueCategories::builder()
            .project_id_or_name("336")
            .build()?;
        redmine.json_response_body::<_, IssueCategoriesWrapper<IssueCategory>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_issue_category() -> Result<(), Box<dyn Error>> {
        let _r_issue_category = ISSUE_CATEGORY_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = GetIssueCategory::builder().id(10).build()?;
        redmine.json_response_body::<_, IssueCategoryWrapper<IssueCategory>>(&endpoint)?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_issue_category() -> Result<(), Box<dyn Error>> {
        let _w_issue_category = ISSUE_CATEGORY_LOCK.write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, _id, name| {
            let create_endpoint = super::CreateIssueCategory::builder()
                .project_id_or_name(name)
                .name("Unittest Issue Category")
                .build()?;
            redmine.ignore_response_body::<_>(&create_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_update_issue_category() -> Result<(), Box<dyn Error>> {
        let _w_issue_category = ISSUE_CATEGORY_LOCK.write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, _id, name| {
            let create_endpoint = super::CreateIssueCategory::builder()
                .project_id_or_name(name)
                .name("Unittest Issue Category")
                .build()?;
            let IssueCategoryWrapper { issue_category }: IssueCategoryWrapper<IssueCategory> =
                redmine.json_response_body::<_, _>(&create_endpoint)?;
            let id = issue_category.id;
            let update_endpoint = super::UpdateIssueCategory::builder()
                .id(id)
                .name("Renamed Unit-Test name")
                .build()?;
            redmine.ignore_response_body::<_>(&update_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_delete_issue_category() -> Result<(), Box<dyn Error>> {
        let _w_issue_category = ISSUE_CATEGORY_LOCK.write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, _id, name| {
            let create_endpoint = super::CreateIssueCategory::builder()
                .project_id_or_name(name)
                .name("Unittest Issue Category")
                .build()?;
            let IssueCategoryWrapper { issue_category }: IssueCategoryWrapper<IssueCategory> =
                redmine.json_response_body::<_, _>(&create_endpoint)?;
            let id = issue_category.id;
            let delete_endpoint = super::DeleteIssueCategory::builder().id(id).build()?;
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
    fn test_completeness_issue_category_type() -> Result<(), Box<dyn Error>> {
        let _r_issue_category = ISSUE_CATEGORY_LOCK.read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListIssueCategories::builder()
            .project_id_or_name("336")
            .build()?;
        let IssueCategoriesWrapper {
            issue_categories: values,
        } = redmine
            .json_response_body::<_, IssueCategoriesWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: IssueCategory = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
