use crate::config::constants::permissions::Permissions;

pub(super) fn platform_fraud_notification_permissions() -> [String; 2] {
    [
        Permissions::VIEW_REWARD_AUDIT.to_string(),
        Permissions::MANAGE_REWARD_FRAUD_BLOCKS.to_string(),
    ]
}

pub(super) fn organization_fraud_notification_permissions() -> [String; 2] {
    [
        Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
        Permissions::MANAGE_ORG_REWARD_BUDGET.to_string(),
    ]
}

#[cfg(test)]
mod tests;
