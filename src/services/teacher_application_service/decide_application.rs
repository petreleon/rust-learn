pub async fn decide_application(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    application_id: i64,
    request: TeacherApplicationDecisionRequest,
) -> Result<TeacherApplication, TeacherApplicationError> {
    let target_status = normalize_decision_status(&request.status)?;
    ensure_decision_permission(conn, actor_user_id, &target_status).await?;

    let application = conn
        .transaction::<_, diesel::result::Error, _>(|conn| {
            Box::pin(async move {
                let current =
                    teacher_application_repository::find_application(conn, application_id).await?;
                if current.status == TEACHER_APPLICATION_STATUS_APPROVED
                    || current.status == TEACHER_APPLICATION_STATUS_REJECTED
                {
                    return Err(diesel::result::Error::RollbackTransaction);
                }

                let now = chrono::Utc::now();
                let updated = teacher_application_repository::update_application_decision(
                    conn,
                    application_id,
                    actor_user_id,
                    &target_status,
                    request.decision_reason.as_deref(),
                    now,
                )
                .await?;
                if target_status == TEACHER_APPLICATION_STATUS_APPROVED {
                    assign_approved_teaching_bundle(conn, &updated).await?;
                }
                teacher_application_repository::create_audit_event(
                    conn,
                    NewTeacherApplicationAuditEvent {
                        application_id,
                        actor_user_id: Some(actor_user_id),
                        event_type: target_status.clone(),
                        from_status: Some(current.status),
                        to_status: target_status,
                        reason: request.decision_reason,
                    },
                )
                .await?;
                Ok(updated)
            })
        })
        .await
        .map_err(|error| match error {
            diesel::result::Error::RollbackTransaction => {
                TeacherApplicationError::InvalidTransition(
                    "final teacher applications cannot be changed".to_string(),
                )
            }
            other => TeacherApplicationError::from(other),
        })?;

    log::info!(
        "event=teacher_application_transition application_id={} actor_user_id={} applicant_user_id={} transition={} to_status={} requested_scope={} organization_sponsor_id={:?} requested_organization_id={:?} requested_course_id={:?}",
        application.id,
        actor_user_id,
        application.applicant_user_id,
        application.status,
        application.status,
        application.requested_scope,
        application.organization_sponsor_id,
        application.requested_organization_id,
        application.requested_course_id
    );

    Ok(application)
}

struct OrganizationApplicationContext {
    users: BTreeMap<i32, TeacherApplicationUserSummary>,
    organizations: BTreeMap<i32, String>,
    courses: BTreeMap<i32, String>,
    audits: BTreeMap<i64, TeacherApplicationAuditSummary>,
}
