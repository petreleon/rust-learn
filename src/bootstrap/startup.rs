use std::sync::Arc;
use std::time::Duration;

use actix_web::rt::time::timeout;

use crate::bootstrap::app_state::AppState;
use crate::bootstrap::readiness::RuntimeReadinessUseCase;
use crate::config::db_setup::version_updater;
use crate::db;
use crate::infra::postgres::access_control::role_catalog_use_case::PostgresRoleCatalogUseCase;
use crate::infra::postgres::content::chapter_use_cases::PostgresChapterUseCases;
use crate::infra::postgres::content::content_item_use_cases::PostgresContentItemUseCases;
use crate::infra::postgres::content::media_url_use_case::PostgresContentMediaUrlUseCase;
use crate::infra::postgres::content::processing_use_case::PostgresContentProcessingUseCase;
use crate::infra::postgres::content::upload_url_use_case::PostgresContentUploadUrlUseCase;
use crate::infra::postgres::notifications::notification_preferences_use_case::PostgresNotificationPreferencesUseCase;
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
        role_catalog_use_case: Arc::new(PostgresRoleCatalogUseCase::new(pool.clone())),
        notification_preferences_use_case: Arc::new(PostgresNotificationPreferencesUseCase::new(
            pool.clone(),
        )),
        chapter_use_cases: Arc::new(PostgresChapterUseCases::new(pool.clone())),
        content_item_use_cases: Arc::new(PostgresContentItemUseCases::new(pool.clone())),
        content_upload_url_use_case: Arc::new(PostgresContentUploadUrlUseCase::new(
            pool.clone(),
            s3.clone(),
        )),
        content_media_url_use_case: Arc::new(PostgresContentMediaUrlUseCase::new(
            pool.clone(),
            s3.clone(),
        )),
        content_processing_use_case: Arc::new(PostgresContentProcessingUseCase::new(pool.clone())),
        readiness_use_case: Arc::new(RuntimeReadinessUseCase::new(pool.clone(), s3.clone())),
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
