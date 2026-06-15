use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::infra::postgres::models::authentication::Authentication;
use crate::infra::postgres::schema::authentications;

pub async fn create_authentication(
    conn: &mut AsyncPgConnection,
    authentication: Authentication,
) -> QueryResult<usize> {
    diesel::insert_into(authentications::table)
        .values(&authentication)
        .execute(conn)
        .await
}
