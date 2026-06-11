async fn build_application_context(
    conn: &mut AsyncPgConnection,
    applications: &[TeacherApplication],
) -> Result<OrganizationApplicationContext, TeacherApplicationError> {
    let user_ids = applications
        .iter()
        .flat_map(|application| [Some(application.applicant_user_id), application.reviewer_id])
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let users = if user_ids.is_empty() {
        BTreeMap::new()
    } else {
        users::table
            .filter(users::id.eq_any(&user_ids))
            .select((users::id, users::name, users::email))
            .load::<(i32, String, String)>(conn)
            .await
            .map_err(TeacherApplicationError::from)?
            .into_iter()
            .map(|(id, name, email)| (id, TeacherApplicationUserSummary { id, name, email }))
            .collect()
    };

    let organization_ids = applications
        .iter()
        .flat_map(|application| {
            [
                application.requested_organization_id,
                application.organization_sponsor_id,
            ]
        })
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let organizations = if organization_ids.is_empty() {
        BTreeMap::new()
    } else {
        organizations::table
            .filter(organizations::id.eq_any(&organization_ids))
            .select((organizations::id, organizations::name))
            .load::<(i32, String)>(conn)
            .await
            .map_err(TeacherApplicationError::from)?
            .into_iter()
            .collect()
    };

    let course_ids = applications
        .iter()
        .filter_map(|application| application.requested_course_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let courses = if course_ids.is_empty() {
        BTreeMap::new()
    } else {
        courses::table
            .filter(courses::id.eq_any(&course_ids))
            .select((courses::id, courses::title))
            .load::<(i32, String)>(conn)
            .await
            .map_err(TeacherApplicationError::from)?
            .into_iter()
            .collect()
    };

    let application_ids = applications
        .iter()
        .map(|application| application.id)
        .collect::<Vec<_>>();
    let audits = if application_ids.is_empty() {
        BTreeMap::new()
    } else {
        let audit_events = teacher_application_audit_events::table
            .filter(teacher_application_audit_events::application_id.eq_any(&application_ids))
            .order(teacher_application_audit_events::created_at.asc())
            .then_order_by(teacher_application_audit_events::id.asc())
            .load::<TeacherApplicationAuditEvent>(conn)
            .await
            .map_err(TeacherApplicationError::from)?;
        build_audit_summaries(audit_events)
    };

    Ok(OrganizationApplicationContext {
        users,
        organizations,
        courses,
        audits,
    })
}

fn organization_application_item(
    application: &TeacherApplication,
    organization_id: i32,
    context: &OrganizationApplicationContext,
) -> OrganizationTeacherApplicationItem {
    OrganizationTeacherApplicationItem {
        id: application.id,
        applicant: context
            .users
            .get(&application.applicant_user_id)
            .cloned()
            .unwrap_or(TeacherApplicationUserSummary {
                id: application.applicant_user_id,
                name: "Unknown applicant".to_string(),
                email: "unknown@example.invalid".to_string(),
            }),
        requested_scope: application.requested_scope.clone(),
        requested_organization: application.requested_organization_id.map(|id| {
            TeacherApplicationOrganizationSummary {
                id,
                name: context
                    .organizations
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| format!("Organization {id}")),
            }
        }),
        requested_course: application.requested_course_id.map(|id| {
            TeacherApplicationCourseSummary {
                id,
                title: context
                    .courses
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| format!("Course {id}")),
            }
        }),
        sponsored_by_this_organization: application.organization_sponsor_id
            == Some(organization_id),
        requested_for_this_organization: application.requested_organization_id
            == Some(organization_id),
        experience_summary: application.experience_summary.clone(),
        portfolio_links: portfolio_links_from_json(&application.portfolio_links),
        status: application.status.clone(),
        reviewer: application
            .reviewer_id
            .and_then(|reviewer_id| context.users.get(&reviewer_id).cloned()),
        decision_reason: application.decision_reason.clone(),
        audit: context
            .audits
            .get(&application.id)
            .cloned()
            .unwrap_or_default(),
        created_at: application.created_at,
        updated_at: application.updated_at,
        decided_at: application.decided_at,
    }
}
