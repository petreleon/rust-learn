mod permission_rules;
mod scope_rules;
mod scope_type;

use std::fmt;

pub use permission_rules::{normalize_filter_permission, normalize_permission};
pub use scope_rules::{
    normalize_filter_scope_type, normalize_scope_ids, normalize_scope_type, DelegatedScope,
    DelegationScope,
};
pub use scope_type::{
    DelegatedScopeType, DELEGATED_SCOPE_COURSE, DELEGATED_SCOPE_ORGANIZATION,
    DELEGATED_SCOPE_PLATFORM,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelegationRuleError {
    InvalidInput(String),
}

impl fmt::Display for DelegationRuleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => formatter.write_str(message),
        }
    }
}

#[cfg(test)]
mod tests;
