use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_teacher_course_students::{
    TeacherCourseStudentProgressItemOutput, TeacherCourseStudentsOutput,
    TeacherCourseStudentsQuery, TeacherCourseStudentsStore,
};
use crate::application::learning::teacher_course_dashboard::{
    TeacherCourseDashboardError, TeacherCoursePermissionSummaryOutput,
};
use crate::db::schema::courses;
use crate::infra::postgres::learning::{
    teacher_course_dashboard_permissions, teacher_course_dashboard_summary_queries,
    teacher_course_reward_eligibility_queries, teacher_course_roster_queries,
    teacher_course_workspace_queries, teacher_student_progress_queries,
    teacher_student_reward_progress_queries,
};
use crate::infra::postgres::models::course::Course;

pub struct PostgresTeacherCourseStudentsStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTeacherCourseStudentsStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl TeacherCourseStudentsStore for PostgresTeacherCourseStudentsStore<'_> {
    fn get_teacher_course_students(
        &mut self,
        query: TeacherCourseStudentsQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseStudentsOutput, TeacherCourseDashboardError>> {
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
            if !can_view_students(&permissions) {
                return Err(TeacherCourseDashboardError::PermissionDenied(
                    "course student progress".to_string(),
                ));
            }

            let can_manage_enrollments = permissions.can_manage_enrollments;
            let teacher_roles = teacher_course_workspace_queries::load_actor_course_roles(
                self.conn,
                query.actor_user_id,
                course.id,
            )
            .await?;
            let total_content_count =
                teacher_student_progress_queries::load_course_content_count(self.conn, course.id)
                    .await?;
            let reward_eligibility =
                teacher_course_reward_eligibility_queries::load_teacher_course_reward_eligibility_summary(
                    self.conn, course.id,
                )
                .await?;
            let roster = teacher_course_roster_queries::load_teacher_course_roster_page(
                self.conn,
                course.id,
                can_manage_enrollments,
            )
            .await?;
            let mut students = Vec::with_capacity(roster.learners.len());
            for learner in roster.learners {
                let rewards =
                    teacher_student_reward_progress_queries::load_teacher_student_reward_progress(
                        self.conn,
                        course.id,
                        learner.user.id,
                    )
                    .await?;
                let progress = teacher_student_progress_queries::load_teacher_student_lesson_progress(
                    self.conn,
                    course.id,
                    learner.user.id,
                    total_content_count,
                )
                .await?;
                let reward_eligibility =
                    teacher_course_reward_eligibility_queries::teacher_student_reward_eligibility_from_count(
                        &reward_eligibility,
                        rewards.reward_candidate_count,
                    );
                students.push(TeacherCourseStudentProgressItemOutput {
                    user: learner.user,
                    roles: learner.roles,
                    access_state: learner.access_state,
                    latest_join_request_status: learner.latest_join_request_status,
                    progress,
                    reward_eligibility,
                    rewards,
                });
            }

            let total = students.len() as i64;
            let course =
                teacher_course_dashboard_summary_queries::build_teacher_course_dashboard_item(
                    self.conn,
                    course,
                    permissions,
                )
                .await?;
            let reward_eligibility_supported = reward_eligibility.supported;

            Ok(TeacherCourseStudentsOutput {
                course,
                teacher_roles,
                students,
                total,
                progress_supported: true,
                reward_eligibility_supported,
                reward_eligibility,
                reward_evidence_supported: true,
            })
        }
        .boxed()
    }
}

fn can_view_students(permissions: &TeacherCoursePermissionSummaryOutput) -> bool {
    permissions.can_manage_enrollments
        || permissions.can_view_reward_candidates
        || permissions.can_approve_reward_candidates
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
