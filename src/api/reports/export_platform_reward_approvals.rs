async fn export_platform_reward_approvals(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_reward_approvals_csv(&mut conn).await {
        Ok(csv) => csv_response("platform-reward-approvals.csv", csv),
        Err(err) => {
            log::error!(
                "event=report_export_failed scope=platform report=reward_approvals error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to export reward approvals")
        }
    }
}

async fn export_platform_token_payouts(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_token_payouts_csv(&mut conn).await {
        Ok(csv) => csv_response("platform-token-payouts.csv", csv),
        Err(err) => {
            log::error!(
                "event=report_export_failed scope=platform report=token_payouts error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to export token payouts")
        }
    }
}

async fn get_platform_wallet_reconciliation(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_wallet_reconciliation(&mut conn).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            log::error!(
                "event=report_load_failed scope=platform report=wallet_reconciliation error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to load wallet reconciliation")
        }
    }
}

async fn export_platform_wallet_credits(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_wallet_credits_csv(&mut conn).await {
        Ok(csv) => csv_response("platform-wallet-credits.csv", csv),
        Err(err) => {
            log::error!(
                "event=report_export_failed scope=platform report=wallet_credits error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to export wallet credits")
        }
    }
}

async fn export_platform_delegated_permissions(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_delegated_permissions_csv(&mut conn).await {
        Ok(csv) => csv_response("platform-delegated-permissions.csv", csv),
        Err(err) => {
            log::error!(
                "event=report_export_failed scope=platform report=delegated_permissions error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to export delegated permissions")
        }
    }
}
