use std::time::Duration;

use actix_web::rt::time::timeout;

use crate::bootstrap::app_state::AppState;
use crate::config::db_setup::version_updater;
use crate::db;
use crate::utils::notifications::NotificationsState;
use crate::utils::s3_utils::S3State;

const ETH_STARTUP_DEPLOY_TIMEOUT: Duration = Duration::from_secs(30);

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

    Ok(AppState {
        notifications: NotificationsState::new(pool.clone()),
        pool,
        s3,
    })
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

async fn deploy_startup_contracts(conn: &mut diesel_async::AsyncPgConnection) {
    match timeout(
        ETH_STARTUP_DEPLOY_TIMEOUT,
        crate::utils::eth_utils::deploy_all_startup(conn, "LearnToken", "LRN", 18),
    )
    .await
    {
        Ok(Ok((token_addr, presigner_addr, importer_addr))) => {
            let presigner_address = presigner_addr
                .map(|addr| format!("{:#x}", addr))
                .unwrap_or_else(|| "none".to_string());
            let importer_address = importer_addr
                .map(|addr| format!("{:#x}", addr))
                .unwrap_or_else(|| "none".to_string());
            log::info!(
                "event=eth_startup_deploy_ready token_address={:#x} presigner_address={} importer_address={}",
                token_addr,
                presigner_address,
                importer_address
            );
        }
        Ok(Err(error)) => log::error!(
            "event=eth_startup_deploy_failed contract_scope=wallet_transfer error={:?}",
            error
        ),
        Err(_) => log::error!(
            "event=eth_startup_deploy_timed_out contract_scope=wallet_transfer timeout_seconds={}",
            ETH_STARTUP_DEPLOY_TIMEOUT.as_secs()
        ),
    }
}
