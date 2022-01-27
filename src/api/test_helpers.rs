use std::error::Error;
use tracing::trace;

use crate::api::groups::{test::GROUP_LOCK, CreateGroup, DeleteGroup, Group, GroupWrapper};
use crate::api::projects::{
    test::PROJECT_LOCK, CreateProject, DeleteProject, GetProject, Project, ProjectWrapper,
};

pub fn with_project<F>(name: &str, f: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&crate::api::Redmine, u64, &str) -> Result<(), Box<dyn Error>>,
{
    let _w_projects = PROJECT_LOCK.write();
    dotenv::dotenv()?;
    let redmine = crate::api::Redmine::from_env()?;
    let get_endpoint = GetProject::builder().project_id_or_name(name).build()?;
    let get_result = redmine.json_response_body::<_, ProjectWrapper<Project>>(&get_endpoint);
    trace!("Get result in {} test:\n{:?}", name, get_result);
    if get_result.is_ok() {
        let delete_endpoint = DeleteProject::builder().project_id_or_name(name).build()?;
        redmine.ignore_response_body::<_>(&delete_endpoint)?;
    }
    let create_endpoint = CreateProject::builder()
        .name(format!("Unittest redmine-api {}", name))
        .identifier(name)
        .build()?;
    let ProjectWrapper { project } =
        redmine.json_response_body::<_, ProjectWrapper<Project>>(&create_endpoint)?;
    let _fb = finally_block::finally(|| {
        trace!(%name, "Deleting test project");
        let delete_endpoint = DeleteProject::builder()
            .project_id_or_name(name)
            .build()
            .unwrap_or_else(|_| panic!("Building delete enedpoint for project {} failed", name));
        redmine
            .ignore_response_body::<_>(&delete_endpoint)
            .unwrap_or_else(|_| panic!("Delete project {} failed", name));
    });
    trace!(%name, "Actual test body starts here");
    f(&redmine, project.id, name)?;
    trace!(%name, "Actual test body ends here");
    Ok(())
}

pub fn with_group<F>(name: &str, f: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&crate::api::Redmine, u64, &str) -> Result<(), Box<dyn Error>>,
{
    let _w_groups = GROUP_LOCK.write();
    dotenv::dotenv()?;
    let redmine = crate::api::Redmine::from_env()?;
    let create_endpoint = CreateGroup::builder().name(name).build()?;
    let GroupWrapper { group } =
        redmine.json_response_body::<_, GroupWrapper<Group>>(&create_endpoint)?;
    let id = group.id;
    let _fb = finally_block::finally(|| {
        trace!(%name, "Deleting test group");
        let delete_endpoint = DeleteGroup::builder()
            .id(id)
            .build()
            .unwrap_or_else(|_| panic!("Building delete endpoint for group {} failed", name));
        redmine
            .ignore_response_body::<_>(&delete_endpoint)
            .unwrap_or_else(|_| panic!("Delete group {} failed", name));
    });
    trace!(%name, "Actual test body starts here");
    f(&redmine, id, name)?;
    trace!(%name, "Actual test body ends here");
    Ok(())
}
