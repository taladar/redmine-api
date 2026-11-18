//! Helpers for testing, mainly for setup and teardown
use std::error::Error;
use tracing::trace;

use crate::api::groups::{
    CreateGroup, DeleteGroup, Group, GroupEssentials, GroupWrapper, GroupsWrapper, ListGroups,
    test::GROUP_LOCK,
};
use crate::api::projects::{
    CreateProject, DeleteProject, GetProject, Project, ProjectWrapper, test::PROJECT_LOCK,
};

/// Prepares the environment for a test, loads the .env file (if there is
/// one) and makes sure the local Redmine test container is running if one is
/// configured (see [`crate::api::test_container`])
///
/// # Errors
///
/// This returns an error if the .env file can not be parsed or the test
/// container can not be set up
pub fn setup() -> Result<(), Box<dyn Error>> {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(e) if e.not_found() => trace!("No .env file found"),
        Err(e) => return Err(e.into()),
    }
    crate::api::test_container::ensure_container()
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
    F: FnOnce(&crate::api::Redmine, u64, &str) -> Result<(), Box<dyn Error>>,
{
    let _w_projects = PROJECT_LOCK.blocking_write();
    setup()?;
    let redmine = crate::api::Redmine::from_env(
        reqwest::blocking::Client::builder()
            .tls_backend_rustls()
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
        .enabled_module_names(vec![
            "files".into(),
            "issue_tracking".into(),
            "news".into(),
            "time_tracking".into(),
            "wiki".into(),
        ])
        .build()?;
    let ProjectWrapper { project } =
        redmine.json_response_body::<_, ProjectWrapper<Project>>(&create_endpoint)?;

    // Add current user to the project with a role that has manage_files permission
    let current_user_id = current_user_id(&redmine)?;

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
    setup()?;
    let redmine = crate::api::Redmine::from_env(
        reqwest::blocking::Client::builder()
            .tls_backend_rustls()
            .build()?,
    )?;
    // remove left-overs from a previous test run that failed to clean up
    let list_endpoint = ListGroups::builder().build()?;
    let GroupsWrapper { groups } =
        redmine.json_response_body::<_, GroupsWrapper<GroupEssentials>>(&list_endpoint)?;
    for group in groups.into_iter().filter(|g| g.name == name) {
        trace!(%name, id = group.id, "Deleting left-over test group");
        let delete_endpoint = DeleteGroup::builder().id(group.id).build()?;
        redmine.ignore_response_body::<_>(&delete_endpoint)?;
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
    f(&redmine, id, name)?;
    trace!(%name, "Actual test body ends here");
    Ok(())
}

/// Returns the id of the user the API key belongs to
///
/// # Errors
///
/// This returns an error when the current user can not be fetched
pub fn current_user_id(redmine: &crate::api::Redmine) -> Result<u64, Box<dyn Error>> {
    let endpoint = crate::api::my_account::GetMyAccount::builder().build()?;
    let crate::api::users::UserWrapper { user } = redmine
        .json_response_body::<_, crate::api::users::UserWrapper<crate::api::my_account::MyAccount>>(
            &endpoint,
        )?;
    Ok(user.id)
}

/// Creates an issue with the given subject in the given project for testing
///
/// The issue is not deleted explicitly, it is deleted along with the project
/// created by [`with_project`]
///
/// # Errors
///
/// This returns an error when it can not create the issue
pub fn create_issue(
    redmine: &crate::api::Redmine,
    project_id: u64,
    subject: &str,
) -> Result<crate::api::issues::Issue, Box<dyn Error>> {
    use crate::api::issues::{CreateIssue, Issue, IssueWrapper};
    let create_endpoint = CreateIssue::builder()
        .project_id(project_id)
        .subject(subject)
        .build()?;
    let IssueWrapper { issue } =
        redmine.json_response_body::<_, IssueWrapper<Issue>>(&create_endpoint)?;
    Ok(issue)
}

/// A Redmine version as declared in the `REDMINE_VERSION` environment
/// variable (e.g. `6.1.2` or `7.0`), used by tests whose expected behaviour
/// depends on the version of the Redmine instance they run against
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RedmineVersion {
    /// major version
    pub major: u64,
    /// minor version
    pub minor: u64,
    /// patch version
    pub patch: u64,
}

impl RedmineVersion {
    /// create a new version for comparisons
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl std::fmt::Display for RedmineVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl std::str::FromStr for RedmineVersion {
    type Err = String;

    /// parses `major[.minor[.patch]]`, ignoring any suffix after the patch
    /// version (e.g. `6.1.2.stable`)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split('.');
        let mut next = |name: &str, required: bool| -> Result<u64, String> {
            match parts.next() {
                Some(part) => part
                    .parse()
                    .map_err(|e| format!("invalid {name} version {part:?} in {s:?}: {e}")),
                None if required => Err(format!("missing {name} version in {s:?}")),
                None => Ok(0),
            }
        };
        Ok(Self {
            major: next("major", true)?,
            minor: next("minor", false)?,
            patch: next("patch", false)?,
        })
    }
}

/// Returns the version of the Redmine instance the tests run against as
/// declared in the `REDMINE_VERSION` environment variable (or `.env` file)
///
/// # Errors
///
/// This returns an error if the variable is not set or can not be parsed
pub fn redmine_version() -> Result<RedmineVersion, Box<dyn Error>> {
    // a missing .env file is fine if the variables are set in the environment
    if let Err(e) = dotenvy::dotenv() {
        trace!("Could not load .env file: {e}");
    }
    let version = std::env::var("REDMINE_VERSION").map_err(|e| {
        format!(
            "REDMINE_VERSION needs to be set to the version of the Redmine instance at \
             REDMINE_URL (e.g. 6.1.2) for version dependent tests: {e}"
        )
    })?;
    Ok(version.parse()?)
}

#[cfg(test)]
mod test {
    use super::RedmineVersion;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_redmine_version() -> Result<(), String> {
        assert_eq!(
            "6.1.2".parse::<RedmineVersion>()?,
            RedmineVersion::new(6, 1, 2)
        );
        assert_eq!(
            "7.0".parse::<RedmineVersion>()?,
            RedmineVersion::new(7, 0, 0)
        );
        assert_eq!("7".parse::<RedmineVersion>()?, RedmineVersion::new(7, 0, 0));
        assert_eq!(
            " 6.1.2.stable ".parse::<RedmineVersion>()?,
            RedmineVersion::new(6, 1, 2)
        );
        for invalid in ["", "seven", "7.x"] {
            if let Ok(version) = invalid.parse::<RedmineVersion>() {
                return Err(format!("{invalid:?} parsed as {version}"));
            }
        }
        assert!(RedmineVersion::new(6, 1, 2) < RedmineVersion::new(7, 0, 0));
        Ok(())
    }
}
