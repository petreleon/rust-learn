use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::infra::postgres::access_control::permission_delegations::{
    has_active_course_delegation, has_active_organization_delegation,
    has_active_platform_delegation,
};
use crate::infra::postgres::access_control::permission_role_queries::{
    has_course_role_permission, has_organization_role_permission, has_platform_role_permission,
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

pub(crate) async fn can_platform_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    can(
        conn,
        AccessActor::user(actor_user_id),
        AccessAction::permission(permission),
        AccessScope::platform(),
    )
    .await
}

pub(crate) async fn can_course_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    can(
        conn,
        AccessActor::user(actor_user_id),
        AccessAction::permission(permission),
        AccessScope::course(course_id),
    )
    .await
}

pub(crate) async fn can_organization_permission(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    can(
        conn,
        AccessActor::user(actor_user_id),
        AccessAction::permission(permission),
        AccessScope::organization(organization_id),
    )
    .await
}

async fn has_platform_permission(
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

async fn has_course_permission(
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

async fn has_organization_permission(
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
