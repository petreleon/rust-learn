use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::middlewares::organization_permission_middleware::OrganizationPermissionMiddleware;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::models::param_type::ParamType;
use crate::services::reporting_service::{
    organization_report_csv, organization_report_summary, platform_fraud_dashboard,
    platform_fraud_dashboard_csv, platform_report_csv, platform_report_summary,
    platform_reward_dashboard, platform_reward_dashboard_csv,
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

pub fn reports_scope() -> actix_web::Scope {
    web::scope("/reports")
        .service(
            web::resource("/platform/summary").route(web::get().to(get_platform_summary).wrap(
                PlatformPermissionMiddleware::new(Permissions::VIEW_REPORT.to_string()),
            )),
        )
        .service(
            web::resource("/platform/summary.csv").route(
                web::get()
                    .to(export_platform_summary)
                    .wrap(PlatformPermissionMiddleware::new(
                        Permissions::EXPORT_DATA.to_string(),
                    )),
            ),
        )
        .service(
            web::resource("/platform/reward-dashboard").route(
                web::get().to(get_platform_reward_dashboard).wrap(
                    PlatformPermissionMiddleware::new(Permissions::VIEW_REWARD_AUDIT.to_string()),
                ),
            ),
        )
        .service(web::resource("/platform/reward-dashboard.csv").route(
            web::get().to(export_platform_reward_dashboard).wrap(
                PlatformPermissionMiddleware::new(Permissions::EXPORT_DATA.to_string()),
            ),
        ))
        .service(
            web::resource("/platform/fraud-dashboard").route(
                web::get().to(get_platform_fraud_dashboard).wrap(
                    PlatformPermissionMiddleware::new(Permissions::VIEW_REWARD_AUDIT.to_string()),
                ),
            ),
        )
        .service(
            web::resource("/platform/fraud-dashboard.csv").route(
                web::get().to(export_platform_fraud_dashboard).wrap(
                    PlatformPermissionMiddleware::new(Permissions::EXPORT_DATA.to_string()),
                ),
            ),
        )
        .service(
            web::resource("/organizations/{id}/summary").route(
                web::get().to(get_organization_summary).wrap(
                    OrganizationPermissionMiddleware::new(
                        Permissions::VIEW_REPORT.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    ),
                ),
            ),
        )
        .service(
            web::resource("/organizations/{id}/summary.csv").route(
                web::get().to(export_organization_summary).wrap(
                    OrganizationPermissionMiddleware::new(
                        Permissions::GENERATE_REPORT.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    ),
                ),
            ),
        )
}
