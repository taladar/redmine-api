//! Queries Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Queries)
//!
//! - [x] all (visible) custom queries endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::{Endpoint, Pageable, ReturnsJsonResponse};

/// a type for query to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Query {
    /// numeric id
    pub id: u64,
    /// display name
    pub name: String,
    /// is this query public
    pub is_public: bool,
    /// the project for project-specific queries
    pub project_id: Option<u64>,
}

/// The endpoint to retrieve all queries visible to the current user
///
/// to actualy use them pass the query_id to the ListIssues endpoint
#[derive(Debug, Builder)]
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

impl<'a> Endpoint for ListQueries {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "queries.json".into()
    }
}

/// helper struct for outer layers with a queries field holding the inner data
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QueriesWrapper<T> {
    /// to parse JSON with queries key
    pub queries: Vec<T>,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_list_queries_no_pagination() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListQueries::builder().build()?;
        redmine.json_response_body::<_, QueriesWrapper<Query>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_queries_first_page() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListQueries::builder().build()?;
        redmine.json_response_body_page::<_, Query>(&endpoint, 0, 25)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_queries_all_pages() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListQueries::builder().build()?;
        redmine.json_response_body_all_pages::<_, Query>(&endpoint)?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_query_type() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListQueries::builder().build()?;
        let QueriesWrapper { queries: values } =
            redmine.json_response_body::<_, QueriesWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: Query = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
