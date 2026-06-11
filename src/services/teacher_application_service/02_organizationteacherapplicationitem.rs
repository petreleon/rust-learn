#[derive(Debug, Clone, Serialize)]
pub struct OrganizationTeacherApplicationItem {
    pub id: i64,
    pub applicant: TeacherApplicationUserSummary,
    pub requested_scope: String,
    pub requested_organization: Option<TeacherApplicationOrganizationSummary>,
    pub requested_course: Option<TeacherApplicationCourseSummary>,
    pub sponsored_by_this_organization: bool,
    pub requested_for_this_organization: bool,
    pub experience_summary: String,
    pub portfolio_links: Vec<String>,
    pub status: String,
    pub reviewer: Option<TeacherApplicationUserSummary>,
    pub decision_reason: Option<String>,
    pub audit: TeacherApplicationAuditSummary,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub decided_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeacherApplicationUserSummary {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeacherApplicationOrganizationSummary {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeacherApplicationCourseSummary {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TeacherApplicationAuditSummary {
    pub event_count: usize,
    pub latest_event_type: Option<String>,
    pub latest_event_at: Option<chrono::DateTime<chrono::Utc>>,
    pub latest_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TeacherApplicationDashboardSummary {
    pub approved: i64,
    pub needs_changes: i64,
    pub rejected: i64,
    pub submitted: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationTeacherApplicationPermissions {
    pub can_view_applications: bool,
    pub can_nominate_teachers: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformTeacherApplicationPermissions {
    pub can_view_applications: bool,
    pub can_approve_applications: bool,
    pub can_reject_applications: bool,
    pub can_request_changes: bool,
}

pub async fn submit_application(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: SubmitTeacherApplicationRequest,
) -> Result<TeacherApplication, TeacherApplicationError> {
    ensure_platform_permission(conn, actor_user_id, Permissions::SUBMIT_TEACHER_APPLICATION)
        .await?;

    let new_application = build_new_application(actor_user_id, request, None)?;
    if let Some(existing) = find_idempotent_application(conn, &new_application).await? {
        log::info!(
            "event=teacher_application_idempotent_replay application_id={} actor_user_id={} applicant_user_id={} status={} requested_scope={} idempotency_key={}",
            existing.id,
            actor_user_id,
            existing.applicant_user_id,
            existing.status,
            existing.requested_scope,
            existing.idempotency_key.as_deref().unwrap_or("none")
        );
        return Ok(existing);
    }
    ensure_no_blocking_application(conn, &new_application).await?;

    let application = conn
        .transaction::<_, diesel::result::Error, _>(|conn| {
            Box::pin(async move {
                let application =
                    teacher_application_repository::create_application(conn, new_application)
                        .await?;
                teacher_application_repository::create_audit_event(
                    conn,
                    NewTeacherApplicationAuditEvent {
                        application_id: application.id,
                        actor_user_id: Some(actor_user_id),
                        event_type: "submitted".to_string(),
                        from_status: None,
                        to_status: application.status.clone(),
                        reason: None,
                    },
                )
                .await?;
                Ok(application)
            })
        })
        .await
        .map_err(TeacherApplicationError::from)?;

    log::info!(
        "event=teacher_application_transition application_id={} actor_user_id={} applicant_user_id={} transition=submitted from_status=none to_status={} requested_scope={} organization_sponsor_id={:?} requested_organization_id={:?} requested_course_id={:?}",
        application.id,
        actor_user_id,
        application.applicant_user_id,
        application.status,
        application.requested_scope,
        application.organization_sponsor_id,
        application.requested_organization_id,
        application.requested_course_id
    );

    Ok(application)
}
