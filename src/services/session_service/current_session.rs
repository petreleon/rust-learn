pub async fn current_session(
    conn: &mut AsyncPgConnection,
    current_user_id: i32,
) -> Result<CurrentSessionResponse, CurrentSessionError> {
    let user = session_repository::find_user(conn, current_user_id).await?;
    if !user.email_verified {
        return Err(CurrentSessionError::EmailUnverified);
    }

    let mut platform = PlatformScopeBuilder {
        roles: session_repository::list_platform_roles(conn, current_user_id)
            .await?
            .into_iter()
            .collect(),
        direct_permissions: session_repository::list_platform_permissions(conn, current_user_id)
            .await?
            .into_iter()
            .collect(),
        delegated_permissions: BTreeSet::new(),
    };

    let mut organizations = BTreeMap::<i32, OrganizationScopeBuilder>::new();
    for row in session_repository::list_organization_roles(conn, current_user_id).await? {
        organization_builder(
            &mut organizations,
            row.organization_id,
            row.organization_name,
        )
        .roles
        .insert(row.role_name);
    }

    for row in session_repository::list_organization_permissions(conn, current_user_id).await? {
        organization_builder(
            &mut organizations,
            row.organization_id,
            row.organization_name,
        )
        .direct_permissions
        .insert(row.permission);
    }

    let mut courses = BTreeMap::<i32, CourseScopeBuilder>::new();
    for row in session_repository::list_course_roles(conn, current_user_id).await? {
        course_builder(
            &mut courses,
            row.course_id,
            row.course_title,
            row.lifecycle_status,
        )
        .roles
        .insert(row.role_name);
    }

    for row in session_repository::list_course_permissions(conn, current_user_id).await? {
        course_builder(
            &mut courses,
            row.course_id,
            row.course_title,
            row.lifecycle_status,
        )
        .direct_permissions
        .insert(row.permission);
    }

    let delegations = delegated_permission_repository::list_delegated_permissions(
        conn,
        DelegatedPermissionFilter {
            grantee_user_id: Some(current_user_id),
            active: Some(true),
            limit: Some(500),
            ..DelegatedPermissionFilter::default()
        },
    )
    .await?;

    let organization_label_ids = delegations
        .iter()
        .filter_map(|delegation| delegation.organization_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let organization_labels =
        session_repository::organization_labels(conn, &organization_label_ids)
            .await?
            .into_iter()
            .collect::<BTreeMap<_, _>>();

    let course_label_ids = delegations
        .iter()
        .filter_map(|delegation| delegation.course_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let course_labels = session_repository::course_labels(conn, &course_label_ids)
        .await?
        .into_iter()
        .map(|(id, title, lifecycle_status)| (id, (title, lifecycle_status)))
        .collect::<BTreeMap<_, _>>();

    for delegation in &delegations {
        match delegation.scope_type.as_str() {
            "platform" => {
                platform
                    .delegated_permissions
                    .insert(delegation.permission.clone());
            }
            "organization" => {
                if let Some(organization_id) = delegation.organization_id {
                    let organization_name = organization_labels
                        .get(&organization_id)
                        .cloned()
                        .unwrap_or_else(|| format!("Organization {organization_id}"));
                    organization_builder(&mut organizations, organization_id, organization_name)
                        .delegated_permissions
                        .insert(delegation.permission.clone());
                }
            }
            "course" => {
                if let Some(course_id) = delegation.course_id {
                    let (course_title, lifecycle_status) = course_labels
                        .get(&course_id)
                        .cloned()
                        .unwrap_or_else(|| (format!("Course {course_id}"), "unknown".to_string()));
                    course_builder(&mut courses, course_id, course_title, lifecycle_status)
                        .delegated_permissions
                        .insert(delegation.permission.clone());
                }
            }
            _ => {}
        }
    }

    Ok(CurrentSessionResponse {
        user: user.into(),
        platform: platform.into(),
        organizations: organizations
            .into_values()
            .map(OrganizationSessionScope::from)
            .collect(),
        courses: courses
            .into_values()
            .map(CourseSessionScope::from)
            .collect(),
        delegated_permissions: delegations
            .into_iter()
            .map(|delegation| {
                delegated_permission_session(delegation, &organization_labels, &course_labels)
            })
            .collect(),
    })
}
