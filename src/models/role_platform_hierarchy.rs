use crate::db::schema::role_platform_hierarchy;
use diesel::prelude::*;
use crate::models::role::PlatformRole;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(PlatformRole))]
#[diesel(table_name = role_platform_hierarchy)]
pub struct RolePlatformHierarchy {
    pub id: i32,
    pub platform_role_id: Option<i32>, // updated field name
    pub hierarchy_level: i32,
}
