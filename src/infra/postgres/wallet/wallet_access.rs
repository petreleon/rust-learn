use diesel_async::AsyncPgConnection;

use crate::application::access_control::authorize_wallet::{
    authorize_wallet_action, WalletAuthorizationAction, WalletAuthorizationError,
};
use crate::infra::postgres::access_control::access_decision_store::PostgresAccessDecisionStore;

pub(super) async fn can_view_user_wallet(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, WalletAuthorizationError> {
    authorize_wallet(
        conn,
        actor_user_id,
        WalletAuthorizationAction::ViewUserWallet,
    )
    .await
}

pub(super) async fn can_view_organization_wallet(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<bool, WalletAuthorizationError> {
    authorize_wallet(
        conn,
        actor_user_id,
        WalletAuthorizationAction::ViewOrganizationWallet { organization_id },
    )
    .await
}

pub(super) async fn can_link_user_wallet(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, WalletAuthorizationError> {
    authorize_wallet(
        conn,
        actor_user_id,
        WalletAuthorizationAction::LinkUserWallet,
    )
    .await
}

pub(super) async fn can_link_organization_wallet(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<bool, WalletAuthorizationError> {
    authorize_wallet(
        conn,
        actor_user_id,
        WalletAuthorizationAction::LinkOrganizationWallet { organization_id },
    )
    .await
}

async fn authorize_wallet(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    action: WalletAuthorizationAction,
) -> Result<bool, WalletAuthorizationError> {
    let mut store = PostgresAccessDecisionStore::new(conn);
    authorize_wallet_action(&mut store, actor_user_id, action).await
}
