use crate::domain::access_control::permission::Permission;

pub fn platform_reward_fraud_block_notification_permissions() -> [Permission; 2] {
    [
        Permission::ViewRewardAudit,
        Permission::ManageRewardFraudBlocks,
    ]
}

pub fn organization_reward_fraud_block_notification_permissions() -> [Permission; 2] {
    [
        Permission::ViewOrgRewardReports,
        Permission::ManageOrgRewardBudget,
    ]
}

#[cfg(test)]
mod tests;
