use std::fmt;

pub const TOKEN_BURN_BURNER_ORGANIZATION: &str = "organization";
pub const TOKEN_BURN_BURNER_USER: &str = "user";
pub const TOKEN_BURN_FEE_NETWORK_USER: &str = "network_fee_paid_by_user";
pub const TOKEN_BURN_FEE_NONE: &str = "none";
pub const TOKEN_BURN_FEE_PLATFORM_DEPOSIT: &str = "platform_deposit_fee";
pub const TOKEN_BURN_FEE_PLATFORM_SUBSIDIZED: &str = "platform_subsidized";
pub const TOKEN_BURN_SOURCE_CENTRALIZED: &str = "centralized_wallet";
pub const TOKEN_BURN_SOURCE_DECENTRALIZED_DIRECT: &str = "decentralized_direct";
pub const TOKEN_BURN_SOURCE_PLATFORM_MEDIATED: &str = "decentralized_platform_mediated";
pub const TOKEN_BURN_STATUS_DEPOSIT_PENDING: &str = "deposit_pending";
pub const TOKEN_BURN_STATUS_FAILED: &str = "failed";
pub const TOKEN_BURN_STATUS_LEADERBOARD_INDEXED: &str = "leaderboard_indexed";
pub const TOKEN_BURN_STATUS_NEEDS_RECONCILIATION: &str = "needs_reconciliation";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenBurnerType {
    User,
    Organization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenBurnFeePath {
    NetworkFeePaidByUser,
    PlatformDepositFee,
    PlatformSubsidized,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenBurnLeaderboardScope {
    All,
    Users,
    Organizations,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenBurnLeaderboardWindow {
    Days7,
    Days30,
    Days365,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenBurnSource {
    CentralizedWallet,
    DecentralizedDirect,
    DecentralizedPlatformMediated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenBurnStatus {
    DepositPending,
    LeaderboardIndexed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenBurnParseError {
    field: &'static str,
    value: String,
}

impl TokenBurnerType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => TOKEN_BURN_BURNER_USER,
            Self::Organization => TOKEN_BURN_BURNER_ORGANIZATION,
        }
    }
}

impl TokenBurnFeePath {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NetworkFeePaidByUser => TOKEN_BURN_FEE_NETWORK_USER,
            Self::PlatformDepositFee => TOKEN_BURN_FEE_PLATFORM_DEPOSIT,
            Self::PlatformSubsidized => TOKEN_BURN_FEE_PLATFORM_SUBSIDIZED,
            Self::None => TOKEN_BURN_FEE_NONE,
        }
    }

    pub fn parse(value: &str) -> Result<Self, TokenBurnParseError> {
        match normalized(value).as_str() {
            TOKEN_BURN_FEE_NETWORK_USER => Ok(Self::NetworkFeePaidByUser),
            TOKEN_BURN_FEE_PLATFORM_DEPOSIT => Ok(Self::PlatformDepositFee),
            TOKEN_BURN_FEE_PLATFORM_SUBSIDIZED => Ok(Self::PlatformSubsidized),
            TOKEN_BURN_FEE_NONE => Ok(Self::None),
            other => Err(parse_error("fee_path", other)),
        }
    }
}

impl TokenBurnLeaderboardScope {
    pub fn parse(value: Option<&str>) -> Result<Self, TokenBurnParseError> {
        match value.map(normalized).as_deref().unwrap_or("all") {
            "all" => Ok(Self::All),
            "users" => Ok(Self::Users),
            "organizations" => Ok(Self::Organizations),
            other => Err(parse_error("scope", other)),
        }
    }
}

impl TokenBurnLeaderboardWindow {
    pub fn days(self) -> i64 {
        match self {
            Self::Days7 => 7,
            Self::Days30 => 30,
            Self::Days365 => 365,
        }
    }

    pub fn parse(value: &str) -> Result<Self, TokenBurnParseError> {
        match normalized(value).as_str() {
            "7d" => Ok(Self::Days7),
            "30d" => Ok(Self::Days30),
            "365d" => Ok(Self::Days365),
            other => Err(parse_error("window", other)),
        }
    }
}

impl TokenBurnSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CentralizedWallet => TOKEN_BURN_SOURCE_CENTRALIZED,
            Self::DecentralizedDirect => TOKEN_BURN_SOURCE_DECENTRALIZED_DIRECT,
            Self::DecentralizedPlatformMediated => TOKEN_BURN_SOURCE_PLATFORM_MEDIATED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, TokenBurnParseError> {
        match normalized(value).as_str() {
            TOKEN_BURN_SOURCE_CENTRALIZED => Ok(Self::CentralizedWallet),
            TOKEN_BURN_SOURCE_DECENTRALIZED_DIRECT => Ok(Self::DecentralizedDirect),
            TOKEN_BURN_SOURCE_PLATFORM_MEDIATED => Ok(Self::DecentralizedPlatformMediated),
            other => Err(parse_error("source", other)),
        }
    }
}

impl TokenBurnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DepositPending => TOKEN_BURN_STATUS_DEPOSIT_PENDING,
            Self::LeaderboardIndexed => TOKEN_BURN_STATUS_LEADERBOARD_INDEXED,
        }
    }
}

impl fmt::Display for TokenBurnParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown token burn {} '{}'",
            self.field, self.value
        )
    }
}

fn normalized(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn parse_error(field: &'static str, value: &str) -> TokenBurnParseError {
    TokenBurnParseError {
        field,
        value: value.to_string(),
    }
}
