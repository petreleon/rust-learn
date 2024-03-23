use crate::db::schema::role_permission_organization;
use diesel::prelude::*;
use crate::models::role::Role;
use crate::models::organization::Organization;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(Organization))]
#[diesel(table_name = role_permission_organization)]
pub struct RolePermissionOrganization {
    pub id: i32,
    pub organization_id: Option<i32>,
    pub role_id: Option<i32>,
    pub permission: String,
}
