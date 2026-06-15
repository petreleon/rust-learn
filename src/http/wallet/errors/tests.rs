use super::{
    wallet_deposit_intent_error, wallet_link_error, wallet_read_error, wallet_token_tax_error,
};
use crate::application::wallet::create_deposit_intent::WalletDepositIntentError;
use crate::application::wallet::link_wallet::WalletLinkError;
use crate::application::wallet::manage_token_tax::WalletTokenTaxError;
use crate::application::wallet::read_wallet::WalletReadError;
use actix_web::{body::to_bytes, http::StatusCode, ResponseError};
use serde_json::Value;

#[actix_web::test]
async fn link_kyc_required_uses_conflict_envelope() {
    let body = parse_body(wallet_link_error(WalletLinkError::KycRequired).error_response()).await;

    assert_eq!(body.status, StatusCode::CONFLICT);
    assert_eq!(body.value["error"]["code"], "kyc_required");
    assert_eq!(
        body.value["error"]["message"],
        "KYC verification is required before wallet actions"
    );
}

#[actix_web::test]
async fn read_wallet_not_linked_uses_not_found_envelope() {
    let body =
        parse_body(wallet_read_error(WalletReadError::WalletNotLinked).error_response()).await;

    assert_eq!(body.status, StatusCode::NOT_FOUND);
    assert_eq!(body.value["error"]["code"], "wallet_not_linked");
    assert_eq!(body.value["error"]["message"], "Wallet not linked");
}

#[actix_web::test]
async fn deposit_invalid_input_keeps_message() {
    let body = parse_body(
        wallet_deposit_intent_error(WalletDepositIntentError::InvalidInput(
            "amount must be positive".to_string(),
        ))
        .error_response(),
    )
    .await;

    assert_eq!(body.status, StatusCode::BAD_REQUEST);
    assert_eq!(body.value["error"]["code"], "invalid_input");
    assert_eq!(body.value["error"]["message"], "amount must be positive");
}

#[actix_web::test]
async fn token_tax_permission_denied_uses_forbidden_envelope() {
    let body =
        parse_body(wallet_token_tax_error(WalletTokenTaxError::PermissionDenied).error_response())
            .await;

    assert_eq!(body.status, StatusCode::FORBIDDEN);
    assert_eq!(body.value["error"]["code"], "wallet_tax_permission_denied");
    assert_eq!(
        body.value["error"]["message"],
        "User does not have wallet tax permission"
    );
}

struct ParsedErrorBody {
    status: StatusCode,
    value: Value,
}

async fn parse_body(response: actix_web::HttpResponse) -> ParsedErrorBody {
    let status = response.status();
    let body = to_bytes(response.into_body()).await.unwrap();
    let value = serde_json::from_slice(&body).unwrap();
    ParsedErrorBody { status, value }
}
