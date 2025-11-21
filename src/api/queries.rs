//! Queries Rest API Endpoint definitions
//!
//! The Redmine API for queries is read-only. It only supports listing all visible queries.
//! The API does not expose endpoints for:
//! - Retrieving a single query by ID (GET /queries/:id)
//! - Creating a query (POST /queries.json)
//! - Updating a query (PUT /queries/:id.json)
//! - Deleting a query (DELETE /queries/:id.json)
//!
//! This was confirmed by examining the Redmine source code (config/routes.rb and app/controllers/queries_controller.rb).
//! Previous analysis indicating inconsistencies due to missing client implementations for these endpoints was a false positive.
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Queries)
//!
//! - [x] all (visible) custom queries endpoint

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::{Endpoint, Pageable, ReturnsJsonResponse};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// The visibility of a query
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum Visibility {
    /// private
    Private = 0,
    /// visible to roles
    Roles = 1,
    /// visible to all users
    Public = 2,
}

/// a type for query list items to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QueryListItem {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// is the query public
    pub is_public: bool,
    /// the project for project-specific queries
    pub project_id: Option<u64>,
}

/// a type for a detailed query to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Query {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// is the query public
    pub is_public: bool,
    /// the project for project-specific queries
    pub project_id: Option<u64>,
    /// the user who created the query
    pub user_id: u64,
    /// a description of the query
    #[serde(default)]
    pub description: Option<String>,
    /// the filters for the query
    pub filters: serde_json::Value,
    /// the column names for the query
    pub column_names: Vec<String>,
    /// the sort criteria for the query
    pub sort_criteria: serde_json::Value,
    /// the options for the query
    pub options: serde_json::Value,
    /// the type of the query
    #[serde(rename = "type")]
    pub query_type: String,
}

/// The endpoint to retrieve all queries visible to the current user
///
/// to actually use them pass the query_id to the ListIssues endpoint
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListQueries {}

impl ReturnsJsonResponse for ListQueries {}
impl Pageable for ListQueries {
    fn response_wrapper_key(&self) -> String {
        "queries".to_string()
    }
}

impl ListQueries {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListQueriesBuilder {
        ListQueriesBuilder::default()
    }
}

impl Endpoint for ListQueries {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "queries.json".into()
    }
}

/// helper struct for outer layers with a queries field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QueriesWrapper<T> {
    /// to parse JSON with queries key
    pub queries: Vec<T>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::test_helpers::with_redmine;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_list_queries_first_page() -> Result<(), Box<dyn Error>> {
        with_redmine(|redmine| {
            let endpoint = ListQueries::builder().build()?;
            redmine.json_response_body_page::<_, QueryListItem>(&endpoint, 0, 25)?;
            Ok(())
        })
    }

    #[traced_test]
    #[test]
    fn test_list_queries_all_pages() -> Result<(), Box<dyn Error>> {
        with_redmine(|redmine| {
            let endpoint = ListQueries::builder().build()?;
            redmine.json_response_body_all_pages::<_, QueryListItem>(&endpoint)?;
            Ok(())
        })
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_query_type() -> Result<(), Box<dyn Error>> {
        with_redmine(|redmine| {
            let endpoint = ListQueries::builder().build()?;
            let values: Vec<serde_json::Value> = redmine.json_response_body_all_pages(&endpoint)?;
            for value in values {
                let o: QueryListItem = serde_json::from_value(value.clone())?;
                let reserialized = serde_json::to_value(o)?;
                assert_eq!(value, reserialized);
            }
            Ok(())
        })
    }
}
