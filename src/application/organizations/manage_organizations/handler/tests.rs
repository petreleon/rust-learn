use futures::future::{ready, BoxFuture, FutureExt};

use super::{create_organization, delete_organization, update_organization};
use crate::application::organizations::manage_organizations::{
    OrganizationCreateCommand, OrganizationManagementError, OrganizationManagementStore,
    OrganizationOutput, OrganizationUpdateCommand,
};

#[tokio::test]
async fn create_organization_delegates_command_to_store() {
    let mut store = FakeOrganizationManagementStore::default();

    let output = create_organization(
        &mut store,
        OrganizationCreateCommand {
            name: "Rust Guild".to_string(),
            website_link: Some("https://rust.example".to_string()),
            profile_url: None,
            course_ids: Some(vec![5, 7]),
        },
    )
    .await
    .expect("create should succeed");

    assert_eq!(output.name, "Rust Guild");
    assert_eq!(store.created_course_ids, Some(vec![5, 7]));
}

#[tokio::test]
async fn update_organization_delegates_command_to_store() {
    let mut store = FakeOrganizationManagementStore::default();

    update_organization(
        &mut store,
        OrganizationUpdateCommand {
            organization_id: 12,
            name: Some("Updated".to_string()),
            website_link: None,
            profile_url: Some("https://cdn.example/profile.png".to_string()),
        },
    )
    .await
    .expect("update should succeed");

    assert_eq!(store.updated_id, Some(12));
}

#[tokio::test]
async fn delete_organization_maps_zero_rows_to_not_found() {
    let mut store = FakeOrganizationManagementStore {
        delete_result: false,
        ..Default::default()
    };

    let error = delete_organization(&mut store, 44)
        .await
        .expect_err("zero deleted rows should map to not found");

    assert_eq!(error, OrganizationManagementError::NotFound);
}

#[derive(Default)]
struct FakeOrganizationManagementStore {
    created_course_ids: Option<Vec<i32>>,
    updated_id: Option<i32>,
    delete_result: bool,
}

impl OrganizationManagementStore for FakeOrganizationManagementStore {
    fn list_organizations(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<OrganizationOutput>, OrganizationManagementError>> {
        ready(Ok(Vec::new())).boxed()
    }

    fn get_organization(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        ready(Ok(output(organization_id, "Existing"))).boxed()
    }

    fn create_organization(
        &mut self,
        command: OrganizationCreateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        self.created_course_ids = command.course_ids.clone();
        ready(Ok(output(1, &command.name))).boxed()
    }

    fn update_organization(
        &mut self,
        command: OrganizationUpdateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        self.updated_id = Some(command.organization_id);
        ready(Ok(output(command.organization_id, "Updated"))).boxed()
    }

    fn delete_organization(
        &mut self,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationManagementError>> {
        ready(Ok(self.delete_result)).boxed()
    }
}

fn output(id: i32, name: &str) -> OrganizationOutput {
    OrganizationOutput {
        id,
        name: name.to_string(),
        website_link: None,
        profile_url: None,
    }
}
