use crate::db::schema::authentications;
use crate::models::user::User;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = authentications)]
pub struct Authentication {
    pub user_id: i32,
    pub type_authentication: String,
    pub info_auth: String,
}
