use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::{
    list_applications::{
        TeacherApplicationListError, TeacherApplicationListFilter, TeacherApplicationListStore,
    },
    TeacherApplicationOutput,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::teacher_applications;
use crate::models::teacher_application::TeacherApplication;
use crate::repositories::platform_repository::user_permission_platform_request;

pub struct PostgresTeacherApplicationListStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTeacherApplicationListStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl TeacherApplicationListStore for PostgresTeacherApplicationListStore<'_> {
    fn can_review_teacher_applications(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationListError>> {
        async move {
            user_permission_platform_request(
                self.conn,
                actor_user_id,
                &Permissions::REVIEW_TEACHER_APPLICATIONS.to_string(),
            )
            .await
            .map_err(map_error)
        }
        .boxed()
    }

    fn list_applications(
        &mut self,
        filter: TeacherApplicationListFilter,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationOutput>, TeacherApplicationListError>> {
        async move { list_applications(self.conn, filter).await }.boxed()
    }
}

async fn list_applications(
    conn: &mut AsyncPgConnection,
    filter: TeacherApplicationListFilter,
) -> Result<Vec<TeacherApplicationOutput>, TeacherApplicationListError> {
    let mut query = teacher_applications::table.into_boxed();
    if let Some(status) = filter.status {
        query = query.filter(teacher_applications::status.eq(status));
    }
    if let Some(applicant_user_id) = filter.applicant_user_id {
        query = query.filter(teacher_applications::applicant_user_id.eq(applicant_user_id));
    }
    if let Some(organization_sponsor_id) = filter.organization_sponsor_id {
        query = query.filter(
            teacher_applications::organization_sponsor_id.eq(Some(organization_sponsor_id)),
        );
    }

    query
        .order(teacher_applications::created_at.desc())
        .limit(filter.limit.unwrap_or(25).clamp(1, 100))
        .offset(filter.offset.unwrap_or(0).max(0))
        .load::<TeacherApplication>(conn)
        .await
        .map(|applications| applications.into_iter().map(Into::into).collect())
        .map_err(map_error)
}

fn map_error(error: diesel::result::Error) -> TeacherApplicationListError {
    TeacherApplicationListError::Database(error.to_string())
}
