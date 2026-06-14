async fn link_course_to_organization(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
) {
    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id,
            organization_id,
            order: 0,
        })
        .execute(conn)
        .await
        .expect("failed to link course to organization");
}

async fn force_assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_name: &str,
) {
    let role_id = role_catalog_store::course_role_id_by_name(conn, role_name)
        .await
        .expect("course role not found");
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

async fn create_custom_course_role(
    conn: &mut AsyncPgConnection,
    role_name: &str,
    permissions: &[Permissions],
) -> i32 {
    let role_id = diesel::insert_into(course_roles::table)
        .values((
            course_roles::name.eq(role_name),
            course_roles::description.eq(Some("test-only permission bundle".to_string())),
        ))
        .returning(course_roles::id)
        .get_result::<i32>(conn)
        .await
        .expect("failed to create custom course role");

    for permission in permissions {
        diesel::insert_into(role_permission_course::table)
            .values((
                role_permission_course::course_id.eq(None::<i32>),
                role_permission_course::course_role_id.eq(Some(role_id)),
                role_permission_course::permission.eq(permission.to_string()),
            ))
            .execute(conn)
            .await
            .expect("failed to assign custom course role permission");
    }

    role_id
}

async fn assign_course_role_id(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_id: i32,
) {
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign custom course role");
}

async fn force_assign_platform_role(conn: &mut AsyncPgConnection, user_id: i32, role_name: &str) {
    let role_id = role_catalog_store::platform_role_id_by_name(conn, role_name)
        .await
        .expect("platform role not found");
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign platform role");
}

async fn create_custom_platform_role(
    conn: &mut AsyncPgConnection,
    role_name: &str,
    permissions: &[Permissions],
) -> i32 {
    let role_id = diesel::insert_into(platform_roles::table)
        .values((
            platform_roles::name.eq(role_name),
            platform_roles::description
                .eq(Some("test-only platform permission bundle".to_string())),
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
    platform_role_records::assign_platform_role_to_user(conn, user_id, role_id)
        .await
        .expect("failed to assign custom platform role");
}
