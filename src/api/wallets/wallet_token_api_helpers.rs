fn wallet_token_transfer_error_response(error: WalletTokenTransferError) -> HttpResponse {
    match error {
        WalletTokenTransferError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have wallet tax permission")
        }
        WalletTokenTransferError::KycRequired => {
            HttpResponse::Conflict().body("KYC verification is required before wallet actions")
        }
        WalletTokenTransferError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        WalletTokenTransferError::InsufficientFunds => {
            HttpResponse::Conflict().body("Insufficient wallet balance")
        }
        WalletTokenTransferError::Database(message) => {
            log::error!("event=wallet_token_transfer_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process wallet token transfer")
        }
    }
}
