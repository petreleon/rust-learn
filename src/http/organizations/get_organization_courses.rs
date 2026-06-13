async fn get_organization_courses(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<OrganizationCourseListParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let list_query = OrganizationCourseListQuery::new(
        query.search.clone(),
        query.lifecycle_status.clone(),
        query.reward_available,
        query.limit,
        query.offset,
    );

    match discover_organization_courses(&mut conn, requester.user_id, organization_id, list_query)
        .await
    {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(OrganizationCourseListError::PermissionDenied(_)) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization courses"),
        Err(OrganizationCourseListError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(OrganizationCourseListError::Database(error)) => {
            log::error!(
                "event=organization_courses_fetch_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to fetch organization courses")
        }
    }
}

async fn get_organization_members(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<OrganizationMemberListParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let list_query = OrganizationMemberListQuery::new(
        query.search.clone(),
        query.role.clone(),
        query.permission.clone(),
        query.limit,
        query.offset,
    );

    match organization_service::list_organization_members(
        &mut conn,
        requester.user_id,
        organization_id,
        list_query,
    )
    .await
    {
        Ok(members) => HttpResponse::Ok().json(members),
        Err(OrganizationMemberListError::PermissionDenied) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization members"),
        Err(OrganizationMemberListError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(OrganizationMemberListError::Database(error)) => {
            log::error!(
                "event=organization_members_fetch_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to fetch organization members")
        }
    }
}

async fn get_organization_dashboard(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let organization_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match organization_service::get_organization_dashboard(
        &mut conn,
        requester.user_id,
        organization_id,
    )
    .await
    {
        Ok(dashboard) => HttpResponse::Ok().json(dashboard),
        Err(OrganizationDashboardError::PermissionDenied) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization dashboard"),
        Err(OrganizationDashboardError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(OrganizationDashboardError::Database(error)) => {
            log::error!(
                "event=organization_dashboard_fetch_failed organization_id={} error={:?}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to fetch organization dashboard")
        }
        Err(OrganizationDashboardError::Reporting(error)) => {
            log::error!(
                "event=organization_dashboard_fetch_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to fetch organization dashboard")
        }
    }
}
