pub async fn list_platform_applications(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    request: PlatformTeacherApplicationsRequest,
) -> Result<PlatformTeacherApplicationsResponse, TeacherApplicationError> {
    ensure_platform_permission(
        conn,
        actor_user_id,
        Permissions::REVIEW_TEACHER_APPLICATIONS,
    )
    .await?;

    let status = match request.status {
        Some(status) => Some(normalize_status(&status)?),
        None => None,
    };
    let search = normalize_optional_text(request.search);
    let limit = request.limit.unwrap_or(25).clamp(1, 100);
    let offset = request.offset.unwrap_or(0).max(0);
    let can_approve_applications = user_has_platform_permission(
        conn,
        actor_user_id,
        Permissions::APPROVE_TEACHER_APPLICATION,
    )
    .await?;
    let can_reject_applications =
        user_has_platform_permission(conn, actor_user_id, Permissions::REJECT_TEACHER_APPLICATION)
            .await?;

    let applications = teacher_applications::table
        .order(teacher_applications::created_at.desc())
        .then_order_by(teacher_applications::id.desc())
        .load::<TeacherApplication>(conn)
        .await
        .map_err(TeacherApplicationError::from)?;

    let summary = teacher_application_summary(&applications);
    let context = build_application_context(conn, &applications).await?;
    let mut items = applications
        .iter()
        .map(|application| platform_application_item(application, &context))
        .collect::<Vec<_>>();

    if let Some(status) = status.as_deref() {
        items.retain(|application| application.status == status);
    }
    if let Some(search) = search.as_deref() {
        let normalized = search.to_lowercase();
        items.retain(|application| platform_application_matches_search(application, &normalized));
    }

    let total = items.len() as i64;
    let applications = items
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect::<Vec<_>>();

    Ok(PlatformTeacherApplicationsResponse {
        applications,
        summary,
        operator_permissions: PlatformTeacherApplicationPermissions {
            can_view_applications: true,
            can_approve_applications,
            can_reject_applications,
            can_request_changes: true,
        },
        total,
        limit,
        offset,
        status,
        search,
    })
}
