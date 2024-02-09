use crate::db::schema::role_organization_hierarchy;
use diesel::prelude::*;
use crate::models::role::Role;
use crate::models::organization::Organization;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Role)]
#[belongs_to(Organization)]
#[diesel(table_name = role_organization_hierarchy)]
pub struct RoleOrganizationHierarchy {
    pub id: i32,
    pub role_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub hierarchy_level: i32,
}
