//! Attachments Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Attachments)
//!
//! - [x] specific attachment endpoint
//! - [x] update attachment endpoint
//! - [x] delete attachment endpoint

use derive_builder::Builder;
use reqwest::Method;
use std::borrow::Cow;

use crate::api::users::UserEssentials;
use crate::api::{Endpoint, NoPagination, ReturnsJsonResponse};

/// a type for attachment to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Attachment {
    /// numeric id
    pub id: u64,
    /// filename as specified on upload
    pub filename: String,
    /// file size
    pub filesize: u64,
    /// content MIME type
    pub content_type: Option<String>,
    /// description
    #[serde(default)]
    pub description: Option<String>,
    /// url where the content of this attachment can be downloaded
    pub content_url: String,
    /// uploader
    pub author: UserEssentials,
    /// The time when this file was uploaded
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// the URL for the thumbnail for this attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// A string containing a hash of the file content (e.g., SHA256 or MD5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// An integer representing the download count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downloads: Option<u64>,
}

/// The endpoint for a specific Redmine attachment
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct GetAttachment {
    /// id of the attachment to retrieve
    id: u64,
}

impl ReturnsJsonResponse for GetAttachment {}
impl NoPagination for GetAttachment {}

impl GetAttachment {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> GetAttachmentBuilder {
        GetAttachmentBuilder::default()
    }
}

impl Endpoint for GetAttachment {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("attachments/{}.json", &self.id).into()
    }
}

/// The endpoint to update a Redmine attachment
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct UpdateAttachment {
    /// id of the attachment to update
    id: u64,
    /// the attachment update data
    attachment: AttachmentUpdate,
}

impl UpdateAttachment {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> UpdateAttachmentBuilder {
        UpdateAttachmentBuilder::default()
    }
}

impl ReturnsJsonResponse for UpdateAttachment {}
impl NoPagination for UpdateAttachment {}

impl Endpoint for UpdateAttachment {
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("attachments/{}.json", &self.id).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&AttachmentWrapper {
                attachment: self.attachment.clone(),
            })?,
        )))
    }
}

/// The attachment update data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub struct AttachmentUpdate {
    /// new filename
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// new description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// The endpoint to delete a Redmine attachment
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteAttachment {
    /// id of the attachment to delete
    id: u64,
}

impl DeleteAttachment {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> DeleteAttachmentBuilder {
        DeleteAttachmentBuilder::default()
    }
}

impl Endpoint for DeleteAttachment {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("attachments/{}.json", &self.id).into()
    }
}

