use crate::application::wallet::read_wallet::{
    WalletReadError, WalletReadStore, WalletReadSubject, WalletView,
};

pub async fn read_wallet(
    store: &mut impl WalletReadStore,
    actor_user_id: i32,
    subject: WalletReadSubject,
) -> Result<WalletView, WalletReadError> {
    match subject {
        WalletReadSubject::OwnUser => read_user_wallet(store, actor_user_id, actor_user_id).await,
        WalletReadSubject::User(user_id) => read_user_wallet(store, actor_user_id, user_id).await,
        WalletReadSubject::Organization(organization_id) => {
            read_organization_wallet(store, actor_user_id, organization_id).await
        }
    }
}

async fn read_user_wallet(
    store: &mut impl WalletReadStore,
    actor_user_id: i32,
    target_user_id: i32,
) -> Result<WalletView, WalletReadError> {
    if actor_user_id != target_user_id && !store.can_view_user_wallet(actor_user_id).await? {
        return Err(WalletReadError::UserPermissionDenied);
    }

    if !store.user_exists(target_user_id).await? {
        return Err(WalletReadError::UserNotFound);
    }

    store
        .find_user_wallet(target_user_id)
        .await?
        .ok_or(WalletReadError::WalletNotLinked)
}

async fn read_organization_wallet(
    store: &mut impl WalletReadStore,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<WalletView, WalletReadError> {
    if !store.organization_exists(organization_id).await? {
        return Err(WalletReadError::OrganizationNotFound);
    }

    if !store
        .can_view_organization_wallet(actor_user_id, organization_id)
        .await?
    {
        return Err(WalletReadError::OrganizationPermissionDenied);
    }

    store
        .find_organization_wallet(organization_id)
        .await?
        .ok_or(WalletReadError::WalletNotLinked)
}
