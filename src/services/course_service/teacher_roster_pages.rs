fn apply_join_request_status_filter<'a>(
    query: course_join_requests::BoxedQuery<'a, diesel::pg::Pg>,
    status: Option<&'a str>,
) -> course_join_requests::BoxedQuery<'a, diesel::pg::Pg> {
    match status {
        Some("all") | None => query,
        Some("open") => query.filter(
            course_join_requests::status
                .eq_any([COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED]),
        ),
        Some(status) => query.filter(course_join_requests::status.eq(status)),
    }
}

async fn load_teacher_course_roster_page(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    can_manage_enrollments: bool,
) -> Result<TeacherCourseRosterPage, TeacherCourseDashboardError> {
    let rows = user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .inner_join(users::table.on(user_role_course::user_id.eq(users::id.nullable())))
        .filter(user_role_course::course_id.eq(course_id))
        .filter(course_roles::name.eq("STUDENT"))
        .order(users::name.asc())
        .then_order_by(users::id.asc())
        .select((
            users::id,
            users::name,
            users::email,
            users::email_verified,
            users::kyc_verified,
        ))
        .load::<(i32, String, String, bool, bool)>(conn)
        .await?;

    let mut learners = Vec::with_capacity(rows.len());
    let mut seen_user_ids = BTreeSet::new();
    let course_reward_eligibility =
        load_teacher_course_reward_eligibility_summary(conn, course_id).await?;
    for (id, name, email, email_verified, kyc_verified) in rows {
        if !seen_user_ids.insert(id) {
            continue;
        }
        let roles = load_actor_course_roles(conn, id, course_id).await?;
        let latest_join_request_status =
            load_latest_join_request_status(conn, course_id, id).await?;
        let reward_eligibility =
            load_teacher_student_reward_eligibility(conn, course_id, id, &course_reward_eligibility)
                .await?;

        learners.push(TeacherCourseRosterLearner {
            user: TeacherEnrollmentUserSummary {
                id,
                name,
                email,
                email_verified,
                kyc_verified,
            },
            roles,
            latest_join_request_status,
            access_state: "enrolled".to_string(),
            can_remove: can_manage_enrollments,
            progress_supported: true,
            reward_eligibility_supported: reward_eligibility.supported,
            reward_eligibility,
        });
    }

    let total = learners.len() as i64;
    Ok(TeacherCourseRosterPage { learners, total })
}

async fn load_teacher_enrollment_user_summary(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<Option<TeacherEnrollmentUserSummary>, TeacherCourseDashboardError> {
    users::table
        .find(user_id)
        .select((
            users::id,
            users::name,
            users::email,
            users::email_verified,
            users::kyc_verified,
        ))
        .first::<(i32, String, String, bool, bool)>(conn)
        .await
        .optional()
        .map(|row| {
            row.map(|(id, name, email, email_verified, kyc_verified)| {
                TeacherEnrollmentUserSummary {
                    id,
                    name,
                    email,
                    email_verified,
                    kyc_verified,
                }
            })
        })
        .map_err(TeacherCourseDashboardError::from)
}

async fn load_latest_join_request_status(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    user_id: i32,
) -> Result<Option<String>, TeacherCourseDashboardError> {
    course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::requester_user_id.eq(user_id))
        .order(course_join_requests::updated_at.desc())
        .select(course_join_requests::status)
        .first::<String>(conn)
        .await
        .optional()
        .map_err(TeacherCourseDashboardError::from)
}
