use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};
use std::str::FromStr;

use crate::application::wallet::retire_tokens::{
    WalletRetirementDraft, WalletRetirementError, WalletRetirementStore, WalletRetirementView,
};
use crate::db::schema::users;
use crate::infra::postgres::wallet::wallet_retirement_records::create_wallet_retirement;
use crate::repositories::persistent_state_repository::get_persistent_state;

const TOKEN_RETIRE_TAX_KEY: &str = "wallet.retire_tax_tokens";

pub struct PostgresWalletRetirementStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresWalletRetirementStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl WalletRetirementStore for PostgresWalletRetirementStore<'_> {
    fn user_kyc_verified(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<bool, WalletRetirementError>> {
        async move {
            users::table
                .find(user_id)
                .select(users::kyc_verified)
                .first::<bool>(self.conn)
                .await
                .map_err(|error| WalletRetirementError::KycLoad(error.to_string()))
        }
        .boxed()
    }

    fn load_platform_retire_tax(
        &mut self,
    ) -> BoxFuture<'_, Result<BigDecimal, WalletRetirementError>> {
        async move {
            let Some(value) = get_persistent_state(self.conn, TOKEN_RETIRE_TAX_KEY)
                .await
                .map_err(|error| WalletRetirementError::TaxLoad(error.to_string()))?
            else {
                return Ok(BigDecimal::from(0));
            };

            BigDecimal::from_str(value.trim()).map_err(|_| {
                WalletRetirementError::TaxLoad("invalid stored retire token tax amount".to_string())
            })
        }
        .boxed()
    }

    fn retire_tokens(
        &mut self,
        user_id: i32,
        draft: WalletRetirementDraft,
    ) -> BoxFuture<'_, Result<WalletRetirementView, WalletRetirementError>> {
        async move { create_wallet_retirement(self.conn, user_id, draft).await }.boxed()
    }
}
