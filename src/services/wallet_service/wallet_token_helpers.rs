impl WalletTokenOperation {
    fn as_str(self) -> &'static str {
        match self {
            WalletTokenOperation::Deposit => TOKEN_TRANSFER_OPERATION_DEPOSIT,
            WalletTokenOperation::Retire => TOKEN_TRANSFER_OPERATION_RETIRE,
        }
    }

    fn tax_key(self) -> &'static str {
        match self {
            WalletTokenOperation::Deposit => TOKEN_DEPOSIT_TAX_KEY,
            WalletTokenOperation::Retire => TOKEN_RETIRE_TAX_KEY,
        }
    }

    fn set_tax_permission(self) -> Permissions {
        match self {
            WalletTokenOperation::Deposit => Permissions::SET_DEPOSIT_TAX,
            WalletTokenOperation::Retire => Permissions::SET_RETIRE_TAX,
        }
    }

}

impl WalletTokenGasPayer {
    fn parse(value: &str) -> Result<Self, WalletTokenTransferError> {
        match value.trim().to_ascii_lowercase().as_str() {
            TOKEN_TRANSFER_GAS_PAYER_USER => Ok(WalletTokenGasPayer::User),
            TOKEN_TRANSFER_GAS_PAYER_PLATFORM => Ok(WalletTokenGasPayer::Platform),
            _ => Err(WalletTokenTransferError::InvalidInput(
                "gas_payer must be 'user' or 'platform'".to_string(),
            )),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            WalletTokenGasPayer::User => TOKEN_TRANSFER_GAS_PAYER_USER,
            WalletTokenGasPayer::Platform => TOKEN_TRANSFER_GAS_PAYER_PLATFORM,
        }
    }
}

impl From<DieselError> for WalletTokenTransferError {
    fn from(error: DieselError) -> Self {
        WalletTokenTransferError::Database(error.to_string())
    }
}

fn is_unique_violation(error: &DieselError) -> bool {
    matches!(
        error,
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
    )
}

pub async fn find_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<Option<Wallet>> {
    wallets::table
        .filter(wallets::user_id.eq(user_id))
        .filter(wallets::organization_id.is_null())
        .first(conn)
        .await
        .optional()
}

pub async fn find_organization_wallet(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> QueryResult<Option<Wallet>> {
    wallets::table
        .filter(wallets::organization_id.eq(organization_id))
        .filter(wallets::user_id.is_null())
        .first(conn)
        .await
        .optional()
}

pub async fn link_user_wallet(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<LinkedWallet> {
    if let Some(wallet) = find_user_wallet(conn, user_id).await? {
        return Ok(LinkedWallet {
            wallet,
            created: false,
        });
    }

    let new_wallet = NewWallet {
        user_id: Some(user_id),
        organization_id: None,
        value: BigDecimal::from(0),
    };

    match diesel::insert_into(wallets::table)
        .values(&new_wallet)
        .get_result(conn)
        .await
    {
        Ok(wallet) => Ok(LinkedWallet {
            wallet,
            created: true,
        }),
        Err(error) if is_unique_violation(&error) => {
            let wallet = find_user_wallet(conn, user_id)
                .await?
                .ok_or(DieselError::NotFound)?;
            Ok(LinkedWallet {
                wallet,
                created: false,
            })
        }
        Err(error) => Err(error),
    }
}
