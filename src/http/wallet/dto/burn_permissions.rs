use serde::Serialize;

use crate::application::wallet::burn_tokens::OrganizationTokenBurnPermissions;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationTokenBurnPermissionsResponse {
    pub organization_id: i32,
    pub required_permission: String,
    pub can_burn: bool,
    pub kyc_verified: bool,
    pub can_request_burn: bool,
}

impl From<OrganizationTokenBurnPermissions> for OrganizationTokenBurnPermissionsResponse {
    fn from(permissions: OrganizationTokenBurnPermissions) -> Self {
        Self {
            organization_id: permissions.organization_id,
            required_permission: permissions.required_permission,
            can_burn: permissions.can_burn,
            kyc_verified: permissions.kyc_verified,
            can_request_burn: permissions.can_request_burn,
        }
    }
}
