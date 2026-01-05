//! Wiki Pages Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_WikiPages)
//!
//! - [X] project specific wiki page endpoint
//! - [X] specific wiki page endpoint
//! - [X] specific wiki page old version endpoint
//! - [X] create or update wiki page endpoint
//! - [X] delete wiki page endpoint
//! - [ ] attachments
//!
//! The following endpoints always return 403 and are apparently not exposed in a usable way:
//! - GetProjectWikiPageHistory
//! - GetProjectWikiPageDiff
//! - RenameProjectWikiPage
//! - ProtectProjectWikiPage
//! - AddAttachmentToProjectWikiPage

use derive_builder::Builder;
use reqwest::Method;
use serde::Serialize;
use std::borrow::Cow;

use crate::api::attachments::Attachment;
use crate::api::users::UserEssentials;
use crate::api::{Endpoint, NoPagination, QueryParams, ReturnsJsonResponse};

/// The types of associated data which can be fetched along with a wiki page
#[derive(Debug, Clone)]
pub enum WikiPageInclude {
    /// Wiki Page Attachments
    Attachments,
}

impl std::fmt::Display for WikiPageInclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attachments => {
                write!(f, "attachments")
            }
        }
    }
}

/// The parent of a wiki page
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct WikiPageParent {
    /// title
    pub title: String,
}

/// a type for wiki pages to use as an API return type for the list call
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct WikiPageEssentials {
    /// title
    pub title: String,
    /// the parent of this page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<WikiPageParent>,
    /// the version number of the wiki page
    pub version: u64,
    /// The time when this wiki page was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// The time when this wiki page was last updated
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub updated_on: time::OffsetDateTime,
    /// wiki page attachments (only when include parameter is used)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// is the wiki page protected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
}

/// a type for wiki pages to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct WikiPage {
    /// title
    pub title: String,
    /// the parent of this page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<WikiPageParent>,
    /// author
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<UserEssentials>,
    /// the text body of the wiki page
    pub text: String,
    /// the version number of the wiki page
    pub version: u64,
    /// the comments supplied when saving this version of the page
    pub comments: String,
    /// The time when this wiki page was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// The time when this wiki page was last updated
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub updated_on: time::OffsetDateTime,
    /// wiki page attachments (only when include parameter is used)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// is the wiki page protected
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
}

/// The endpoint for all wiki pages in a project
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListProjectWikiPages<'a> {
    /// project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// Include associated data
    #[builder(default)]
    include: Option<Vec<WikiPageInclude>>,
}

impl<'a> ReturnsJsonResponse for ListProjectWikiPages<'a> {}
impl<'a> NoPagination for ListProjectWikiPages<'a> {}

impl<'a> ListProjectWikiPages<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListProjectWikiPagesBuilder<'a> {
        ListProjectWikiPagesBuilder::default()
    }
}

impl<'a> Endpoint for ListProjectWikiPages<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/wiki/index.json", self.project_id_or_name).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("include", self.include.as_ref());
        params
    }
}

/// The endpoint for a specific Redmine project wiki page
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetProjectWikiPage<'a> {
    /// the project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// the title as it appears in the URL
    #[builder(setter(into))]
    title: Cow<'a, str>,
    /// the types of associate data to include
    #[builder(default)]
    include: Option<Vec<WikiPageInclude>>,
}

impl ReturnsJsonResponse for GetProjectWikiPage<'_> {}
impl NoPagination for GetProjectWikiPage<'_> {}

impl<'a> GetProjectWikiPage<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetProjectWikiPageBuilder<'a> {
        GetProjectWikiPageBuilder::default()
    }
}

impl Endpoint for GetProjectWikiPage<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/wiki/{}.json",
            &self.project_id_or_name, &self.title
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("include", self.include.as_ref());
        params
    }
}

