use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewDataset, TeacherApplicationPlatformReviewError,
    TeacherApplicationPlatformReviewStore,
};
use crate::db::schema::teacher_applications;
use crate::infra::postgres::access_control::permission_checks;
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
    fn list_applications(
        &mut self,
    ) -> BoxFuture<
        '_,
        Result<TeacherApplicationPlatformReviewDataset, TeacherApplicationPlatformReviewError>,
    > {
        async move { list_applications(self.conn).await }.boxed()
    }
}

impl AccessDecisionStore for PostgresTeacherApplicationPlatformReviewStore<'_> {
    type Error = TeacherApplicationPlatformReviewError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationPlatformReviewError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
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
