pub const WALLET_OWNER_TYPE_ORGANIZATION: &str = "organization";
pub const WALLET_OWNER_TYPE_USER: &str = "user";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletOwnerType {
    User,
    Organization,
}

impl WalletOwnerType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => WALLET_OWNER_TYPE_USER,
            Self::Organization => WALLET_OWNER_TYPE_ORGANIZATION,
        }
    }

    pub fn from_user_id(user_id: Option<i32>) -> Self {
        if user_id.is_some() {
            Self::User
        } else {
            Self::Organization
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_owner_type_from_user_presence() {
        assert_eq!(
            WalletOwnerType::from_user_id(Some(7)),
            WalletOwnerType::User
        );
        assert_eq!(
            WalletOwnerType::from_user_id(None),
            WalletOwnerType::Organization
        );
        assert_eq!(WalletOwnerType::User.as_str(), "user");
    }
}
