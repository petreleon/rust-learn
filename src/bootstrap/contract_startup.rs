use std::time::Duration;

use actix_web::rt::time::timeout;

const ETH_STARTUP_DEPLOY_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) async fn deploy_startup_contracts(conn: &mut diesel_async::AsyncPgConnection) {
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
