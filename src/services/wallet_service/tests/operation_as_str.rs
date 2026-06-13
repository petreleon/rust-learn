// ── WalletTokenOperation enum ──

#[test]
fn operation_as_str() {
    assert_eq!(
        WalletTokenOperation::Deposit.as_str(),
        TOKEN_TRANSFER_OPERATION_DEPOSIT
    );
    assert_eq!(
        WalletTokenOperation::Retire.as_str(),
        TOKEN_TRANSFER_OPERATION_RETIRE
    );
}

#[test]
fn operation_tax_key() {
    assert_eq!(
        WalletTokenOperation::Deposit.tax_key(),
        TOKEN_DEPOSIT_TAX_KEY
    );
    assert_eq!(WalletTokenOperation::Retire.tax_key(), TOKEN_RETIRE_TAX_KEY);
}

#[test]
fn operation_set_tax_permission() {
    assert_eq!(
        WalletTokenOperation::Deposit.set_tax_permission(),
        Permissions::SET_DEPOSIT_TAX
    );
    assert_eq!(
        WalletTokenOperation::Retire.set_tax_permission(),
        Permissions::SET_RETIRE_TAX
    );
}

#[test]
fn operation_transaction_type() {
    assert_eq!(
        WalletTokenOperation::Deposit.transaction_type(),
        TOKEN_DEPOSIT_TRANSACTION_TYPE
    );
    assert_eq!(
        WalletTokenOperation::Retire.transaction_type(),
        TOKEN_RETIRE_TRANSACTION_TYPE
    );
}

// ── WalletTokenGasPayer enum ──

#[test]
fn parse_valid_gas_payers() {
    assert_eq!(
        WalletTokenGasPayer::parse(TOKEN_TRANSFER_GAS_PAYER_USER).unwrap(),
        WalletTokenGasPayer::User
    );
    assert_eq!(
        WalletTokenGasPayer::parse(TOKEN_TRANSFER_GAS_PAYER_PLATFORM).unwrap(),
        WalletTokenGasPayer::Platform
    );
    assert_eq!(
        WalletTokenGasPayer::parse("USER").unwrap(),
        WalletTokenGasPayer::User
    );
    assert_eq!(
        WalletTokenGasPayer::parse("  platform  ").unwrap(),
        WalletTokenGasPayer::Platform
    );
}

#[test]
fn parse_invalid_gas_payer_fails() {
    assert!(WalletTokenGasPayer::parse("").is_err());
    assert!(WalletTokenGasPayer::parse("unknown").is_err());
    assert!(WalletTokenGasPayer::parse("admin").is_err());
}

#[test]
fn gas_payer_as_str() {
    assert_eq!(
        WalletTokenGasPayer::User.as_str(),
        TOKEN_TRANSFER_GAS_PAYER_USER
    );
    assert_eq!(
        WalletTokenGasPayer::Platform.as_str(),
        TOKEN_TRANSFER_GAS_PAYER_PLATFORM
    );
}
