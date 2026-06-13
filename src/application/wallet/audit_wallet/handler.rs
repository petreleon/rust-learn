use crate::application::wallet::audit_wallet::{
    WalletAudit, WalletAuditError, WalletAuditStore, WalletAuditSubject, WalletAuditTarget,
};

pub async fn audit_wallet(
    store: &mut impl WalletAuditStore,
    target: WalletAuditTarget,
) -> Result<WalletAudit, WalletAuditError> {
    store.load_wallet_audit(target).await
}

pub async fn audit_wallet_for_actor(
    store: &mut impl WalletAuditStore,
    actor_user_id: i32,
    subject: WalletAuditSubject,
) -> Result<WalletAudit, WalletAuditError> {
    match subject {
        WalletAuditSubject::OwnUser => audit_user_wallet(store, actor_user_id, actor_user_id).await,
        WalletAuditSubject::User(user_id) => audit_user_wallet(store, actor_user_id, user_id).await,
        WalletAuditSubject::Organization(organization_id) => {
            audit_organization_wallet(store, actor_user_id, organization_id).await
        }
    }
}

async fn audit_user_wallet(
    store: &mut impl WalletAuditStore,
    actor_user_id: i32,
    target_user_id: i32,
) -> Result<WalletAudit, WalletAuditError> {
    if actor_user_id != target_user_id && !store.can_view_user_wallet(actor_user_id).await? {
        return Err(WalletAuditError::UserPermissionDenied);
    }

    if !store.user_exists(target_user_id).await? {
        return Err(WalletAuditError::UserNotFound);
    }

    let target = store
        .find_user_wallet(target_user_id)
        .await?
        .ok_or(WalletAuditError::WalletNotLinked)?;

    store.load_wallet_audit(target).await
}

async fn audit_organization_wallet(
    store: &mut impl WalletAuditStore,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<WalletAudit, WalletAuditError> {
    if !store.organization_exists(organization_id).await? {
        return Err(WalletAuditError::OrganizationNotFound);
    }

    if !store
        .can_view_organization_wallet(actor_user_id, organization_id)
        .await?
    {
        return Err(WalletAuditError::OrganizationPermissionDenied);
    }

    let target = store
        .find_organization_wallet(organization_id)
        .await?
        .ok_or(WalletAuditError::WalletNotLinked)?;

    store.load_wallet_audit(target).await
}

#[cfg(test)]
#[path = "handler_tests.rs"]
mod handler_tests;
