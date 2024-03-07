use crate::db::schema::user_role_organization;
use diesel::prelude::*;
use crate::models::user::User;
use crate::models::role::Role;
use crate::models::organization::Organization;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(Organization))]
#[diesel(table_name = user_role_organization)]
pub struct UserRoleOrganization {
    pub id: i32,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
    pub organization_id: Option<i32>,
}
