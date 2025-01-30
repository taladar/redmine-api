//! Uploads Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_api#Attaching-files)
//!
//! - [x] upload file endpoint
//! - [ ] create project file endpoint (in api::files)
//! - [x] [CreateIssue|crate::api::issues::CreateIssue] parameter for attachments (in api::issues)
//! - [x] [UpdateIssue|crate::api::issues::UpdateIssue] parameter for attachments (in api::issues)
//! - [ ] apparently news can have attachments too?

/// The endpoint to upload a file to Redmine for use in either project files
/// or issue attachments
///
use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;
use std::io::Read;
use std::path::PathBuf;

use crate::api::{Endpoint, QueryParams, ReturnsJsonResponse};

/// return type for the [UploadFile] endpoint, there is not much point in
/// making your own since it only has one field and if that is not used
/// calling [UploadFile] is useless
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FileUploadToken {
    /// the file upload token to be used in other endpoints
    token: String,
}

/// endpoint to upload a file for use in either project files or issue attachments
///
/// the token it returns needs to be passed to one of those endpoints for the file
/// to actually be visible anywhere in Redmine
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct UploadFile<'a> {
    /// the actual file to send to Redmine
    #[builder(setter(into))]
    file: PathBuf,
    /// the filename to send to Redmine, if not set it will be taken from
    /// the file
    #[builder(default, setter(into))]
    filename: Option<Cow<'a, str>>,
}

impl ReturnsJsonResponse for UploadFile<'_> {}

impl<'a> UploadFile<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> UploadFileBuilder<'a> {
        UploadFileBuilder::default()
    }
}

impl Endpoint for UploadFile<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "uploads.json".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        if let Some(ref filename) = self.filename {
            params.push("filename", filename);
        } else {
            let filename = self.file.file_name();
            if let Some(filename) = filename {
                params.push_opt("filename", filename.to_str());
            }
        }
        params
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        let mut file_content: Vec<u8> = Vec::new();
        let mut f = std::fs::File::open(&self.file)
            .map_err(|e| crate::Error::UploadFileError(self.file.clone(), e))?;
        f.read_to_end(&mut file_content)
            .map_err(|e| crate::Error::UploadFileError(self.file.clone(), e))?;
        Ok(Some(("application/octet-stream", file_content)))
    }
}

/// A lot of APIs in Redmine wrap their data in an extra layer, this is a
/// helper struct for outer layers with a upload field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UploadWrapper<T> {
    /// to parse JSON with upload key
    pub upload: T,
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use crate::api::issues::{
        test::ISSUES_LOCK, CreateIssue, Issue, IssueWrapper, UpdateIssue, UploadedAttachment,
    };
    use crate::api::test_helpers::with_project;
    use std::error::Error;
    use tracing_test::traced_test;

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_issue_with_attachment() -> Result<(), Box<dyn Error>> {
        let _w_issues = ISSUES_LOCK.write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, project_id, _| {
            let upload_endpoint = UploadFile::builder().file("README.md").build()?;
            let UploadWrapper {
                upload: FileUploadToken { token },
            } = redmine
                .json_response_body::<_, UploadWrapper<FileUploadToken>>(&upload_endpoint)?;
            let create_endpoint = CreateIssue::builder()
                .project_id(project_id)
                .subject("Attachment Test Issue")
                .uploads(vec![UploadedAttachment {
                    token: token.into(),
                    filename: "README.md".into(),
                    description: Some("Uploaded as part of unit test for redmine-api".into()),
                    content_type: "text/markdown".into(),
                }])
                .build()?;
            redmine.json_response_body::<_, IssueWrapper<Issue>>(&create_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_update_issue_with_attachment() -> Result<(), Box<dyn Error>> {
        let _w_issues = ISSUES_LOCK.write();
        let name = format!("unittest_{}", function_name!());
        with_project(&name, |redmine, project_id, _| {
            let upload_endpoint = UploadFile::builder().file("README.md").build()?;
            let UploadWrapper {
                upload: FileUploadToken { token },
            } = redmine
                .json_response_body::<_, UploadWrapper<FileUploadToken>>(&upload_endpoint)?;
            let create_endpoint = CreateIssue::builder()
                .project_id(project_id)
                .subject("Attachment Test Issue")
                .build()?;
            let IssueWrapper { issue }: IssueWrapper<Issue> =
                redmine.json_response_body::<_, _>(&create_endpoint)?;
            let update_endpoint = UpdateIssue::builder()
                .id(issue.id)
                .subject("New test subject")
                .uploads(vec![UploadedAttachment {
                    token: token.into(),
                    filename: "README.md".into(),
                    description: Some("Uploaded as part of unit test for redmine-api".into()),
                    content_type: "text/markdown".into(),
                }])
                .build()?;
            redmine.ignore_response_body::<_>(&update_endpoint)?;
            Ok(())
        })?;
        Ok(())
    }
}
