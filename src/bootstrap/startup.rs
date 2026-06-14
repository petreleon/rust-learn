use crate::bootstrap::app_state::AppState;
use crate::bootstrap::contract_startup::deploy_startup_contracts;
use crate::bootstrap::use_case_wiring::build_app_state;
use crate::config::db_setup::version_updater;
use crate::db;
use crate::infra::object_storage::S3State;

pub async fn initialize_app_state() -> std::io::Result<AppState> {
    let pool = db::try_establish_connection().map_err(|error| {
        log::error!("event=db_pool_init_failed error={}", error);
        std::io::Error::other(error)
    })?;

    let s3 = S3State::new_from_env().await.map_err(|error| {
        log::error!("event=s3_init_failed error={:?}", error);
        std::io::Error::other("S3 init failed")
    })?;

    run_startup_tasks(&pool).await?;

    Ok(build_app_state(pool, s3))
}

async fn run_startup_tasks(pool: &db::DbPool) -> std::io::Result<()> {
    let mut conn = pool.get().await.map_err(|error| {
        log::error!("event=db_connection_failed phase=startup error={:?}", error);
        std::io::Error::other(format!("Failed to get DB connection from pool: {error}"))
    })?;

    version_updater(&mut conn).await.map_err(|error| {
        log::error!("event=db_version_update_failed error={:?}", error);
        std::io::Error::other(format!("Failed to update database version: {error}"))
    })?;

    deploy_startup_contracts(&mut conn).await;
    Ok(())
}
