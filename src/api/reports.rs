use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::middlewares::organization_permission_middleware::OrganizationPermissionMiddleware;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::models::param_type::ParamType;
use crate::services::reporting_service::{
    organization_report_csv, organization_report_summary, organization_reward_dashboard,
    organization_reward_dashboard_csv, platform_delegated_permissions_csv,
    platform_fraud_dashboard, platform_fraud_dashboard_csv, platform_report_csv,
    platform_report_summary, platform_reward_approvals_csv, platform_reward_dashboard,
    platform_reward_dashboard_csv, platform_teacher_applications_csv, platform_token_payouts_csv,
    platform_wallet_credits_csv, platform_wallet_reconciliation,
};
use actix_web::{web, HttpResponse, Responder};

fn csv_response(filename: &str, body: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/csv; charset=utf-8")
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", filename),
        ))
        .body(body)
}

async fn get_platform_summary(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_report_summary(&mut conn).await {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => {
            log::error!("event=report_load_failed scope=platform error={:?}", err);
            HttpResponse::InternalServerError().body("Failed to load platform report")
        }
    }
}

async fn export_platform_summary(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_report_summary(&mut conn).await {
        Ok(summary) => csv_response("platform-summary.csv", platform_report_csv(&summary)),
        Err(err) => {
            log::error!("event=report_export_failed scope=platform error={:?}", err);
            HttpResponse::InternalServerError().body("Failed to export platform report")
        }
    }
}

async fn get_platform_reward_dashboard(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_reward_dashboard(&mut conn).await {
        Ok(dashboard) => HttpResponse::Ok().json(dashboard),
        Err(err) => {
            log::error!(
                "event=report_load_failed scope=platform report=reward_dashboard error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to load reward dashboard")
        }
    }
}

async fn export_platform_reward_dashboard(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_reward_dashboard(&mut conn).await {
        Ok(dashboard) => csv_response(
            "platform-reward-dashboard.csv",
            platform_reward_dashboard_csv(&dashboard),
        ),
        Err(err) => {
            log::error!(
                "event=report_export_failed scope=platform report=reward_dashboard error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to export reward dashboard")
        }
    }
}

async fn get_platform_fraud_dashboard(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_fraud_dashboard(&mut conn).await {
        Ok(dashboard) => HttpResponse::Ok().json(dashboard),
        Err(err) => {
            log::error!(
                "event=report_load_failed scope=platform report=fraud_dashboard error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to load fraud dashboard")
        }
    }
}

async fn export_platform_fraud_dashboard(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_fraud_dashboard(&mut conn).await {
        Ok(dashboard) => csv_response(
            "platform-fraud-dashboard.csv",
            platform_fraud_dashboard_csv(&dashboard),
        ),
        Err(err) => {
            log::error!(
                "event=report_export_failed scope=platform report=fraud_dashboard error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to export fraud dashboard")
        }
    }
}

async fn export_platform_teacher_applications(pool: web::Data<db::DbPool>) -> impl Responder {
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match platform_teacher_applications_csv(&mut conn).await {
        Ok(csv) => csv_response("platform-teacher-applications.csv", csv),
        Err(err) => {
            log::error!(
                "event=report_export_failed scope=platform report=teacher_applications error={:?}",
                err
            );
            HttpResponse::InternalServerError().body("Failed to export teacher applications")
        }
    }
}

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
            log::error!("event=report_load_failed scope=platform report=wallet_reconciliation error={:?}", err);
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

async fn get_organization_summary(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match organization_report_summary(&mut conn, organization_id).await {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(err) => {
            log::error!(
                "event=report_load_failed scope=organization organization_id={} error={:?}",
                organization_id,
                err
            );
            HttpResponse::InternalServerError().body("Failed to load organization report")
        }
    }
}

async fn export_organization_summary(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match organization_report_summary(&mut conn, organization_id).await {
        Ok(summary) => csv_response(
            &format!("organization-{}-summary.csv", organization_id),
            organization_report_csv(&summary),
        ),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(err) => {
            log::error!(
                "event=report_export_failed scope=organization organization_id={} error={:?}",
                organization_id,
                err
            );
            HttpResponse::InternalServerError().body("Failed to export organization report")
        }
    }
}

async fn get_organization_reward_dashboard(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match organization_reward_dashboard(&mut conn, organization_id).await {
        Ok(dashboard) => HttpResponse::Ok().json(dashboard),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(err) => {
            log::error!(
                "event=report_load_failed scope=organization report=reward_dashboard organization_id={} error={:?}",
                organization_id,
                err
            );
            HttpResponse::InternalServerError().body("Failed to load organization reward dashboard")
        }
    }
}

