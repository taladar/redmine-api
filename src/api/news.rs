//! News Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_News)
//!
//! - [x] all news endpoint
//! - [x] project news endpoint
//!
use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::projects::ProjectEssentials;
use crate::api::users::UserEssentials;
use crate::api::{Endpoint, Pageable, ReturnsJsonResponse};

/// a type for news to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, Builder)]
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
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListProjectNews<'a> {
    /// project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl ReturnsJsonResponse for ListProjectNews<'_> {}
impl Pageable for ListProjectNews<'_> {
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

impl Endpoint for ListProjectNews<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/news.json", self.project_id_or_name).into()
    }
}

/// The endpoint for a specific news item
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetNews {
    /// the id of the news item to retrieve
    id: u64,
}

impl ReturnsJsonResponse for GetNews {}
impl crate::api::NoPagination for GetNews {}

impl GetNews {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetNewsBuilder {
        GetNewsBuilder::default()
    }
}

impl Endpoint for GetNews {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("news/{}.json", self.id).into()
    }
}

/// The endpoint to create a Redmine news item
#[derive(Debug, Clone, Builder, serde::Serialize)]
#[builder(setter(strip_option))]
pub struct CreateNews<'a> {
    /// project id or name as it appears in the URL
    #[builder(setter(into))]
    #[serde(skip_serializing)]
    project_id_or_name: Cow<'a, str>,
    /// the title of the news
    #[builder(setter(into))]
    title: Cow<'a, str>,
    /// the summary of the news
    #[builder(setter(into), default)]
    summary: Option<Cow<'a, str>>,
    /// the description of the news (body)
    #[builder(setter(into))]
    description: Cow<'a, str>,
}

impl<'a> CreateNews<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> CreateNewsBuilder<'a> {
        CreateNewsBuilder::default()
    }
}

impl crate::api::NoPagination for CreateNews<'_> {}

impl Endpoint for CreateNews<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/news.json", self.project_id_or_name).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&SingleNewsWrapper::<CreateNews> {
                news: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to update a Redmine news item
#[derive(Debug, Clone, Builder, serde::Serialize)]
#[builder(setter(strip_option))]
pub struct UpdateNews<'a> {
    /// the id of the news item to update
    #[serde(skip_serializing)]
    id: u64,
    /// the title of the news
    #[builder(setter(into), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Cow<'a, str>>,
    /// the summary of the news
    #[builder(setter(into), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<Cow<'a, str>>,
    /// the description of the news (body)
    #[builder(setter(into), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Cow<'a, str>>,
}

impl<'a> UpdateNews<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> UpdateNewsBuilder<'a> {
        UpdateNewsBuilder::default()
    }
}

impl Endpoint for UpdateNews<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("news/{}.json", self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&SingleNewsWrapper::<UpdateNews> {
                news: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to delete a Redmine news item
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteNews {
    /// the id of the news item to delete
    id: u64,
}

impl DeleteNews {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> DeleteNewsBuilder {
        DeleteNewsBuilder::default()
    }
}

impl Endpoint for DeleteNews {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("news/{}.json", self.id).into()
    }
}

/// helper struct for outer layers with a news field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NewsWrapper<T> {
    /// to parse JSON with news key
    pub news: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a news field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SingleNewsWrapper<T> {
    /// to parse JSON with a news key
    pub news: T,
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
    fn test_list_news_first_page() -> Result<(), Box<dyn Error>> {
        with_redmine(|redmine| {
            let endpoint = ListNews::builder().build()?;
            redmine.json_response_body_page::<_, News>(&endpoint, 0, 25)?;
            Ok(())
        })
    }

    #[traced_test]
    #[test]
    fn test_list_news_all_pages() -> Result<(), Box<dyn Error>> {
        with_redmine(|redmine| {
            let endpoint = ListNews::builder().build()?;
            redmine.json_response_body_all_pages::<_, News>(&endpoint)?;
            Ok(())
        })
    }

    #[traced_test]
    #[test]
    fn test_get_update_delete_news() -> Result<(), Box<dyn Error>> {
        crate::api::test_helpers::with_project("test_get_update_delete_news", |redmine, _, name| {
            let create_endpoint = CreateNews::builder()
                .project_id_or_name(name)
                .title("Test News")
                .summary("Test Summary")
                .description("Test Description")
                .build()?;
            redmine.ignore_response_body(&create_endpoint)?;
            let list_endpoint = ListProjectNews::builder()
                .project_id_or_name(name)
                .build()?;
            let news: Vec<News> = redmine.json_response_body_all_pages(&list_endpoint)?;
            let created_news = news
                .into_iter()
                .find(|n| n.title == "Test News")
                .ok_or("Could not find created news")?;
            let get_endpoint = GetNews::builder().id(created_news.id).build()?;
            let fetched_news: SingleNewsWrapper<News> =
                redmine.json_response_body(&get_endpoint)?;
            assert_eq!(created_news, fetched_news.news);
            let update_endpoint = UpdateNews::builder()
                .id(created_news.id)
                .title("New Test News")
                .build()?;
            redmine.ignore_response_body(&update_endpoint)?;
            let delete_endpoint = DeleteNews::builder().id(created_news.id).build()?;
            redmine.ignore_response_body(&delete_endpoint)?;
            Ok(())
        })
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_news_type() -> Result<(), Box<dyn Error>> {
        with_redmine(|redmine| {
            let endpoint = ListNews::builder().build()?;
            let values: Vec<serde_json::Value> = redmine.json_response_body_all_pages(&endpoint)?;
            for value in values {
                let o: News = serde_json::from_value(value.clone())?;
                let reserialized = serde_json::to_value(o)?;
                assert_eq!(value, reserialized);
            }
            Ok(())
        })
    }
}
