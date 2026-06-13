#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletLinkError {
    UserNotFound,
    OrganizationNotFound,
    KycRequired,
    UserPermissionDenied,
    OrganizationPermissionDenied,
    UserLoad(String),
    OrganizationLoad(String),
    UserAccessCheck(String),
    OrganizationAccessCheck(String),
    KycLoad(String),
    WalletLoad(String),
    WalletCreate(String),
    Connection(String),
}
