use diesel::prelude::*;
use crate::db::schema::authentications;
use crate::models::user::User;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Queryable, Insertable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = authentications)]
pub struct Authentication {
    pub user_id: i32,
    pub type_authentication: String,
    pub info_auth: String,
}

impl Authentication {
    pub async fn create(new_auth: Authentication, conn: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::insert_into(authentications::table)
            .values(&new_auth)
            .execute(conn)
            .await
    }
}
