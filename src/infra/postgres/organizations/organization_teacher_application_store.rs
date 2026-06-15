use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationDataset, OrganizationTeacherApplicationListError,
    OrganizationTeacherApplicationListStore, OrganizationTeacherApplicationOrganizationOutput,
    OrganizationTeacherApplicationPermissionsOutput,
};
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::organizations::organization_teacher_application_audit_context::teacher_application_summary;
use crate::infra::postgres::organizations::organization_teacher_application_context::build_context;
use crate::infra::postgres::organizations::organization_teacher_application_mappers::organization_teacher_application_item;
use crate::infra::postgres::schema::{organizations, teacher_applications};

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

impl AccessDecisionStore for PostgresOrganizationTeacherApplicationStore<'_> {
    type Error = OrganizationTeacherApplicationListError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, OrganizationTeacherApplicationListError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(map_error)
        }
        .boxed()
    }
}

fn map_error(error: diesel::result::Error) -> OrganizationTeacherApplicationListError {
    match error {
        diesel::result::Error::NotFound => OrganizationTeacherApplicationListError::NotFound,
        other => OrganizationTeacherApplicationListError::Database(other.to_string()),
    }
}