/// The endpoint for a specific Redmine project wiki page version
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetProjectWikiPageVersion<'a> {
    /// the project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// the title as it appears in the URL
    #[builder(setter(into))]
    title: Cow<'a, str>,
    /// the version
    version: u64,
    /// the types of associate data to include
    #[builder(default)]
    include: Option<Vec<WikiPageInclude>>,
}

impl ReturnsJsonResponse for GetProjectWikiPageVersion<'_> {}
impl NoPagination for GetProjectWikiPageVersion<'_> {}

impl<'a> GetProjectWikiPageVersion<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetProjectWikiPageVersionBuilder<'a> {
        GetProjectWikiPageVersionBuilder::default()
    }
}

impl Endpoint for GetProjectWikiPageVersion<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/wiki/{}/{}.json",
            &self.project_id_or_name, &self.title, &self.version,
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("include", self.include.as_ref());
        params
    }
}

/// The endpoint to create or update a Redmine project wiki page
#[derive(Debug, Clone, Builder, serde::Serialize, serde::Deserialize)]
#[builder(setter(strip_option))]
pub struct CreateOrUpdateProjectWikiPage<'a> {
    /// the project id or name as it appears in the URL
    #[serde(skip_serializing)]
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// the title as it appears in the URL
    #[serde(skip_serializing)]
    #[builder(setter(into))]
    title: Cow<'a, str>,
    /// the version to update, if the version is not this a 409 Conflict is returned
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    version: Option<u64>,
    /// the body text of the page
    #[builder(setter(into))]
    text: Cow<'a, str>,
    /// the comment for the update history
    #[builder(setter(into))]
    comments: Cow<'a, str>,
    /// used when renaming or moving a page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    redirect_existing_links: Option<bool>,
    /// is the wiki page the start page for the project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    is_start_page: Option<bool>,
}

impl<'a> CreateOrUpdateProjectWikiPage<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> CreateOrUpdateProjectWikiPageBuilder<'a> {
        CreateOrUpdateProjectWikiPageBuilder::default()
    }
}

impl Endpoint for CreateOrUpdateProjectWikiPage<'_> {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/wiki/{}.json",
            &self.project_id_or_name, &self.title
        )
        .into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&WikiPageWrapper::<CreateOrUpdateProjectWikiPage> {
                wiki_page: (*self).to_owned(),
            })?,
        )))
    }
}

/// The endpoint to delete a Redmine project wiki page
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteProjectWikiPage<'a> {
    /// the project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// the title as it appears in the URL
    #[builder(setter(into))]
    title: Cow<'a, str>,
    /// what to do with descendant pages: `null` (default) or `destroy`
    #[builder(default)]
    todo: Option<Cow<'a, str>>,
    /// the id of the wiki page to reassign descendant pages to
    #[builder(default)]
    reassign_to_id: Option<u64>,
}

impl<'a> DeleteProjectWikiPage<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> DeleteProjectWikiPageBuilder<'a> {
        DeleteProjectWikiPageBuilder::default()
    }
}

impl<'a> Endpoint for DeleteProjectWikiPage<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/wiki/{}.json",
            &self.project_id_or_name, &self.title
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("todo", self.todo.as_ref());
        params.push_opt("reassign_to_id", self.reassign_to_id);
        params
    }
}

/// The endpoint to delete a specific version of a Redmine project wiki page
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteProjectWikiPageVersion<'a> {
    /// the project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// the title as it appears in the URL
    #[builder(setter(into))]
    title: Cow<'a, str>,
    /// the version to delete
    version: u64,
}

impl<'a> DeleteProjectWikiPageVersion<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> DeleteProjectWikiPageVersionBuilder<'a> {
        DeleteProjectWikiPageVersionBuilder::default()
    }
}

impl<'a> Endpoint for DeleteProjectWikiPageVersion<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/wiki/{}/{}/destroy_version.json",
            &self.project_id_or_name, &self.title, &self.version
        )
        .into()
    }
}

/// The endpoint to get the annotated view of a Redmine project wiki page
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetProjectWikiPageAnnotate<'a> {
    /// the project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// the title as it appears in the URL
    #[builder(setter(into))]
    title: Cow<'a, str>,
    /// the version to annotate
    version: u64,
}

