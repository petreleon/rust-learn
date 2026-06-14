use crate::db::schema::users;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::support::WalletTokenTransferError;

pub(super) async fn ensure_user_kyc_verified(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<(), WalletTokenTransferError> {
    let verified = users::table
        .find(user_id)
        .select(users::kyc_verified)
        .first::<bool>(conn)
        .await?;

    if verified {
        Ok(())
    } else {
        Err(WalletTokenTransferError::KycRequired)
    }
}
