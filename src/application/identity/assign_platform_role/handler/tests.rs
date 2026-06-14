use futures::future::{ready, BoxFuture, FutureExt};

use super::*;

#[derive(Default)]
struct FakeRoleAssignmentStore {
    commands: Vec<AssignPlatformRoleCommand>,
    result: Option<AssignPlatformRoleError>,
}

impl PlatformRoleAssignmentStore for FakeRoleAssignmentStore {
    fn assign_role(
        &mut self,
        command: AssignPlatformRoleCommand,
    ) -> BoxFuture<'_, Result<(), AssignPlatformRoleError>> {
        self.commands.push(command);
        ready(match &self.result {
            Some(error) => Err(error.clone()),
            None => Ok(()),
        })
        .boxed()
    }
}

#[tokio::test]
async fn delegates_assignment_and_returns_notification_context() {
    let mut store = FakeRoleAssignmentStore::default();

    let outcome = assign_platform_role(&mut store, command()).await.unwrap();

    assert_eq!(store.commands, vec![command()]);
    assert_eq!(outcome.target_user_id, 9);
    assert_eq!(outcome.role_name, "USER");
}

#[tokio::test]
async fn returns_store_error_without_outcome() {
    let mut store = FakeRoleAssignmentStore {
        result: Some(AssignPlatformRoleError::HierarchyViolation),
        ..Default::default()
    };

    let error = assign_platform_role(&mut store, command())
        .await
        .unwrap_err();

    assert_eq!(error, AssignPlatformRoleError::HierarchyViolation);
}

fn command() -> AssignPlatformRoleCommand {
    AssignPlatformRoleCommand {
        requester_user_id: 7,
        target_user_id: 9,
        role_name: "USER".to_string(),
    }
}
