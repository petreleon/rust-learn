use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_teacher_course_workspace::{
    TeacherCoursePublicationSummaryOutput, TeacherCourseWorkspaceOutput,
    TeacherCourseWorkspaceQuery, TeacherCourseWorkspaceStore,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::infra::postgres::learning::{
    teacher_course_dashboard_permissions, teacher_course_dashboard_summary_queries,
    teacher_course_workspace_queries,
};
use crate::infra::postgres::models::course::Course;
use crate::infra::postgres::schema::courses;

pub struct PostgresTeacherCourseWorkspaceStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTeacherCourseWorkspaceStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl TeacherCourseWorkspaceStore for PostgresTeacherCourseWorkspaceStore<'_> {
    fn get_teacher_course_workspace(
        &mut self,
        query: TeacherCourseWorkspaceQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseWorkspaceOutput, TeacherCourseDashboardError>> {
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
            if !permissions.has_teacher_access() {
                return Err(TeacherCourseDashboardError::PermissionDenied(
                    "teaching course access".to_string(),
                ));
            }

            let teacher_roles = teacher_course_workspace_queries::load_actor_course_roles(
                self.conn,
                query.actor_user_id,
                course.id,
            )
            .await?;
            let publication = TeacherCoursePublicationSummaryOutput {
                course_lifecycle_status: course.lifecycle_status.clone(),
                content_publication_status_supported: true,
            };
            let chapters =
                teacher_course_workspace_queries::load_teacher_course_workspace_chapters(
                    self.conn,
                    course.id,
                    &course.lifecycle_status,
                )
                .await?;
            let course =
                teacher_course_dashboard_summary_queries::build_teacher_course_dashboard_item(
                    self.conn,
                    course,
                    permissions,
                )
                .await?;

            Ok(TeacherCourseWorkspaceOutput {
                course,
                teacher_roles,
                publication,
                chapters,
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
