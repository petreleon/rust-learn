use crate::db::schema::user_role_organization;
use crate::models::organization::Organization;
use crate::models::role::OrganizationRole;
use crate::models::user::User;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(OrganizationRole))]
#[diesel(belongs_to(Organization))]
#[diesel(table_name = user_role_organization)]
pub struct UserRoleOrganization {
    pub id: i32,
    pub user_id: Option<i32>,
    pub organization_role_id: Option<i32>, // updated field name
    pub organization_id: Option<i32>,
}
