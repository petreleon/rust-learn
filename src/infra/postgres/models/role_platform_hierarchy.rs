use crate::infra::postgres::models::role::PlatformRole;
use crate::infra::postgres::schema::role_platform_hierarchy;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(PlatformRole))]
#[diesel(table_name = role_platform_hierarchy)]
pub struct RolePlatformHierarchy {
    pub id: i32,
    pub platform_role_id: Option<i32>, // updated field name
    pub hierarchy_level: i32,
}
