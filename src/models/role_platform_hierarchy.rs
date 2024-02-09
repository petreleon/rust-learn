use crate::db::schema::role_platform_hierarchy;
use diesel::prelude::*;
use crate::models::role::Role;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Role)]
#[diesel(table_name = role_platform_hierarchy)]
pub struct RolePlatformHierarchy {
    pub id: i32,
    pub role_id: Option<i32>,
    pub hierarchy_level: i32,
}
