use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};
use std::{env, str::FromStr};

use crate::application::wallet::create_deposit_intent::{
    WalletDepositGasPayer, WalletDepositIntentDraft, WalletDepositIntentError,
    WalletDepositIntentStore, WalletDepositIntentView,
};
use crate::db::schema::users;
use crate::infra::postgres::operations::persistent_state::get_persistent_state;
use crate::infra::postgres::wallet::wallet_deposit_intent_records::insert_deposit_intent;

const TOKEN_DEPOSIT_TAX_KEY: &str = "wallet.deposit_tax_tokens";
const PLATFORM_IMPORTER_ADDRESS_KEY: &str = "platform_importer_address";

pub struct PostgresWalletDepositIntentStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresWalletDepositIntentStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl WalletDepositIntentStore for PostgresWalletDepositIntentStore<'_> {
    fn user_kyc_verified(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletDepositIntentError>> {
        async move {
            users::table
                .find(user_id)
                .select(users::kyc_verified)
                .first::<bool>(self.conn)
                .await
                .map_err(|error| WalletDepositIntentError::KycLoad(error.to_string()))
        }
        .boxed()
    }

    fn load_platform_deposit_tax(
        &mut self,
    ) -> BoxFuture<'_, Result<BigDecimal, WalletDepositIntentError>> {
        async move {
            let Some(value) = get_persistent_state(self.conn, TOKEN_DEPOSIT_TAX_KEY)
                .await
                .map_err(|error| WalletDepositIntentError::TaxLoad(error.to_string()))?
            else {
                return Ok(BigDecimal::from(0));
            };

            BigDecimal::from_str(value.trim()).map_err(|_| {
                WalletDepositIntentError::TaxLoad(
                    "invalid stored deposit token tax amount".to_string(),
                )
            })
        }
        .boxed()
    }

    fn configured_deposit_platform_address(
        &mut self,
        gas_payer: WalletDepositGasPayer,
    ) -> BoxFuture<'_, Result<String, WalletDepositIntentError>> {
        async move { configured_deposit_platform_address(self.conn, gas_payer).await }.boxed()
    }

    fn create_deposit_intent(
        &mut self,
        user_id: i32,
        draft: WalletDepositIntentDraft,
    ) -> BoxFuture<'_, Result<WalletDepositIntentView, WalletDepositIntentError>> {
        async move { insert_deposit_intent(self.conn, user_id, draft).await }.boxed()
    }
}

async fn configured_deposit_platform_address(
    conn: &mut AsyncPgConnection,
    gas_payer: WalletDepositGasPayer,
) -> Result<String, WalletDepositIntentError> {
    let configured = match gas_payer {
        WalletDepositGasPayer::User => env::var("WALLET_DEPOSIT_TREASURY_ADDRESS")
            .ok()
            .or_else(|| env::var("PLATFORM_TREASURY").ok()),
        WalletDepositGasPayer::Platform => {
            get_persistent_state(conn, PLATFORM_IMPORTER_ADDRESS_KEY)
                .await
                .map_err(|error| WalletDepositIntentError::ConfigurationLoad(error.to_string()))?
                .or_else(|| env::var("WALLET_DEPOSIT_IMPORTER_ADDRESS").ok())
                .or_else(|| env::var("PLATFORM_IMPORTER_ADDRESS").ok())
        }
    };

    configured
        .map(|value| normalize_address(&value))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            WalletDepositIntentError::InvalidInput(
                "platform deposit receiver is not configured".to_string(),
            )
        })
}

fn normalize_address(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
