//! Attachments Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Attachments)
//!
//! - [x] specific attachment endpoint
//! - [ ] update attachment endpoint (not documented and the link to the issue in the wiki points to an issue about something else)
//! - [x] delete attachment endpoint

use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::api::users::UserEssentials;
use crate::api::{Endpoint, ReturnsJsonResponse};

/// a type for attachment to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Attachment {
    /// numeric id
    pub id: u64,
    /// filename as specified on upload
    pub filename: String,
    /// file size
    pub filesize: u64,
    /// content MIME type
    pub content_type: String,
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
}

/// The endpoint for a specific Redmine attachment
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct GetAttachment {
    /// id of the attachment to retrieve
    id: u64,
}

impl<'a> ReturnsJsonResponse for GetAttachment {}

impl<'a> GetAttachment {
    /// Create a builder for the endpoint.
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

/// The endpoint to delete a Redmine attachment
#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct DeleteAttachment {
    /// id of the attachment to delete
    id: u64,
}

impl DeleteAttachment {
    /// Create a builder for the endpoint.
    pub fn builder() -> DeleteAttachmentBuilder {
        DeleteAttachmentBuilder::default()
    }
}

impl<'a> Endpoint for DeleteAttachment {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("attachments/{}.json", &self.id).into()
    }
}

/// helper struct for outer layers with a attachment field holding the inner data
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = GetAttachment::builder().id(3).build()?;
        redmine.json_response_body::<_, AttachmentWrapper<Attachment>>(&endpoint)?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[traced_test]
    #[test]
    fn test_completeness_attachment_type() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv()?;
        let redmine = crate::api::Redmine::from_env()?;
        let endpoint = GetAttachment::builder().id(3).build()?;
        let AttachmentWrapper { attachment: value } =
            redmine.json_response_body::<_, AttachmentWrapper<serde_json::Value>>(&endpoint)?;
        let o: Attachment = serde_json::from_value(value.clone())?;
        let reserialized = serde_json::to_value(o)?;
        assert_eq!(value, reserialized);
        Ok(())
    }
}
