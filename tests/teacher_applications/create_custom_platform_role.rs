async fn create_custom_platform_role(
    conn: &mut AsyncPgConnection,
    role_name: &str,
    permissions: &[Permissions],
) -> i32 {
    let role_id = diesel::insert_into(platform_roles::table)
        .values((
            platform_roles::name.eq(role_name),
            platform_roles::description.eq(Some(
                "test-only teacher application permission bundle".to_string(),
            )),
        ))
        .returning(platform_roles::id)
        .get_result::<i32>(conn)
        .await
        .expect("failed to create custom platform role");

    for permission in permissions {
        diesel::insert_into(role_permission_platform::table)
            .values((
                role_permission_platform::platform_role_id.eq(Some(role_id)),
                role_permission_platform::permission.eq(permission.to_string()),
            ))
            .execute(conn)
            .await
            .expect("failed to assign custom platform role permission");
    }

    role_id
}

async fn assign_platform_role_id(conn: &mut AsyncPgConnection, user_id: i32, role_id: i32) {
    UserRolePlatform::assign(conn, user_id, role_id)
        .await
        .expect("failed to assign custom platform role");
}

fn platform_application_request() -> SubmitTeacherApplicationRequest {
    SubmitTeacherApplicationRequest {
        requested_scope: "platform".to_string(),
        requested_organization_id: None,
        requested_course_id: None,
        experience_summary: "Five years teaching Rust and mentoring students.".to_string(),
        organization_sponsor_id: None,
        portfolio_links: Some(vec![
            "https://example.com/portfolio".to_string(),
            "   ".to_string(),
        ]),
        idempotency_key: None,
    }
}
