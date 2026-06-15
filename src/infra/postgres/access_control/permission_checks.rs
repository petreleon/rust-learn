use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::db::schema::{
    role_permission_course, role_permission_organization, role_permission_platform,
    user_role_course, user_role_organization, user_role_platform,
};
use crate::infra::postgres::access_control::permission_delegations::{
    has_active_course_delegation, has_active_organization_delegation,
    has_active_platform_delegation,
};

pub(crate) async fn can(
    conn: &mut AsyncPgConnection,
    actor: AccessActor,
    action: AccessAction,
    scope: AccessScope,
) -> QueryResult<bool> {
    let permission = action.permission_name();
    match scope {
        AccessScope::Platform(_) => has_platform_permission(conn, actor.user_id, permission).await,
        AccessScope::Course(scope) => {
            has_course_permission(conn, actor.user_id, scope.course_id(), permission).await
        }
        AccessScope::Organization(scope) => {
            has_organization_permission(conn, actor.user_id, scope.organization_id(), permission)
                .await
        }
    }
}

pub(crate) async fn has_platform_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    if has_platform_role_permission(conn, actor_user_id, permission).await? {
        return Ok(true);
    }

    let has_delegation = has_active_platform_delegation(conn, actor_user_id, permission).await?;
    if has_delegation {
        log::info!(
            "event=delegated_permission_used scope=platform user_id={} permission={}",
            actor_user_id,
            permission
        );
    }

    Ok(has_delegation)
}

pub(crate) async fn has_course_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    if has_course_role_permission(conn, actor_user_id, course_id, permission).await? {
        return Ok(true);
    }

    let has_delegation =
        has_active_course_delegation(conn, actor_user_id, course_id, permission).await?;
    if has_delegation {
        log::info!(
            "event=delegated_permission_used scope=course user_id={} course_id={} permission={}",
            actor_user_id,
            course_id,
            permission
        );
    }

    Ok(has_delegation)
}

pub(crate) async fn has_organization_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    if has_organization_role_permission(conn, actor_user_id, organization_id, permission).await? {
        return Ok(true);
    }

    let has_delegation =
        has_active_organization_delegation(conn, actor_user_id, organization_id, permission)
            .await?;
    if has_delegation {
        log::info!(
            "event=delegated_permission_used scope=organization user_id={} organization_id={} permission={}",
            actor_user_id,
            organization_id,
            permission
        );
    }

    Ok(has_delegation)
}

async fn has_platform_role_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    select(exists(
        user_role_platform::table
            .inner_join(role_permission_platform::table.on(
                user_role_platform::platform_role_id.eq(role_permission_platform::platform_role_id),
            ))
            .filter(user_role_platform::user_id.eq(actor_user_id))
            .filter(role_permission_platform::permission.eq(permission)),
    ))
    .get_result(conn)
    .await
}

async fn has_course_role_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    select(
        exists(
            user_role_course::table
                .inner_join(role_permission_course::table.on(
                    user_role_course::course_role_id.eq(role_permission_course::course_role_id),
                ))
                .filter(user_role_course::user_id.eq(actor_user_id))
                .filter(user_role_course::course_id.eq(course_id))
                .filter(role_permission_course::permission.eq(permission)),
        ),
    )
    .get_result(conn)
    .await
}

async fn has_organization_role_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    select(exists(
        user_role_organization::table
            .inner_join(
                role_permission_organization::table
                    .on(user_role_organization::organization_role_id
                        .eq(role_permission_organization::organization_role_id)),
            )
            .filter(user_role_organization::user_id.eq(actor_user_id))
            .filter(user_role_organization::organization_id.eq(organization_id))
            .filter(role_permission_organization::permission.eq(permission)),
    ))
    .get_result(conn)
    .await
}
