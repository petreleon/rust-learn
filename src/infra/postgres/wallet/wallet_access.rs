use diesel_async::AsyncPgConnection;

use crate::config::constants::permissions::Permissions;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;

pub(super) async fn can_view_user_wallet(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> diesel::QueryResult<bool> {
    has_any_platform_permission(conn, actor_user_id, user_wallet_view_permissions()).await
}

pub(super) async fn can_view_organization_wallet(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> diesel::QueryResult<bool> {
    if has_any_platform_permission(conn, actor_user_id, user_wallet_view_permissions()).await? {
        return Ok(true);
    }

    has_any_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        organization_wallet_view_permissions(),
    )
    .await
}

pub(super) async fn can_link_user_wallet(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> diesel::QueryResult<bool> {
    has_any_platform_permission(conn, actor_user_id, user_wallet_link_permissions()).await
}

pub(super) async fn can_link_organization_wallet(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> diesel::QueryResult<bool> {
    if has_any_platform_permission(conn, actor_user_id, user_wallet_link_permissions()).await? {
        return Ok(true);
    }

    has_any_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        organization_wallet_link_permissions(),
    )
    .await
}

fn user_wallet_view_permissions() -> Vec<String> {
    vec![
        Permissions::VIEW_WALLET.to_string(),
        Permissions::VIEW_TRANSACTIONS.to_string(),
        Permissions::RECONCILE_WALLETS.to_string(),
        Permissions::MANAGE_WALLETS.to_string(),
    ]
}

fn organization_wallet_view_permissions() -> Vec<String> {
    vec![
        Permissions::MANAGE_ORG_WALLETS.to_string(),
        Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
        Permissions::MANAGE_ORG_REWARD_BUDGET.to_string(),
    ]
}

fn user_wallet_link_permissions() -> Vec<String> {
    vec![
        Permissions::CREATE_WALLET.to_string(),
        Permissions::MANAGE_WALLETS.to_string(),
    ]
}

fn organization_wallet_link_permissions() -> Vec<String> {
    vec![Permissions::MANAGE_ORG_WALLETS.to_string()]
}

async fn has_any_platform_permission(
    conn: &mut AsyncPgConnection,
    requester_id: i32,
    permissions: Vec<String>,
) -> diesel::QueryResult<bool> {
    for permission in permissions {
        if user_permission_platform_request(conn, requester_id, &permission).await? {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn has_any_organization_permission(
    conn: &mut AsyncPgConnection,
    requester_id: i32,
    organization_id: i32,
    permissions: Vec<String>,
) -> diesel::QueryResult<bool> {
    for permission in permissions {
        if user_permission_organization_request(conn, requester_id, organization_id, &permission)
            .await?
        {
            return Ok(true);
        }
    }

    Ok(false)
}
