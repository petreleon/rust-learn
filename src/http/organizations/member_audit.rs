use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::db;
use crate::db::schema::organization_member_audit_events;
use crate::models::organization_member_audit_event::OrganizationMemberAuditEvent;

pub(super) async fn get_member_audit_route(
    path: web::Path<(i32, i32)>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
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
