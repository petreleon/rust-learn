async fn get_organization_reward_dashboard(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let from = parse_date_query(&query, "from");
    let to = parse_date_query(&query, "to");

    match organization_reward_dashboard(&mut conn, organization_id, from, to).await {
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
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let from = parse_date_query(&query, "from");
    let to = parse_date_query(&query, "to");

    match organization_reward_dashboard(&mut conn, organization_id, from, to).await {
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
