async fn get_organization_teacher_applications(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<OrganizationTeacherApplicationsRequest>,
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

    match crate::services::teacher_application_service::list_organization_applications(
        &mut conn,
        requester.user_id,
        organization_id,
        query.into_inner(),
    )
    .await
    {
        Ok(applications) => HttpResponse::Ok().json(applications),
        Err(TeacherApplicationError::PermissionDenied(_)) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization teacher applications"),
        Err(TeacherApplicationError::InvalidInput(message)) => {
            HttpResponse::BadRequest().body(message)
        }
        Err(TeacherApplicationError::InvalidTransition(message)) => {
            HttpResponse::Conflict().body(message)
        }
        Err(TeacherApplicationError::NotFound) => {
            HttpResponse::NotFound().body("Organization not found")
        }
        Err(TeacherApplicationError::Database(error)) => {
            log::error!(
                "event=organization_teacher_applications_fetch_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError()
                .body("Failed to fetch organization teacher applications")
        }
    }
}

async fn assign_role(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    body: web::Json<AssignRoleRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let (org_id, target_user_id) = path.into_inner();
    let role_name = &body.role_name;

    // Identify Requester from JWT
    let requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    match organization_service::assign_role(&pool, requester_id, target_user_id, org_id, role_name)
        .await
    {
        Ok(_) => {
            if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                if let Err(err) = notifications
                    .send_role_assignment_notification(
                        target_user_id,
                        "organization",
                        Some(org_id),
                        role_name,
                    )
                    .await
                {
                    log::warn!(
                        "event=notification_send_failed kind=role_assignment scope=organization organization_id={} target_user_id={} error={:?}",
                        org_id,
                        target_user_id,
                        err
                    );
                }
            }

            HttpResponse::Ok().body("Role assigned successfully")
        }
        Err(msg) => {
            if msg.contains("Hierarchy check failed") {
                HttpResponse::Forbidden().body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank.")
            } else if msg.contains("Role or User not found") {
                HttpResponse::BadRequest().body(msg)
            } else {
                log::error!(
                    "event=organization_role_assign_failed organization_id={} requester_user_id={} target_user_id={} role={} error={}",
                    org_id,
                    requester_id,
                    target_user_id,
                    role_name,
                    msg
                );
                HttpResponse::InternalServerError().body("Failed to assign role")
            }
        }
    }
}

async fn remove_organization_member_route(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let (org_id, target_user_id) = path.into_inner();
    let _requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    match organization_service::remove_organization_member(&pool, org_id, target_user_id).await {
        Ok(_) => HttpResponse::Ok().body("Member removed"),
        Err(msg) => {
            log::error!(
                "event=organization_member_remove_failed organization_id={} target_user_id={} error={}",
                org_id, target_user_id, msg
            );
            if msg.contains("User not found") {
                HttpResponse::NotFound().body(msg)
            } else {
                HttpResponse::InternalServerError().body("Failed to remove member")
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct AddMemberRequest {
    email: String,
    role_name: Option<String>,
}