impl<'a> GetProjectWikiPageAnnotate<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetProjectWikiPageAnnotateBuilder<'a> {
        GetProjectWikiPageAnnotateBuilder::default()
    }
}

impl NoPagination for GetProjectWikiPageAnnotate<'_> {}

impl Endpoint for GetProjectWikiPageAnnotate<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/wiki/{}/annotate.json",
            &self.project_id_or_name, &self.title
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push("v", self.version);
        params
    }
}

/// The endpoint to export a Redmine project wiki page
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ExportProjectWikiPage<'a> {
    /// the project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
    /// the title as it appears in the URL
    #[builder(setter(into))]
    title: Cow<'a, str>,
    /// the version to export
    #[builder(default)]
    version: Option<u64>,
}

impl<'a> ExportProjectWikiPage<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ExportProjectWikiPageBuilder<'a> {
        ExportProjectWikiPageBuilder::default()
    }
}

impl NoPagination for ExportProjectWikiPage<'_> {}

impl Endpoint for ExportProjectWikiPage<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "projects/{}/wiki/{}/export.json",
            &self.project_id_or_name, &self.title
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        let mut params = QueryParams::default();
        params.push_opt("v", self.version);
        params
    }
}

/// helper struct for outer layers with a wiki_pages field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct WikiPagesWrapper<T> {
    /// to parse JSON with wiki_pages key
    pub wiki_pages: Vec<T>,
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a wiki_page field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct WikiPageWrapper<T> {
    /// to parse JSON with an wiki_page key
    pub wiki_page: T,
}

#[cfg(test)]
pub(crate) mod test {
    use crate::api::projects::{ListProjects, Project, ProjectsInclude, test::PROJECT_LOCK};

    use super::*;
    use std::error::Error;
    use tokio::sync::RwLock;
    use tracing_test::traced_test;

    /// needed so we do not get 404s when listing while
    /// creating/deleting or creating/updating/deleting
    pub static PROJECT_WIKI_PAGE_LOCK: RwLock<()> = RwLock::const_new(());