async fn export_organization_reward_dashboard(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match organization_reward_dashboard(&mut conn, organization_id).await {
        Ok(dashboard) => csv_response(
            &format!("organization-{}-reward-dashboard.csv", organization_id),
            organization_reward_dashboard_csv(&dashboard),
        ),
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(err) => {
            log::error!(
                "event=report_export_failed scope=organization report=reward_dashboard organization_id={} error={:?}",
                organization_id,
                err
            );
            HttpResponse::InternalServerError()
                .body("Failed to export organization reward dashboard")
        }
    }
}

pub fn reports_scope() -> actix_web::Scope {
    web::scope("/reports")
        .service(
            web::resource("/platform/summary").route(web::get().to(get_platform_summary).wrap(
                PlatformPermissionMiddleware::require(Permissions::VIEW_REPORT.to_string()),
            )),
        )
        .service(
            web::resource("/platform/summary.csv").route(
                web::get()
                    .to(export_platform_summary)
                    .wrap(PlatformPermissionMiddleware::require(
                        Permissions::EXPORT_DATA.to_string(),
                    )),
            ),
        )
        .service(web::resource("/platform/reward-dashboard").route(
            web::get().to(get_platform_reward_dashboard).wrap(
                PlatformPermissionMiddleware::require(Permissions::VIEW_REWARD_AUDIT.to_string()),
            ),
        ))
        .service(web::resource("/platform/reward-dashboard.csv").route(
            web::get().to(export_platform_reward_dashboard).wrap(
                PlatformPermissionMiddleware::require(Permissions::EXPORT_DATA.to_string()),
            ),
        ))
        .service(web::resource("/platform/fraud-dashboard").route(
            web::get().to(get_platform_fraud_dashboard).wrap(
                PlatformPermissionMiddleware::require(Permissions::VIEW_REWARD_AUDIT.to_string()),
            ),
        ))
        .service(web::resource("/platform/fraud-dashboard.csv").route(
            web::get().to(export_platform_fraud_dashboard).wrap(
                PlatformPermissionMiddleware::require(Permissions::EXPORT_DATA.to_string()),
            ),
        ))
        .service(web::resource("/platform/teacher-applications.csv").route(
            web::get().to(export_platform_teacher_applications).wrap(
                PlatformPermissionMiddleware::require(Permissions::EXPORT_DATA.to_string()),
            ),
        ))
        .service(web::resource("/platform/reward-approvals.csv").route(
            web::get().to(export_platform_reward_approvals).wrap(
                PlatformPermissionMiddleware::require(Permissions::EXPORT_DATA.to_string()),
            ),
        ))
        .service(web::resource("/platform/token-payouts.csv").route(
            web::get().to(export_platform_token_payouts).wrap(
                PlatformPermissionMiddleware::require(Permissions::EXPORT_DATA.to_string()),
            ),
        ))
        .service(web::resource("/platform/wallet-credits.csv").route(
            web::get().to(export_platform_wallet_credits).wrap(
                PlatformPermissionMiddleware::require(Permissions::EXPORT_DATA.to_string()),
            ),
        ))
        .service(web::resource("/platform/delegated-permissions.csv").route(
            web::get().to(export_platform_delegated_permissions).wrap(
                PlatformPermissionMiddleware::require(Permissions::EXPORT_DATA.to_string()),
            ),
        ))
        .service(web::resource("/platform/wallet-reconciliation").route(
            web::get().to(get_platform_wallet_reconciliation).wrap(
                PlatformPermissionMiddleware::require(Permissions::MANAGE_WALLETS.to_string()),
            ),
        ))
        .service(web::resource("/organizations/{id}/summary").route(
            web::get().to(get_organization_summary).wrap(
                OrganizationPermissionMiddleware::require(
                    Permissions::VIEW_REPORT.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                ),
            ),
        ))
        .service(web::resource("/organizations/{id}/summary.csv").route(
            web::get().to(export_organization_summary).wrap(
                OrganizationPermissionMiddleware::require(
                    Permissions::GENERATE_REPORT.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                ),
            ),
        ))
        .service(web::resource("/organizations/{id}/reward-dashboard").route(
            web::get().to(get_organization_reward_dashboard).wrap(
                OrganizationPermissionMiddleware::require(
                    Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                ),
            ),
        ))
        .service(
            web::resource("/organizations/{id}/reward-dashboard.csv").route(
                web::get().to(export_organization_reward_dashboard).wrap(
                    OrganizationPermissionMiddleware::require(
                        Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    ),
                ),
            ),
        )
}
