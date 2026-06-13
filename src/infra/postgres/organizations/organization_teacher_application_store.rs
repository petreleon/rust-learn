use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationDataset, OrganizationTeacherApplicationListError,
    OrganizationTeacherApplicationListStore, OrganizationTeacherApplicationOrganizationOutput,
    OrganizationTeacherApplicationPermissionsOutput,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{organizations, teacher_applications};
use crate::infra::postgres::organizations::organization_teacher_application_audit_context::teacher_application_summary;
use crate::infra::postgres::organizations::organization_teacher_application_context::build_context;
use crate::infra::postgres::organizations::organization_teacher_application_mappers::organization_teacher_application_item;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;

pub struct PostgresOrganizationTeacherApplicationStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationTeacherApplicationStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationTeacherApplicationListStore for PostgresOrganizationTeacherApplicationStore<'_> {
    fn organization(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<
            OrganizationTeacherApplicationOrganizationOutput,
            OrganizationTeacherApplicationListError,
        >,
    > {
        async move {
            organizations::table
                .find(organization_id)
                .select((organizations::id, organizations::name))
                .first::<(i32, String)>(self.conn)
                .await
                .map(|(id, name)| OrganizationTeacherApplicationOrganizationOutput { id, name })
                .map_err(map_error)
        }
        .boxed()
    }

    fn can_view_applications(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationTeacherApplicationListError>> {
        async move {
            user_has_platform_or_organization_permission(
                self.conn,
                actor_user_id,
                organization_id,
                &Permissions::VIEW_ORG_TEACHER_APPLICATIONS.to_string(),
            )
            .await
        }
        .boxed()
    }

    fn can_nominate_teachers(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationTeacherApplicationListError>> {
        async move {
            user_has_platform_or_organization_permission(
                self.conn,
                actor_user_id,
                organization_id,
                &Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW.to_string(),
            )
            .await
        }
        .boxed()
    }

    fn list_teacher_applications(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<OrganizationTeacherApplicationDataset, OrganizationTeacherApplicationListError>,
    > {
        async move {
            let applications = teacher_applications::table
                .filter(
                    teacher_applications::organization_sponsor_id
                        .eq(Some(organization_id))
                        .or(teacher_applications::requested_organization_id
                            .eq(Some(organization_id))),
                )
                .order(teacher_applications::created_at.desc())
                .then_order_by(teacher_applications::id.desc())
                .load(self.conn)
                .await
                .map_err(map_error)?;
            let summary = teacher_application_summary(&applications);
            let context = build_context(self.conn, &applications).await?;
            let applications = applications
                .iter()
                .map(|application| {
                    organization_teacher_application_item(application, organization_id, &context)
                })
                .collect();

            Ok(OrganizationTeacherApplicationDataset {
                applications,
                summary,
                operator_permissions: OrganizationTeacherApplicationPermissionsOutput::default(),
            })
        }
        .boxed()
    }
}

async fn user_has_platform_or_organization_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    organization_id: i32,
    permission: &str,
) -> Result<bool, OrganizationTeacherApplicationListError> {
    if user_permission_platform_request(conn, user_id, permission)
        .await
        .map_err(map_error)?
    {
        return Ok(true);
    }

    user_permission_organization_request(conn, user_id, organization_id, permission)
        .await
        .map_err(map_error)
}

fn map_error(error: diesel::result::Error) -> OrganizationTeacherApplicationListError {
    match error {
        diesel::result::Error::NotFound => OrganizationTeacherApplicationListError::NotFound,
        other => OrganizationTeacherApplicationListError::Database(other.to_string()),
    }
}
