use crate::application::wallet::link_wallet::{
    LinkedWalletView, WalletLinkError, WalletLinkStore, WalletLinkSubject,
};

pub async fn link_wallet(
    store: &mut impl WalletLinkStore,
    actor_user_id: i32,
    subject: WalletLinkSubject,
) -> Result<LinkedWalletView, WalletLinkError> {
    match subject {
        WalletLinkSubject::OwnUser => link_user_wallet(store, actor_user_id, actor_user_id).await,
        WalletLinkSubject::User(user_id) => link_user_wallet(store, actor_user_id, user_id).await,
        WalletLinkSubject::Organization(organization_id) => {
            link_organization_wallet(store, actor_user_id, organization_id).await
        }
    }
}

async fn link_user_wallet(
    store: &mut impl WalletLinkStore,
    actor_user_id: i32,
    target_user_id: i32,
) -> Result<LinkedWalletView, WalletLinkError> {
    if actor_user_id != target_user_id && !store.can_link_user_wallet(actor_user_id).await? {
        return Err(WalletLinkError::UserPermissionDenied);
    }

    if !store.user_exists(target_user_id).await? {
        return Err(WalletLinkError::UserNotFound);
    }

    if !store.user_kyc_verified(target_user_id).await? {
        return Err(WalletLinkError::KycRequired);
    }

    store.link_user_wallet(target_user_id).await
}

async fn link_organization_wallet(
    store: &mut impl WalletLinkStore,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<LinkedWalletView, WalletLinkError> {
    if !store.organization_exists(organization_id).await? {
        return Err(WalletLinkError::OrganizationNotFound);
    }

    if !store
        .can_link_organization_wallet(actor_user_id, organization_id)
        .await?
    {
        return Err(WalletLinkError::OrganizationPermissionDenied);
    }

    store.link_organization_wallet(organization_id).await
}
