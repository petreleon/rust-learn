use diesel::prelude::*;
use crate::db::schema::authorizations;
use crate::models::user::User;

#[derive(Queryable, Insertable, Associations)]
#[belongs_to(User)]
#[table_name="authorizations"]
pub struct Authorization {
    pub user_id: i32,
    pub type_authorization: String,
    pub info_auth: String,
}
