use crate::db::schema::user_role_platform;
use diesel::prelude::*;
use crate::models::user::User;
use crate::models::role::Role;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = user_role_platform)]
pub struct UserRolePlatform {
    pub id: i32,
    pub user_id: Option<i32>,
    pub role_id: Option<i32>,
}
