use std::sync::Arc;

use actix_web::web;

use crate::application::wallet::audit_wallet::WalletAuditUseCase;
use crate::application::wallet::create_deposit_intent::WalletDepositIntentUseCase;
use crate::application::wallet::link_wallet::WalletLinkUseCase;
use crate::application::wallet::manage_token_tax::WalletTokenTaxUseCase;
use crate::application::wallet::read_wallet::WalletReadUseCase;
use crate::application::wallet::retire_tokens::WalletRetirementUseCase;
use crate::infra::postgres::wallet::wallet_audit_use_case::PostgresWalletAuditUseCase;
use crate::infra::postgres::wallet::wallet_deposit_intent_use_case::PostgresWalletDepositIntentUseCase;
use crate::infra::postgres::wallet::wallet_link_use_case::PostgresWalletLinkUseCase;
use crate::infra::postgres::wallet::wallet_read_use_case::PostgresWalletReadUseCase;
use crate::infra::postgres::wallet::wallet_retirement_use_case::PostgresWalletRetirementUseCase;
use crate::infra::postgres::wallet::wallet_token_tax_use_case::PostgresWalletTokenTaxUseCase;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct WalletUseCases {
    pub audit: Arc<dyn WalletAuditUseCase>,
    pub deposit_intent: Arc<dyn WalletDepositIntentUseCase>,
    pub link: Arc<dyn WalletLinkUseCase>,
    pub read: Arc<dyn WalletReadUseCase>,
    pub retirement: Arc<dyn WalletRetirementUseCase>,
    pub token_tax: Arc<dyn WalletTokenTaxUseCase>,
}

pub fn build_wallet_use_cases(pool: &DbPool) -> WalletUseCases {
    WalletUseCases {
        audit: Arc::new(PostgresWalletAuditUseCase::new(pool.clone())),
        deposit_intent: Arc::new(PostgresWalletDepositIntentUseCase::new(pool.clone())),
        link: Arc::new(PostgresWalletLinkUseCase::new(pool.clone())),
        read: Arc::new(PostgresWalletReadUseCase::new(pool.clone())),
        retirement: Arc::new(PostgresWalletRetirementUseCase::new(pool.clone())),
        token_tax: Arc::new(PostgresWalletTokenTaxUseCase::new(pool.clone())),
    }
}

pub fn configure_wallet_app_data(cfg: &mut web::ServiceConfig, use_cases: &WalletUseCases) {
    cfg.app_data(web::Data::new(use_cases.audit.clone()))
        .app_data(web::Data::new(use_cases.deposit_intent.clone()))
        .app_data(web::Data::new(use_cases.link.clone()))
        .app_data(web::Data::new(use_cases.read.clone()))
        .app_data(web::Data::new(use_cases.retirement.clone()))
        .app_data(web::Data::new(use_cases.token_tax.clone()));
}
