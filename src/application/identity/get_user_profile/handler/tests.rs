use chrono::NaiveDate;
use futures::future::{ready, BoxFuture, FutureExt};

use super::*;
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::identity::list_users::ListUsersQuery;

#[derive(Default)]
struct FakeUserProfileStore {
    can_view: bool,
    find_calls: Vec<i32>,
    permission_checks: Vec<i32>,
}

impl UserProfileStore for FakeUserProfileStore {
    fn list_users(
        &mut self,
        _: ListUsersQuery,
    ) -> BoxFuture<'_, Result<Vec<UserProfileOutput>, UserProfileError>> {
        ready(Ok(Vec::new())).boxed()
    }

    fn find_user(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<UserProfileOutput, UserProfileError>> {
        self.find_calls.push(user_id);
        ready(Ok(profile(user_id))).boxed()
    }
}

impl AccessDecisionStore for FakeUserProfileStore {
    type Error = UserProfileError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, UserProfileError>> {
        assert_eq!(action.permission_name(), VIEW_USER);
        assert!(matches!(scope, AccessScope::Platform(_)));
        self.permission_checks.push(actor.user_id);
        ready(Ok(self.can_view)).boxed()
    }
}

#[tokio::test]
async fn self_read_skips_permission_check() {
    let mut store = FakeUserProfileStore::default();

    let profile = get_user_profile(&mut store, command(7, 7)).await.unwrap();

    assert_eq!(profile.id, 7);
    assert_eq!(store.permission_checks, Vec::<i32>::new());
    assert_eq!(store.find_calls, vec![7]);
}

#[tokio::test]
async fn other_user_read_requires_view_user_permission() {
    let mut store = FakeUserProfileStore::default();

    let error = get_user_profile(&mut store, command(7, 9))
        .await
        .unwrap_err();

    assert_eq!(error, UserProfileError::Forbidden);
    assert_eq!(store.permission_checks, vec![7]);
    assert_eq!(store.find_calls, Vec::<i32>::new());
}

#[tokio::test]
async fn permission_allows_other_user_read() {
    let mut store = FakeUserProfileStore {
        can_view: true,
        ..Default::default()
    };

    let profile = get_user_profile(&mut store, command(7, 9)).await.unwrap();

    assert_eq!(profile.id, 9);
    assert_eq!(store.permission_checks, vec![7]);
    assert_eq!(store.find_calls, vec![9]);
}

fn command(requester_user_id: i32, target_user_id: i32) -> GetUserProfileCommand {
    GetUserProfileCommand {
        requester_user_id,
        target_user_id,
    }
}

fn profile(id: i32) -> UserProfileOutput {
    UserProfileOutput {
        id,
        name: format!("User {id}"),
        email: format!("user{id}@example.com"),
        date_of_birth: None,
        created_at: NaiveDate::from_ymd_opt(2026, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        kyc_verified: false,
        email_verified: true,
    }
}