/// helper struct for outer layers with a attachment field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AttachmentWrapper<T> {
    /// to parse JSON with attachment key
    pub attachment: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_get_attachment() -> Result<(), Box<dyn Error>> {
        use crate::api::issues::{CreateIssue, GetIssue, Issue, IssueWrapper, UploadedAttachment};
        use crate::api::test_helpers::with_project;
        use crate::api::uploads::{FileUploadToken, UploadFile, UploadWrapper};

        with_project("test_get_attachment", |redmine, project_id, _| {
            let upload_endpoint = UploadFile::builder().file("README.md").build()?;
            let UploadWrapper {
                upload: FileUploadToken { id: _, token },
            } = redmine
                .json_response_body::<_, UploadWrapper<FileUploadToken>>(&upload_endpoint)?;
            let create_issue_endpoint = CreateIssue::builder()
                .project_id(project_id)
                .subject("Attachment Test Issue")
                .uploads(vec![UploadedAttachment {
                    token: token.into(),
                    filename: "README.md".into(),
                    description: Some("Uploaded as part of unit test for redmine-api".into()),
                    content_type: "application/octet-stream".into(),
                }])
                .build()?;
            let IssueWrapper {
                issue: created_issue,
            } = redmine.json_response_body::<_, IssueWrapper<Issue>>(&create_issue_endpoint)?;

            let get_issue_endpoint = GetIssue::builder()
                .id(created_issue.id)
                .include(vec![crate::api::issues::IssueInclude::Attachments])
                .build()?;
            let IssueWrapper { issue } =
                redmine.json_response_body::<_, IssueWrapper<Issue>>(&get_issue_endpoint)?;
            let attachment_id = issue.attachments.unwrap().first().unwrap().id;

            let endpoint = GetAttachment::builder().id(attachment_id).build()?;
            redmine.json_response_body::<_, AttachmentWrapper<Attachment>>(&endpoint)?;
            Ok(())
        })
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_attachment_type() -> Result<(), Box<dyn Error>> {
        let current_span = tracing::Span::current();
        crate::api::test_helpers::with_redmine(current_span, |redmine| {
            let endpoint = GetAttachment::builder().id(38468).build()?;
            let AttachmentWrapper { attachment: value } =
                redmine.json_response_body::<_, AttachmentWrapper<serde_json::Value>>(&endpoint)?;
            let o: Attachment = serde_json::from_value(value.clone())?;
            let reserialized = serde_json::to_value(o)?;
            assert_eq!(value, reserialized);
            Ok(())
        })
    }

    #[traced_test]
    #[test]
    fn test_update_delete_attachment() -> Result<(), Box<dyn Error>> {
        use crate::api::issues::{CreateIssue, GetIssue, Issue, IssueWrapper, UploadedAttachment};
        use crate::api::test_helpers::with_project;
        use crate::api::uploads::{FileUploadToken, UploadFile, UploadWrapper};

        with_project("test_update_delete_attachment", |redmine, project_id, _| {
            let upload_endpoint = UploadFile::builder().file("README.md").build()?;
            let UploadWrapper {
                upload: FileUploadToken { id: _, token },
            } = redmine
                .json_response_body::<_, UploadWrapper<FileUploadToken>>(&upload_endpoint)?;
            let create_issue_endpoint = CreateIssue::builder()
                .project_id(project_id)
                .subject("Attachment Test Issue")
                .uploads(vec![UploadedAttachment {
                    token: token.into(),
                    filename: "README.md".into(),
                    description: Some("Uploaded as part of unit test for redmine-api".into()),
                    content_type: "application/octet-stream".into(),
                }])
                .build()?;
            let IssueWrapper {
                issue: created_issue,
            } = redmine.json_response_body::<_, IssueWrapper<Issue>>(&create_issue_endpoint)?;

            let get_issue_endpoint = GetIssue::builder()
                .id(created_issue.id)
                .include(vec![crate::api::issues::IssueInclude::Attachments])
                .build()?;
            let IssueWrapper { issue } =
                redmine.json_response_body::<_, IssueWrapper<Issue>>(&get_issue_endpoint)?;
            let attachment_id = issue.attachments.unwrap().first().unwrap().id;

            let update_endpoint = UpdateAttachment::builder()
                .id(attachment_id)
                .attachment(AttachmentUpdate {
                    filename: Some("new_readme.md".to_string()),
                    description: Some("new description".to_string()),
                })
                .build()?;
            redmine.ignore_response_body(&update_endpoint)?;

            let get_endpoint = GetAttachment::builder().id(attachment_id).build()?;
            let AttachmentWrapper { attachment } =
                redmine.json_response_body::<_, AttachmentWrapper<Attachment>>(&get_endpoint)?;
            assert_eq!(attachment.filename, "new_readme.md");
            assert_eq!(attachment.description.unwrap(), "new description");

            let delete_endpoint = DeleteAttachment::builder().id(attachment_id).build()?;
            redmine.ignore_response_body(&delete_endpoint)?;

            let get_issue_endpoint = GetIssue::builder()
                .id(issue.id)
                .include(vec![crate::api::issues::IssueInclude::Attachments])
                .build()?;
            let IssueWrapper { issue } =
                redmine.json_response_body::<_, IssueWrapper<Issue>>(&get_issue_endpoint)?;
            assert!(issue.attachments.is_none_or(|v| v.is_empty()));
            Ok(())
        })
    }
}
