use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewDataset, TeacherApplicationPlatformReviewError,
    TeacherApplicationPlatformReviewStore,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::teacher_applications;
use crate::infra::postgres::teacher_applications::teacher_application_permissions::has_platform_permission;
use crate::infra::postgres::teacher_applications::teacher_application_platform_review_audit::application_summary;
use crate::infra::postgres::teacher_applications::teacher_application_platform_review_context::build_context;
use crate::infra::postgres::teacher_applications::teacher_application_platform_review_mappers::platform_review_item;
use crate::models::teacher_application::TeacherApplication;

pub struct PostgresTeacherApplicationPlatformReviewStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTeacherApplicationPlatformReviewStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl TeacherApplicationPlatformReviewStore for PostgresTeacherApplicationPlatformReviewStore<'_> {
    fn can_review_teacher_applications(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationPlatformReviewError>> {
        self.has_permission(actor_user_id, Permissions::REVIEW_TEACHER_APPLICATIONS)
    }

    fn can_approve_teacher_application(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationPlatformReviewError>> {
        self.has_permission(actor_user_id, Permissions::APPROVE_TEACHER_APPLICATION)
    }

    fn can_reject_teacher_application(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationPlatformReviewError>> {
        self.has_permission(actor_user_id, Permissions::REJECT_TEACHER_APPLICATION)
    }

    fn list_applications(
        &mut self,
    ) -> BoxFuture<
        '_,
        Result<TeacherApplicationPlatformReviewDataset, TeacherApplicationPlatformReviewError>,
    > {
        async move { list_applications(self.conn).await }.boxed()
    }
}

impl PostgresTeacherApplicationPlatformReviewStore<'_> {
    fn has_permission(
        &mut self,
        actor_user_id: i32,
        permission: Permissions,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationPlatformReviewError>> {
        async move {
            has_platform_permission(self.conn, actor_user_id, permission)
                .await
                .map_err(map_error)
        }
        .boxed()
    }
}

async fn list_applications(
    conn: &mut AsyncPgConnection,
) -> Result<TeacherApplicationPlatformReviewDataset, TeacherApplicationPlatformReviewError> {
    let applications = teacher_applications::table
        .order(teacher_applications::created_at.desc())
        .then_order_by(teacher_applications::id.desc())
        .load::<TeacherApplication>(conn)
        .await
        .map_err(map_error)?;
    let summary = application_summary(&applications);
    let context = build_context(conn, &applications).await?;
    let applications = applications
        .iter()
        .map(|application| platform_review_item(application, &context))
        .collect();

    Ok(TeacherApplicationPlatformReviewDataset {
        applications,
        summary,
    })
}

fn map_error(error: diesel::result::Error) -> TeacherApplicationPlatformReviewError {
    TeacherApplicationPlatformReviewError::Database(error.to_string())
}
