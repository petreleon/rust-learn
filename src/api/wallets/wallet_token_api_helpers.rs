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

async fn list_wallet_token_taxes(req: HttpRequest, pool: web::Data<db::DbPool>) -> impl Responder {
    if let Err(response) = authenticated_user(&req) {
        return response;
    }
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match wallet_service::get_wallet_token_taxes(&mut conn).await {
        Ok(settings) => HttpResponse::Ok().json(settings),
        Err(error) => wallet_token_transfer_error_response(error),
    }
}
