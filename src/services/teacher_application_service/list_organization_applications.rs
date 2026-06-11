pub async fn list_organization_applications(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    request: OrganizationTeacherApplicationsRequest,
) -> Result<OrganizationTeacherApplicationsResponse, TeacherApplicationError> {
    let organization = organizations::table
        .find(organization_id)
        .select((organizations::id, organizations::name))
        .first::<(i32, String)>(conn)
        .await
        .map_err(TeacherApplicationError::from)?;

    let can_view_applications = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORG_TEACHER_APPLICATIONS,
    )
    .await?;
    let can_nominate_teachers = user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW,
    )
    .await?;

    if !can_view_applications && !can_nominate_teachers {
        return Err(TeacherApplicationError::PermissionDenied(
            Permissions::VIEW_ORG_TEACHER_APPLICATIONS.to_string(),
        ));
    }

    let status = match request.status {
        Some(status) => Some(normalize_status(&status)?),
        None => None,
    };
    let search = normalize_optional_text(request.search);
    let limit = request.limit.unwrap_or(25).clamp(1, 100);
    let offset = request.offset.unwrap_or(0).max(0);

    let applications = teacher_applications::table
        .filter(
            teacher_applications::organization_sponsor_id
                .eq(Some(organization_id))
                .or(teacher_applications::requested_organization_id.eq(Some(organization_id))),
        )
        .order(teacher_applications::created_at.desc())
        .then_order_by(teacher_applications::id.desc())
        .load::<TeacherApplication>(conn)
        .await
        .map_err(TeacherApplicationError::from)?;

    let summary = teacher_application_summary(&applications);
    let context = build_application_context(conn, &applications).await?;
    let mut items = applications
        .iter()
        .map(|application| organization_application_item(application, organization_id, &context))
        .collect::<Vec<_>>();

    if let Some(status) = status.as_deref() {
        items.retain(|application| application.status == status);
    }
    if let Some(search) = search.as_deref() {
        let normalized = search.to_lowercase();
        items.retain(|application| {
            organization_application_matches_search(application, &normalized)
        });
    }

    let total = items.len() as i64;
    let applications = items
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect::<Vec<_>>();

    Ok(OrganizationTeacherApplicationsResponse {
        organization: OrganizationTeacherApplicationsOrganization {
            id: organization.0,
            name: organization.1,
        },
        applications,
        summary,
        operator_permissions: OrganizationTeacherApplicationPermissions {
            can_view_applications,
            can_nominate_teachers,
        },
        total,
        limit,
        offset,
        status,
        search,
    })
}
