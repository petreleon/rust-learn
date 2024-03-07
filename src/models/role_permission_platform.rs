use crate::db::schema::role_permission_platform;
use diesel::prelude::*;
use crate::models::role::Role;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Role))]
#[diesel(table_name = role_permission_platform)]
pub struct RolePermissionPlatform {
    pub id: i32,
    pub role_id: Option<i32>,
    pub permission: String,
}
