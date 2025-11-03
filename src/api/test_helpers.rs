//! Helpers for testing, mainly for setup and teardown
use std::error::Error;
use tracing::trace;

use crate::api::groups::{CreateGroup, DeleteGroup, Group, GroupWrapper, test::GROUP_LOCK};
use crate::api::projects::{
    CreateProject, DeleteProject, GetProject, Project, ProjectWrapper, test::PROJECT_LOCK,
};

/// Create a project for testing and then call the function with the project
/// id and name and then cleans up the project in both the error and the ok
/// cases with a finally block
///
/// # Errors
///
/// This returns an error when it can not create the Redmine object from
/// environment variables, can not create the project or when the function
/// passed in fails
///
/// # Panics
///
/// This panics if deleting the project fails since we can not really return
/// errors from the finally block
pub fn with_project<F>(name: &str, f: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&crate::api::Redmine, u64, &str) -> Result<(), Box<dyn Error>>,
{
    let _w_projects = PROJECT_LOCK.blocking_write();
    dotenvy::dotenv()?;
    let redmine = crate::api::Redmine::from_env(
        reqwest::blocking::Client::builder()
            .use_rustls_tls()
            .build()?,
    )?;
    let get_endpoint = GetProject::builder().project_id_or_name(name).build()?;
    let get_result = redmine.json_response_body::<_, ProjectWrapper<Project>>(&get_endpoint);
    trace!("Get result in {} test:\n{:?}", name, get_result);
    if get_result.is_ok() {
        let delete_endpoint = DeleteProject::builder().project_id_or_name(name).build()?;
        redmine.ignore_response_body::<_>(&delete_endpoint)?;
    }
    let create_endpoint = CreateProject::builder()
        .name(format!("Unittest redmine-api {name}"))
        .identifier(name)
        .build()?;
    let ProjectWrapper { project } =
        redmine.json_response_body::<_, ProjectWrapper<Project>>(&create_endpoint)?;
    let _fb = finally_block::finally(|| {
        trace!(%name, "Deleting test project");
        let delete_endpoint = DeleteProject::builder()
            .project_id_or_name(name)
            .build()
            .unwrap_or_else(|_| panic!("Building delete endpoint for project {name} failed"));
        redmine
            .ignore_response_body::<_>(&delete_endpoint)
            .unwrap_or_else(|_| panic!("Delete project {name} failed"));
    });
    trace!(%name, "Actual test body starts here");
    f(&redmine, project.id, name)?;
    trace!(%name, "Actual test body ends here");
    Ok(())
}

/// Creates a group for testing, calls the function with the group id and name
/// and then cleans up the group in both the ok and error case with a finally
/// block
///
/// # Errors
///
/// This returns an error when it can not create the Redmine object from
/// environment variables, can not create the group or when the function
/// passed in fails
///
/// # Panics
///
/// This panics if deleting the group fails since we can not really return
/// errors from the finally block
pub fn with_group<F>(name: &str, f: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&crate::api::Redmine, u64, &str) -> Result<(), Box<dyn Error>>,
{
    let _w_groups = GROUP_LOCK.blocking_write();
    dotenvy::dotenv()?;
    let redmine = crate::api::Redmine::from_env(
        reqwest::blocking::Client::builder()
            .use_rustls_tls()
            .build()?,
    )?;
    let create_endpoint = CreateGroup::builder().name(name).build()?;
    let GroupWrapper { group } =
        redmine.json_response_body::<_, GroupWrapper<Group>>(&create_endpoint)?;
    let id = group.id;
    let _fb = finally_block::finally(|| {
        trace!(%name, "Deleting test group");
        let delete_endpoint = DeleteGroup::builder()
            .id(id)
            .build()
            .unwrap_or_else(|_| panic!("Building delete endpoint for group {name} failed"));
        redmine
            .ignore_response_body::<_>(&delete_endpoint)
            .unwrap_or_else(|_| panic!("Delete group {name} failed"));
    });
    trace!(%name, "Actual test body starts here");
    f(&redmine, id, name)?;
    trace!(%name, "Actual test body ends here");
    Ok(())
}
