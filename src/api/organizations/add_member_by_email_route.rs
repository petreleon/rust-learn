async fn add_member_by_email_route(
    req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<AddMemberRequest>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    use crate::models::user::User;

    let org_id = path.into_inner();
    let requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let target_user = match User::find_by_email(body.email.trim(), &mut conn).await {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => {
            return HttpResponse::NotFound().body("User not found by email")
        }
        Err(e) => {
            log::error!(
                "event=org_member_add_user_lookup_failed email={} error={}",
                body.email,
                e
            );
            return HttpResponse::InternalServerError().body("Failed to look up user");
        }
    };

    let role_name = body.role_name.as_deref().unwrap_or("STUDENT");

    match organization_service::assign_role(&pool, requester_id, target_user.id, org_id, role_name)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "user_id": target_user.id,
            "name": target_user.name,
            "email": target_user.email,
            "role": role_name,
        })),
        Err(msg) => {
            log::error!(
                "event=org_member_add_failed org_id={} user_id={} error={}",
                org_id,
                target_user.id,
                msg
            );
            if msg.contains("Hierarchy") {
                HttpResponse::Forbidden().body(msg)
            } else {
                HttpResponse::InternalServerError().body("Failed to add member")
            }
        }
    }
}

async fn get_member_audit_route(
    path: web::Path<(i32, i32)>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    use crate::db::schema::organization_member_audit_events;
    use crate::models::organization_member_audit_event::OrganizationMemberAuditEvent;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let (org_id, user_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };

    match organization_member_audit_events::table
        .filter(organization_member_audit_events::organization_id.eq(org_id))
        .filter(organization_member_audit_events::target_user_id.eq(user_id))
        .order(organization_member_audit_events::created_at.desc())
        .load::<OrganizationMemberAuditEvent>(&mut conn)
        .await
    {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => {
            log::error!(
                "event=member_audit_fetch_failed org_id={} user_id={} error={}",
                org_id,
                user_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to load audit events")
        }
    }
}
