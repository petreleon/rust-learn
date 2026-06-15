mod permission_rules;
mod scope_rules;

pub use permission_rules::{normalize_filter_permission, normalize_permission};
pub use scope_rules::{
    normalize_filter_scope_type, normalize_scope_ids, normalize_scope_type, DelegatedScope,
    DelegationScope, DELEGATED_SCOPE_COURSE, DELEGATED_SCOPE_ORGANIZATION,
    DELEGATED_SCOPE_PLATFORM,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelegationRuleError {
    InvalidInput(String),
}

#[cfg(test)]
mod tests;
