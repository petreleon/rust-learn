#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletAuditSubject {
    OwnUser,
    User(i32),
    Organization(i32),
}
