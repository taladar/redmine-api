//! News Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_News)
//!
//! - [x] all news endpoint
//! - [x] project news endpoint
//!
use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::projects::ProjectEssentials;
use crate::api::users::UserEssentials;
use crate::api::{Endpoint, Pageable, ReturnsJsonResponse};

/// a type for news to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct News {
    /// numeric id
    pub id: u64,
    /// the project the news was published in
    pub project: ProjectEssentials,
    /// the author of the news
    pub author: UserEssentials,
    /// the title of the news
    pub title: String,
    /// the summary of the news
    pub summary: String,
    /// the description of the news (body)
    pub description: String,
    /// The time when this project was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
}
/// The endpoint for all news
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ListNews {}

impl ReturnsJsonResponse for ListNews {}
impl Pageable for ListNews {
    fn response_wrapper_key(&self) -> String {
        "news".to_string()
    }
}

impl ListNews {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListNewsBuilder {
        ListNewsBuilder::default()
    }
}

impl Endpoint for ListNews {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "news.json".into()
    }
}

/// The endpoint for project news
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct ListProjectNews<'a> {
    /// project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl<'a> ReturnsJsonResponse for ListProjectNews<'a> {}
impl<'a> Pageable for ListProjectNews<'a> {
    fn response_wrapper_key(&self) -> String {
        "news".to_string()
    }
}

impl<'a> ListProjectNews<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListProjectNewsBuilder<'a> {
        ListProjectNewsBuilder::default()
    }
}

impl<'a> Endpoint for ListProjectNews<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/news.json", self.project_id_or_name).into()
    }
}

/// helper struct for outer layers with a news field holding the inner data
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NewsWrapper<T> {
    /// to parse JSON with news key
    pub news: Vec<T>,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_list_news_no_pagination() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListNews::builder().build()?;
        redmine.json_response_body::<_, NewsWrapper<News>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_news_first_page() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListNews::builder().build()?;
        redmine.json_response_body_page::<_, News>(&endpoint, 0, 25)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_list_news_all_pages() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListNews::builder().build()?;
        redmine.json_response_body_all_pages::<_, News>(&endpoint)?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_news_type() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = ListNews::builder().build()?;
        let NewsWrapper { news: values } =
            redmine.json_response_body::<_, NewsWrapper<serde_json::Value>>(&endpoint)?;
        for value in values {
            let o: News = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
        }
        Ok(())
    }
}
