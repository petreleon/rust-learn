use crate::infra::postgres::models::user::User;
use crate::infra::postgres::schema::authentications;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = authentications)]
pub struct Authentication {
    pub user_id: i32,
    pub type_authentication: String,
    pub info_auth: String,
}