    #[traced_test]
    #[test]
    fn test_list_project_wiki_pages() -> Result<(), Box<dyn Error>> {
        let _r_project = PROJECT_LOCK.blocking_read();
        let _r_project_wiki_pages = PROJECT_WIKI_PAGE_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .tls_backend_rustls()
                .build()?,
        )?;
        let endpoint = ListProjectWikiPages::builder()
            .project_id_or_name("25")
            .build()?;
        redmine.json_response_body::<_, WikiPagesWrapper<WikiPageEssentials>>(&endpoint)?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_wiki_page_essentials() -> Result<(), Box<dyn Error>> {
        let _r_project = PROJECT_LOCK.blocking_read();
        let _r_issues = PROJECT_WIKI_PAGE_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .tls_backend_rustls()
                .build()?,
        )?;
        let endpoint = ListProjects::builder()
            .include(vec![ProjectsInclude::EnabledModules])
            .build()?;
        let projects = redmine.json_response_body_all_pages_iter::<_, Project>(&endpoint);
        let mut checked_projects = 0;
        for project in projects {
            let project = project?;
            if !project
                .enabled_modules
                .is_some_and(|em| em.iter().any(|m| m.name == "wiki"))
            {
                // skip projects where wiki is disabled
                continue;
            }
            let endpoint = ListProjectWikiPages::builder()
                .project_id_or_name(project.id.to_string())
                .include(vec![WikiPageInclude::Attachments])
                .build()?;
            let Ok(WikiPagesWrapper { wiki_pages: values }) =
                redmine.json_response_body::<_, WikiPagesWrapper<serde_json::Value>>(&endpoint)
            else {
                // TODO: some projects return a 404 for their wiki for unknown reasons even with an
                //       enabled wiki module. They also do not have a wiki tab so I assume
                //       it is intentional, they are not closed or archived either
                //
                //       Further analysis seems to indicate that this should not happen and is most
                //       likely an issue resulting from a database state from a buggy old version of
                //       Redmine
                continue;
            };
            checked_projects += 1;
            for value in values {
                let o: WikiPageEssentials = serde_json::from_value(value.clone())?;
                let reserialized = serde_json::to_value(o)?;
                assert_eq!(value, reserialized);
            }
        }
        assert!(checked_projects > 0);
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_project_wiki_page() -> Result<(), Box<dyn Error>> {
        let _r_project = PROJECT_LOCK.blocking_read();
        let _r_project_wiki_pages = PROJECT_WIKI_PAGE_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .tls_backend_rustls()
                .build()?,
        )?;
        let endpoint = GetProjectWikiPage::builder()
            .project_id_or_name("25")
            .title("Administration")
            .build()?;
        redmine.json_response_body::<_, WikiPageWrapper<WikiPage>>(&endpoint)?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_wiki_page() -> Result<(), Box<dyn Error>> {
        let _r_project = PROJECT_LOCK.blocking_read();
        let _r_issues = PROJECT_WIKI_PAGE_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .tls_backend_rustls()
                .build()?,
        )?;
        let endpoint = ListProjects::builder()
            .include(vec![ProjectsInclude::EnabledModules])
            .build()?;
        let projects = redmine.json_response_body_all_pages_iter::<_, Project>(&endpoint);
        let mut checked_pages = 0;
        for project in projects {
            let project = project?;
            if !project
                .enabled_modules
                .is_some_and(|em| em.iter().any(|m| m.name == "wiki"))
            {
                // skip projects where wiki is disabled
                continue;
            }
            let endpoint = ListProjectWikiPages::builder()
                .project_id_or_name(project.id.to_string())
                .include(vec![WikiPageInclude::Attachments])
                .build()?;
            let Ok(WikiPagesWrapper { wiki_pages }) =
                redmine.json_response_body::<_, WikiPagesWrapper<WikiPageEssentials>>(&endpoint)
            else {
                // TODO: some projects return a 404 for their wiki for unknown reasons even with an
                //       enabled wiki module. They also do not have a wiki tab so I assume
                //       it is intentional, they are not closed or archived either
                //
                //       Further analysis seems to indicate that this should not happen and is most
                //       likely an issue resulting from a database state from a buggy old version of
                //       Redmine
                continue;
            };
            checked_pages += 1;
            for wiki_page in wiki_pages {
                let endpoint = GetProjectWikiPage::builder()
                    .project_id_or_name(project.id.to_string())
                    .title(wiki_page.title)
                    .include(vec![WikiPageInclude::Attachments])
                    .build()?;
                let WikiPageWrapper { wiki_page: value } = redmine
                    .json_response_body::<_, WikiPageWrapper<serde_json::Value>>(&endpoint)?;
                let o: WikiPage = serde_json::from_value(value.clone())?;
                let reserialized = serde_json::to_value(o)?;
                assert_eq!(value, reserialized);
            }
        }
        assert!(checked_pages > 0);
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_get_project_wiki_page_version() -> Result<(), Box<dyn Error>> {
        let _r_project = PROJECT_LOCK.blocking_read();
        let _r_project_wiki_pages = PROJECT_WIKI_PAGE_LOCK.blocking_read();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .tls_backend_rustls()
                .build()?,
        )?;
        let endpoint = GetProjectWikiPageVersion::builder()
            .project_id_or_name("25")
            .title("Administration")
            .version(18)
            .build()?;
        redmine.json_response_body::<_, WikiPageWrapper<WikiPage>>(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_create_update_and_delete_project_wiki_page() -> Result<(), Box<dyn Error>> {
        let _r_project = PROJECT_LOCK.blocking_read();
        let _w_project_wiki_pages = PROJECT_WIKI_PAGE_LOCK.blocking_write();
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .tls_backend_rustls()
                .build()?,
        )?;
        let endpoint = GetProjectWikiPage::builder()
            .project_id_or_name("25")
            .title("CreateWikiPageTest")
            .build()?;
        if redmine.ignore_response_body(&endpoint).is_ok() {
            // left-over from past test that failed to complete
            let endpoint = DeleteProjectWikiPage::builder()
                .project_id_or_name("25")
                .title("CreateWikiPageTest")
                .build()?;
            redmine.ignore_response_body(&endpoint)?;
        }
        let endpoint = CreateOrUpdateProjectWikiPage::builder()
            .project_id_or_name("25")
            .title("CreateWikiPageTest")
            .text("Test Content")
            .comments("Create Page Test")
            .build()?;
        redmine.ignore_response_body(&endpoint)?;
        let endpoint = CreateOrUpdateProjectWikiPage::builder()
            .project_id_or_name("25")
            .title("CreateWikiPageTest")
            .text("Test Content Updates")
            .version(1)
            .comments("Update Page Test")
            .build()?;
        redmine.ignore_response_body(&endpoint)?;
        let endpoint = DeleteProjectWikiPage::builder()
            .project_id_or_name("25")
            .title("CreateWikiPageTest")
            .build()?;
        redmine.ignore_response_body(&endpoint)?;
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_wiki_page_lifecycle() -> Result<(), Box<dyn Error>> {
        use crate::api::test_helpers::with_project;

        with_project("test_wiki_page_lifecycle", |redmine, project_id, _| {
            tracing::debug!("Creating wiki page TestWikiPage");
            let endpoint = CreateOrUpdateProjectWikiPage::builder()
                .project_id_or_name(project_id.to_string())
                .title("TestWikiPage")
                .text("Test Content")
                .comments("Create Page Test")
                .build()?;
            redmine.ignore_response_body(&endpoint)?;

            tracing::debug!("Verifying existence, content and version of wiki page TestWikiPage");
            let get_endpoint = GetProjectWikiPage::builder()
                .project_id_or_name(project_id.to_string())
                .title("TestWikiPage")
                .build()?;
            let WikiPageWrapper { wiki_page } =
                redmine.json_response_body::<_, WikiPageWrapper<WikiPage>>(&get_endpoint)?;
            assert_eq!(wiki_page.text, "Test Content");
            assert_eq!(wiki_page.version, 1);

            tracing::debug!("Updating wiki page TestWikiPage");
            let update_endpoint = CreateOrUpdateProjectWikiPage::builder()
                .project_id_or_name(project_id.to_string())
                .title("TestWikiPage")
                .text("Test Content Updates")
                .version(1)
                .comments("Update Page Test")
                .build()?;
            redmine.ignore_response_body(&update_endpoint)?;

            tracing::debug!(
                "Verifying existence, content and version of updated wiki page TestWikiPage"
            );
            let get_endpoint = GetProjectWikiPage::builder()
                .project_id_or_name(project_id.to_string())
                .title("TestWikiPage")
                .build()?;
            let WikiPageWrapper { wiki_page } =
                redmine.json_response_body::<_, WikiPageWrapper<WikiPage>>(&get_endpoint)?;
            assert_eq!(wiki_page.text, "Test Content Updates");
            assert_eq!(wiki_page.version, 2);

            tracing::debug!("Verifying existence and content of wiki page TestWikiPage version 1");
            let version_endpoint = GetProjectWikiPageVersion::builder()
                .project_id_or_name(project_id.to_string())
                .title("TestWikiPage")
                .version(1)
                .build()?;
            let WikiPageWrapper { wiki_page } =
                redmine.json_response_body::<_, WikiPageWrapper<WikiPage>>(&version_endpoint)?;
            assert_eq!(wiki_page.text, "Test Content");

            tracing::debug!("Deleting wiki page TestWikiPage");
            let delete_endpoint = DeleteProjectWikiPage::builder()
                .project_id_or_name(project_id.to_string())
                .title("TestWikiPage")
                .build()?;
            redmine.ignore_response_body(&delete_endpoint)?;

            Ok(())
        })
    }
}
