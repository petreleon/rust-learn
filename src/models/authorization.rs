use diesel::prelude::*;
use super::schema::authorizations;

#[derive(Queryable, Insertable)]
#[table_name="authorizations"]
pub struct Authorization {
    pub id_user: i32,
    pub type_authorization: String,
    pub info_auth: String,
}
