use crate::db::schema::user_role_platform;
use crate::models::role::PlatformRole;
use crate::models::user::User;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(PlatformRole))]
#[diesel(table_name = user_role_platform)]
pub struct UserRolePlatform {
    pub id: i32,
    pub user_id: i32,
    pub platform_role_id: i32,
}
