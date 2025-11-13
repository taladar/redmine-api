//! Files Rest API Endpoint definitions
//!
//! [Redmine Documentation](https://www.redmine.org/projects/redmine/wiki/Rest_Files)
//!
//! - [x] project specific files endpoint
//! - [ ] create file endpoint

use std::borrow::Cow;

use derive_builder::Builder;
use reqwest::Method;
use serde::Serialize;

use crate::api::users::UserEssentials;
use crate::api::{Endpoint, NoPagination, QueryParams, ReturnsJsonResponse};

/// a type for files to use as an API return type
///
/// alternatively you can use your own type limited to the fields you need
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct File {
    /// numeric id
    pub id: u64,
    /// the filename
    pub filename: String,
    /// the file size in bytes
    pub filesize: u64,
    /// the file content type
    pub content_type: String,
    /// the file description
    pub description: String,
    /// a token for the file
    pub token: String,
    /// the author of the file
    pub author: UserEssentials,
    /// The time when this file was created
    #[serde(
        serialize_with = "crate::api::serialize_rfc3339",
        deserialize_with = "crate::api::deserialize_rfc3339"
    )]
    pub created_on: time::OffsetDateTime,
    /// the digest of the file
    pub digest: String,
    /// the number of downloads
    pub downloads: u64,
}

/// The endpoint for all files in a Redmine project
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ListProjectFiles<'a> {
    /// project id or name as it appears in the URL
    #[builder(setter(into))]
    project_id_or_name: Cow<'a, str>,
}

impl ReturnsJsonResponse for ListProjectFiles<'_> {}
impl NoPagination for ListProjectFiles<'_> {}

impl<'a> ListProjectFiles<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> ListProjectFilesBuilder<'a> {
        ListProjectFilesBuilder::default()
    }
}

impl Endpoint for ListProjectFiles<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/files.json", self.project_id_or_name).into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        QueryParams::default()
    }
}

/// The endpoint to create a Redmine file
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(setter(strip_option))]
pub struct CreateFile<'a> {
    /// project id or name as it appears in the URL
    #[builder(setter(into))]
    #[serde(skip_serializing)]
    project_id_or_name: Cow<'a, str>,
    /// the token of the uploaded file
    #[builder(setter(into))]
    token: Cow<'a, str>,
    /// the version to attach the file to
    #[builder(default)]
    version_id: Option<u64>,
    /// the filename
    #[builder(setter(into), default)]
    filename: Option<Cow<'a, str>>,
    /// a description for the file
    #[builder(setter(into), default)]
    description: Option<Cow<'a, str>>,
}

impl<'a> CreateFile<'a> {
    /// Create a builder for the endpoint.
    #[must_use]
    pub fn builder() -> CreateFileBuilder<'a> {
        CreateFileBuilder::default()
    }
}

impl Endpoint for CreateFile<'_> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("projects/{}/files.json", self.project_id_or_name).into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::Error> {
        Ok(Some((
            "application/json",
            serde_json::to_vec(&FileWrapper::<CreateFile> {
                file: (*self).to_owned(),
            })?,
        )))
    }
}

/// helper struct for outer layers with a file field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct FileWrapper<T> {
    /// to parse JSON with file key
    pub file: T,
}

/// helper struct for outer layers with a files field holding the inner data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct FilesWrapper<T> {
    /// to parse JSON with files key
    pub files: Vec<T>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::uploads::UploadFile;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tempfile;
    use tracing_test::traced_test;

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_list_project_files_no_pagination() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        crate::api::test_helpers::with_project(&name, |redmine, _id, name| {
            let endpoint = ListProjectFiles::builder()
                .project_id_or_name(name)
                .build()?;
            redmine.json_response_body::<_, FilesWrapper<File>>(&endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_create_file() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        crate::api::test_helpers::with_project(&name, |redmine, _id, name| {
            let mut temp_file = tempfile::NamedTempFile::new()?;
            use std::io::Write;
            write!(temp_file, "test file content")?;
            let upload_endpoint = UploadFile::builder()
                .file(temp_file.path().to_path_buf())
                .content_type("text/plain")
                .build()?;
            let upload: crate::api::uploads::UploadWrapper<crate::api::uploads::FileUploadToken> =
                redmine.json_response_body(&upload_endpoint)?;
            let endpoint = CreateFile::builder()
                .project_id_or_name(name)
                .token(upload.upload.token)
                .build()?;
            redmine.ignore_response_body(&endpoint)?;
            Ok(())
        })?;
        Ok(())
    }

    /// this tests if any of the results contain a field we are not deserializing
    ///
    /// this will only catch fields we missed if they are part of the response but
    /// it is better than nothing
    #[function_name::named]
    #[traced_test]
    #[test]
    fn test_completeness_file_type() -> Result<(), Box<dyn Error>> {
        let name = format!("unittest_{}", function_name!());
        crate::api::test_helpers::with_project(&name, |redmine, _id, name| {
            let endpoint = ListProjectFiles::builder()
                .project_id_or_name(name)
                .build()?;
            let raw_values =
                redmine.json_response_body::<_, FilesWrapper<serde_json::Value>>(&endpoint)?;
            for value in raw_values.files {
                let o: File = serde_json::from_value(value.clone())?;
                let reserialized = serde_json::to_value(o)?;
                assert_eq!(value, reserialized);
            }
            Ok(())
        })?;
        Ok(())
    }
}
