use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletTokenTaxOperation {
    Deposit,
    Retire,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletTokenTaxOperationParseError {
    value: String,
}

impl WalletTokenTaxOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deposit => "deposit",
            Self::Retire => "retire",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WalletTokenTaxOperationParseError> {
        match value {
            "deposit" => Ok(Self::Deposit),
            "retire" => Ok(Self::Retire),
            other => Err(WalletTokenTaxOperationParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for WalletTokenTaxOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for WalletTokenTaxOperationParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown wallet token tax operation '{}'",
            self.value
        )
    }
}
