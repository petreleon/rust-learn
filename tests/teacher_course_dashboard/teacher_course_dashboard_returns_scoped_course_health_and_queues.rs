use crate::{
    dashboard_list_assertions::*, students_access_assertions::*,
    teacher_course_dashboard_fixture::*, workspace_enrollment_assertions::*,
};

#[actix_web::test]
async fn teacher_course_dashboard_returns_scoped_course_health_and_queues() {
    let fixture = setup_teacher_dashboard_fixture().await;

    assert_teacher_dashboard_list(&fixture).await;
    assert_teacher_workspace(&fixture).await;
    assert_teacher_enrollments(&fixture).await;
    assert_teacher_pending_filter(&fixture).await;
    assert_teacher_students(&fixture).await;
    assert_outsider_teacher_dashboard_access(&fixture).await;
}
