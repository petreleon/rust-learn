use crate::db::schema::role_organization_hierarchy;
use diesel::prelude::*;
use crate::models::role::OrganizationRole;
use crate::models::organization::Organization;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(OrganizationRole))]
#[diesel(belongs_to(Organization))]
#[diesel(table_name = role_organization_hierarchy)]
pub struct RoleOrganizationHierarchy {
    pub id: i32,
    pub organization_role_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub hierarchy_level: i32,
}
