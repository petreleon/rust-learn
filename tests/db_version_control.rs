use diesel_async::{
    pooled_connection::deadpool::Object as PooledConnection, AsyncConnection, AsyncPgConnection,
    RunQueryDsl,
};
use rust_learn::infra::postgres::establish_connection;
use rust_learn::infra::postgres::operations::db_version_control as db_version_control_records;
use rust_learn::infra::postgres::schema::db_version_control;

async fn setup_conn() -> PooledConnection<AsyncPgConnection> {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    pool.get()
        .await
        .expect("failed to get DB connection from pool")
}

#[actix_web::test]
async fn update_version_recreates_missing_control_row() {
    let mut conn = setup_conn().await;

    let result: Result<(), diesel::result::Error> = conn
        .transaction::<_, diesel::result::Error, _>(|conn| {
            Box::pin(async move {
                diesel::delete(db_version_control::table)
                    .execute(conn)
                    .await?;

                db_version_control_records::update_version(conn, 7).await?;
                let version_after_insert =
                    db_version_control_records::get_current_version(conn).await?;
                assert_eq!(version_after_insert, 7);

                db_version_control_records::update_version(conn, 9).await?;
                let version_after_update =
                    db_version_control_records::get_current_version(conn).await?;
                assert_eq!(version_after_update, 9);

                Err(diesel::result::Error::RollbackTransaction)
            })
        })
        .await;

    assert!(matches!(
        result,
        Err(diesel::result::Error::RollbackTransaction)
    ));
}
