#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletLinkSubject {
    OwnUser,
    User(i32),
    Organization(i32),
}
