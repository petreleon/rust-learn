use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::wallet::index_deposit::{
    ObservedWalletDepositEvent, WalletDepositIndexError, WalletDepositIndexOutput,
    WalletDepositIndexStore,
};
use crate::infra::postgres::wallet::wallet_deposit_index_records::index_observed_wallet_deposit;

pub struct PostgresWalletDepositIndexStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresWalletDepositIndexStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl WalletDepositIndexStore for PostgresWalletDepositIndexStore<'_> {
    fn index_observed_deposit(
        &mut self,
        event: ObservedWalletDepositEvent,
    ) -> BoxFuture<'_, Result<WalletDepositIndexOutput, WalletDepositIndexError>> {
        async move { index_observed_wallet_deposit(self.conn, event).await }.boxed()
    }
}
