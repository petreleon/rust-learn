use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_teacher_course_enrollment_workspace::{
    TeacherCourseEnrollmentWorkspaceOutput, TeacherCourseEnrollmentWorkspaceQuery,
    TeacherCourseEnrollmentWorkspaceStore,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::db::schema::courses;
use crate::infra::postgres::learning::{
    teacher_course_dashboard_permissions, teacher_course_dashboard_summary_queries,
    teacher_course_join_request_queries, teacher_course_reward_eligibility_queries,
    teacher_course_roster_queries, teacher_course_workspace_queries,
};
use crate::infra::postgres::models::course::Course;

pub struct PostgresTeacherCourseEnrollmentWorkspaceStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTeacherCourseEnrollmentWorkspaceStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl TeacherCourseEnrollmentWorkspaceStore for PostgresTeacherCourseEnrollmentWorkspaceStore<'_> {
    fn get_teacher_course_enrollment_workspace(
        &mut self,
        query: TeacherCourseEnrollmentWorkspaceQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseEnrollmentWorkspaceOutput, TeacherCourseDashboardError>>
    {
        async move {
            let course = courses::table
                .find(query.course_id)
                .first::<Course>(self.conn)
                .await
                .map_err(map_dashboard_error)?;
            let permissions =
                teacher_course_dashboard_permissions::build_teacher_course_permissions(
                    self.conn,
                    query.actor_user_id,
                    course.id,
                )
                .await?;
            if !permissions.can_manage_enrollments {
                return Err(TeacherCourseDashboardError::PermissionDenied(
                    "course enrollment management".to_string(),
                ));
            }

            let can_manage_enrollments = permissions.can_manage_enrollments;
            let teacher_roles = teacher_course_workspace_queries::load_actor_course_roles(
                self.conn,
                query.actor_user_id,
                course.id,
            )
            .await?;
            let join_requests =
                teacher_course_join_request_queries::load_teacher_course_join_request_page(
                    self.conn,
                    course.id,
                    &query,
                    can_manage_enrollments,
                )
                .await?;
            let roster = teacher_course_roster_queries::load_teacher_course_roster_page(
                self.conn,
                course.id,
                can_manage_enrollments,
            )
            .await?;
            let reward_eligibility =
                teacher_course_reward_eligibility_queries::load_teacher_course_reward_eligibility_summary(
                    self.conn, course.id,
                )
                .await?;
            let course =
                teacher_course_dashboard_summary_queries::build_teacher_course_dashboard_item(
                    self.conn,
                    course,
                    permissions,
                )
                .await?;

            Ok(TeacherCourseEnrollmentWorkspaceOutput {
                course,
                teacher_roles,
                join_requests,
                roster,
                progress_supported: true,
                reward_eligibility_supported: reward_eligibility.supported,
                reward_eligibility,
            })
        }
        .boxed()
    }
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
