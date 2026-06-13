#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletReadSubject {
    OwnUser,
    User(i32),
    Organization(i32),
}
