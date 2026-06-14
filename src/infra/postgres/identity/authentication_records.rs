use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::authentications;
use crate::models::authentication::Authentication;

pub async fn create_authentication(
    conn: &mut AsyncPgConnection,
    authentication: Authentication,
) -> QueryResult<usize> {
    diesel::insert_into(authentications::table)
        .values(&authentication)
        .execute(conn)
        .await
}
