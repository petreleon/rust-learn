use serde::Serialize;

use crate::application::access_control::role_catalog::RoleCatalogEntry;

#[derive(Debug, Clone, Serialize)]
pub struct RoleResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl From<RoleCatalogEntry> for RoleResponse {
    fn from(role: RoleCatalogEntry) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
        }
    }
}
