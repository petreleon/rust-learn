use crate::db::schema::user_role_platform;
use diesel::prelude::*;
use crate::models::user::User;
use crate::models::role::PlatformRole;

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(PlatformRole))]
#[diesel(table_name = user_role_platform)]
pub struct UserRolePlatform {
    pub id: i32,
    pub user_id: Option<i32>,
    pub platform_role_id: Option<i32>, // updated!
}
