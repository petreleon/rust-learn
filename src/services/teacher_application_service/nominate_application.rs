pub async fn nominate_application(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    request: OrganizationTeacherNominationRequest,
) -> Result<TeacherApplication, TeacherApplicationError> {
    ensure_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW,
    )
    .await?;

    let requested_scope = request
        .requested_scope
        .clone()
        .unwrap_or_else(|| TEACHER_APPLICATION_SCOPE_ORGANIZATION.to_string());
    let submit_request = SubmitTeacherApplicationRequest {
        requested_scope,
        requested_organization_id: Some(organization_id),
        requested_course_id: request.requested_course_id,
        experience_summary: request.experience_summary,
        organization_sponsor_id: Some(organization_id),
        portfolio_links: request.portfolio_links,
        idempotency_key: request.idempotency_key,
    };
    let new_application = build_new_application(
        request.applicant_user_id,
        submit_request,
        Some(organization_id),
    )?;
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
                        event_type: "organization_nominated".to_string(),
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
        "event=teacher_application_transition application_id={} actor_user_id={} applicant_user_id={} transition=organization_nominated from_status=none to_status={} requested_scope={} organization_sponsor_id={:?} requested_organization_id={:?} requested_course_id={:?}",
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
