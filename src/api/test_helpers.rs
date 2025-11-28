use crate::api::RedmineAsync;
use std::error::Error;

use tracing::trace;

use crate::api::groups::{CreateGroup, DeleteGroup, Group, GroupWrapper, test::GROUP_LOCK};
use crate::api::projects::{
    CreateProject, DeleteProject, GetProject, Project, ProjectWrapper, test::PROJECT_LOCK,
};
use crate::api::testcontainers_helpers;

/// A helper function to dispatch a test closure to all configured Redmine
/// instances for sync tests
///
/// # Errors
///
/// Returns an error if the Redmine test environment setup fails or if the provided closure `f` returns an error.
pub fn with_redmine<F>(current_span: tracing::Span, f: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(&crate::api::Redmine) -> Result<(), Box<dyn Error>>,
{
    if std::env::var("REDMINE_TEST_MODE").unwrap_or_default() == "testcontainers" {
        let versions = std::env::var("REDMINE_VERSIONS").unwrap_or_else(|_| "6.1.0".to_string());
        for version in versions.split(',') {
            testcontainers_helpers::with_redmine_container(
                version,
                current_span.clone(),
                |redmine, _| f(redmine),
            )?;
        }
    } else {
        dotenvy::dotenv()?;
        let redmine = crate::api::Redmine::from_env(
            reqwest::blocking::Client::builder()
                .use_rustls_tls()
                .build()?,
        )?;
        f(&redmine)?;
    }
    Ok(())
}

/// A helper function to dispatch a test closure to all configured Redmine
/// instances for sync tests
///
/// # Errors
///
/// Returns an error if the Redmine test environment setup fails or if the provided asynchronous closure `f` returns an error.
pub async fn with_redmine_async<F>(current_span: tracing::Span, f: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(
            &std::sync::Arc<RedmineAsync>,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<(), Box<dyn Error>>> + Send + 'static>,
        >
        + Send
        + Sync
        + 'static
        + Clone,
{
    if std::env::var("REDMINE_TEST_MODE").unwrap_or_default() == "testcontainers" {
        let versions = std::env::var("REDMINE_VERSIONS").unwrap_or_else(|_| "6.1.0".to_string());
        for version in versions.split(',') {
            let f_clone = f.clone();
            testcontainers_helpers::with_redmine_container_async(
                version,
                current_span.clone(),
                move |redmine, _| {
                    let redmine_cloned = redmine.clone();
                    Box::pin(async move { f_clone(&redmine_cloned).await })
                },
            )
            .await?;
        }
    } else {
        dotenvy::dotenv()?;
        let redmine = crate::api::RedmineAsync::from_env(
            reqwest::Client::builder().use_rustls_tls().build()?,
        )?;
        f(&redmine).await?;
    }
    Ok(())
}

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
    F: Fn(&crate::api::Redmine, u64, &str) -> Result<(), Box<dyn Error>>,
{
    let current_span = tracing::Span::current();
    with_redmine(current_span, |redmine| {
        if std::env::var("REDMINE_TEST_MODE").unwrap_or_default() != "testcontainers" {
            let _w_projects = PROJECT_LOCK.blocking_write();
        }
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
            .enabled_module_names(vec![
                "files".into(),
                "issue_tracking".into(),
                "news".into(),
                "wiki".into(),
            ])
            .build()?;
        let ProjectWrapper { project } =
            redmine.json_response_body::<_, ProjectWrapper<Project>>(&create_endpoint)?;

        // Add current user to the project with a role that has manage_files permission
        let current_user_endpoint = crate::api::my_account::GetMyAccount::builder().build()?;
        let current_user_wrapper = redmine.json_response_body::<_, crate::api::users::UserWrapper<
            crate::api::my_account::MyAccount,
        >>(&current_user_endpoint)?;
        let current_user_id = current_user_wrapper.user.id;

        let list_roles_endpoint = crate::api::roles::ListRoles::builder().build()?;
        let roles_wrapper = redmine.json_response_body::<_, crate::api::roles::RolesWrapper<
            crate::api::roles::RoleEssentials,
        >>(&list_roles_endpoint)?;

        // Find a role that is likely to have manage_files permission (e.g., Manager)
        let manager_role = roles_wrapper
            .roles
            .into_iter()
            .find(|role| role.name.contains("Manager"))
            .ok_or("No Manager role found")?;

        let create_membership_endpoint =
            crate::api::project_memberships::CreateProjectMembership::builder()
                .project_id_or_name(project.id.to_string())
                .user_ids(vec![current_user_id])
                .role_ids(vec![manager_role.id])
                .build()?;
        redmine.ignore_response_body::<_>(&create_membership_endpoint)?;

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
        f(redmine, project.id, name)?;
        trace!(%name, "Actual test body ends here");
        Ok(())
    })
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
    F: Fn(&crate::api::Redmine, u64, &str) -> Result<(), Box<dyn Error>>,
{
    let current_span = tracing::Span::current();
    with_redmine(current_span, |redmine| {
        if std::env::var("REDMINE_TEST_MODE").unwrap_or_default() != "testcontainers" {
            let _w_groups = GROUP_LOCK.blocking_write();
        }
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
        f(redmine, id, name)?;
        trace!(%name, "Actual test body ends here");
        Ok(())
    })
}
