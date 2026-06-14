#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletReadError {
    UserNotFound,
    OrganizationNotFound,
    WalletNotLinked,
    UserPermissionDenied,
    OrganizationPermissionDenied,
    UserLoad(String),
    OrganizationLoad(String),
    UserAccessCheck(String),
    OrganizationAccessCheck(String),
    WalletLoad(String),
    Connection(String),
}
