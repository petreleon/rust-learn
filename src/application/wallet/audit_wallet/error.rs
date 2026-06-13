#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletAuditError {
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
    AuditLoad(String),
    Connection(String),
    Database(String),
}
