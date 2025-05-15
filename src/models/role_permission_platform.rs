use crate::db::schema::role_permission_platform;
use diesel::prelude::*;
use crate::models::role::PlatformRole;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(PlatformRole))]
#[diesel(table_name = role_permission_platform)]
pub struct RolePermissionPlatform {
    pub id: i32,
    pub platform_role_id: Option<i32>, // changed here!
    pub permission: String,
}
