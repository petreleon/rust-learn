use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::assign_course_role::{
    assign_course_role, CourseRoleAssignmentCommand, CourseRoleAssignmentError,
    CourseRoleAssignmentStore,
};

#[derive(Default)]
struct FakeCourseRoleAssignmentStore {
    actor_level: Option<i32>,
    target_level: Option<i32>,
    role_id: Option<i32>,
    role_level: Option<i32>,
    assigned: Option<(i32, i32, i32)>,
}

impl CourseRoleAssignmentStore for FakeCourseRoleAssignmentStore {
    fn actor_min_level(
        &mut self,
        _actor_user_id: i32,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>> {
        let level = self.actor_level;
        async move { Ok(level) }.boxed()
    }

    fn target_min_level(
        &mut self,
        _target_user_id: i32,
        _course_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>> {
        let level = self.target_level;
        async move { Ok(level) }.boxed()
    }

    fn role_id_by_name(
        &mut self,
        _role_name: &str,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>> {
        let role_id = self.role_id;
        async move { Ok(role_id) }.boxed()
    }

    fn role_hierarchy_level(
        &mut self,
        _role_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>> {
        let level = self.role_level;
        async move { Ok(level) }.boxed()
    }

    fn assign_role(
        &mut self,
        target_user_id: i32,
        course_id: i32,
        role_id: i32,
    ) -> BoxFuture<'_, Result<(), CourseRoleAssignmentError>> {
        self.assigned = Some((target_user_id, course_id, role_id));
        async move { Ok(()) }.boxed()
    }
}

#[tokio::test]
async fn assign_course_role_allows_actor_to_assign_lower_rank() {
    let mut store = FakeCourseRoleAssignmentStore {
        actor_level: Some(1),
        role_id: Some(23),
        role_level: Some(2),
        ..Default::default()
    };

    let output = assign_course_role(&mut store, command("STUDENT"))
        .await
        .expect("higher actor should assign lower role");

    assert_eq!(output.role_name, "STUDENT");
    assert_eq!(store.assigned, Some((9, 17, 23)));
}

#[tokio::test]
async fn assign_course_role_rejects_equal_or_higher_target_role() {
    let mut store = FakeCourseRoleAssignmentStore {
        actor_level: Some(2),
        role_id: Some(7),
        role_level: Some(1),
        ..Default::default()
    };

    let error = assign_course_role(&mut store, command("TEACHER"))
        .await
        .expect_err("actor cannot assign role above their rank");

    assert_eq!(error, CourseRoleAssignmentError::HierarchyViolation);
    assert_eq!(store.assigned, None);
}

#[tokio::test]
async fn assign_course_role_rejects_target_with_equal_or_higher_rank() {
    let mut store = FakeCourseRoleAssignmentStore {
        actor_level: Some(1),
        target_level: Some(1),
        role_id: Some(23),
        role_level: Some(2),
        ..Default::default()
    };

    let error = assign_course_role(&mut store, command("STUDENT"))
        .await
        .expect_err("actor cannot modify peer or higher target");

    assert_eq!(error, CourseRoleAssignmentError::HierarchyViolation);
    assert_eq!(store.assigned, None);
}

fn command(role_name: &str) -> CourseRoleAssignmentCommand {
    CourseRoleAssignmentCommand {
        actor_user_id: 5,
        course_id: 17,
        target_user_id: 9,
        role_name: role_name.to_string(),
    }
}
