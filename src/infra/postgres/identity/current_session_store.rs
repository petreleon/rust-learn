use std::collections::{BTreeMap, BTreeSet};

use diesel::result::Error as DieselError;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::identity::current_session::{CurrentSessionError, CurrentSessionOutput};
use crate::application::identity::ports::CurrentSessionStore;
use crate::infra::postgres::identity::current_session_course_queries::{
    list_course_permissions, list_course_roles,
};
use crate::infra::postgres::identity::current_session_delegations::{
    active_delegations_for_user, apply_delegations, course_labels, organization_labels,
};
use crate::infra::postgres::identity::current_session_organization_queries::{
    list_organization_permissions, list_organization_roles,
};
use crate::infra::postgres::identity::current_session_platform_queries::{
    find_user, list_platform_permissions, list_platform_roles,
};
use crate::infra::postgres::identity::current_session_scope_builder::{
    course_builder, delegated_permission_session, organization_builder, PlatformScopeBuilder,
};

pub struct PostgresCurrentSessionStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCurrentSessionStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CurrentSessionStore for PostgresCurrentSessionStore<'_> {
    fn load_current_session(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<CurrentSessionOutput, CurrentSessionError>> {
        async move { load_current_session(self.conn, user_id).await }.boxed()
    }
}

async fn load_current_session(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<CurrentSessionOutput, CurrentSessionError> {
    let user = find_user(conn, user_id)
        .await
        .map_err(map_current_session_error)?;
    if !user.email_verified {
        return Err(CurrentSessionError::EmailUnverified);
    }

    let mut platform = PlatformScopeBuilder {
        roles: list_platform_roles(conn, user_id)
            .await
            .map_err(map_current_session_error)?
            .into_iter()
            .collect(),
        direct_permissions: list_platform_permissions(conn, user_id)
            .await
            .map_err(map_current_session_error)?
            .into_iter()
            .collect(),
        delegated_permissions: BTreeSet::new(),
    };

    let mut organizations = BTreeMap::new();
    for row in list_organization_roles(conn, user_id)
        .await
        .map_err(map_current_session_error)?
    {
        organization_builder(
            &mut organizations,
            row.organization_id,
            row.organization_name,
        )
        .roles
        .insert(row.role_name);
    }

    for row in list_organization_permissions(conn, user_id)
        .await
        .map_err(map_current_session_error)?
    {
        organization_builder(
            &mut organizations,
            row.organization_id,
            row.organization_name,
        )
        .direct_permissions
        .insert(row.permission);
    }

    let mut courses = BTreeMap::new();
    for row in list_course_roles(conn, user_id)
        .await
        .map_err(map_current_session_error)?
    {
        course_builder(
            &mut courses,
            row.course_id,
            row.course_title,
            row.lifecycle_status,
        )
        .roles
        .insert(row.role_name);
    }

    for row in list_course_permissions(conn, user_id)
        .await
        .map_err(map_current_session_error)?
    {
        course_builder(
            &mut courses,
            row.course_id,
            row.course_title,
            row.lifecycle_status,
        )
        .direct_permissions
        .insert(row.permission);
    }

    let delegations = active_delegations_for_user(conn, user_id).await?;

    let organization_labels = organization_labels(conn, &delegations).await?;
    let course_labels = course_labels(conn, &delegations).await?;
    apply_delegations(
        &delegations,
        &organization_labels,
        &course_labels,
        &mut platform,
        &mut organizations,
        &mut courses,
    );

    Ok(CurrentSessionOutput {
        user: user.into(),
        platform: platform.into(),
        organizations: organizations.into_values().map(Into::into).collect(),
        courses: courses.into_values().map(Into::into).collect(),
        delegated_permissions: delegations
            .into_iter()
            .map(|delegation| {
                delegated_permission_session(delegation, &organization_labels, &course_labels)
            })
            .collect(),
    })
}

fn map_current_session_error(error: DieselError) -> CurrentSessionError {
    match error {
        DieselError::NotFound => CurrentSessionError::MissingUser,
        other => CurrentSessionError::Database(other.to_string()),
    }
}
