use crate::domain::access_control::permissions::Permissions;

pub fn platform_reward_fraud_block_notification_permissions() -> [Permissions; 2] {
    [
        Permissions::VIEW_REWARD_AUDIT,
        Permissions::MANAGE_REWARD_FRAUD_BLOCKS,
    ]
}

pub fn organization_reward_fraud_block_notification_permissions() -> [Permissions; 2] {
    [
        Permissions::VIEW_ORG_REWARD_REPORTS,
        Permissions::MANAGE_ORG_REWARD_BUDGET,
    ]
}

#[cfg(test)]
mod tests;
