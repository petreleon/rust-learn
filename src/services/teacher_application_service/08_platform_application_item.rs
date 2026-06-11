fn platform_application_item(
    application: &TeacherApplication,
    context: &OrganizationApplicationContext,
) -> PlatformTeacherApplicationItem {
    PlatformTeacherApplicationItem {
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
        sponsor_organization: application.organization_sponsor_id.map(|id| {
            TeacherApplicationOrganizationSummary {
                id,
                name: context
                    .organizations
                    .get(&id)
                    .cloned()
                    .unwrap_or_else(|| format!("Organization {id}")),
            }
        }),
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

fn build_audit_summaries(
    audit_events: Vec<TeacherApplicationAuditEvent>,
) -> BTreeMap<i64, TeacherApplicationAuditSummary> {
    let mut summaries = BTreeMap::<i64, TeacherApplicationAuditSummary>::new();
    for event in audit_events {
        let summary = summaries.entry(event.application_id).or_default();
        summary.event_count += 1;
        summary.latest_event_type = Some(event.event_type);
        summary.latest_event_at = Some(event.created_at);
        summary.latest_reason = event.reason;
    }
    summaries
}

fn teacher_application_summary(
    applications: &[TeacherApplication],
) -> TeacherApplicationDashboardSummary {
    let mut summary = TeacherApplicationDashboardSummary::default();
    for application in applications {
        summary.total += 1;
        match application.status.as_str() {
            TEACHER_APPLICATION_STATUS_APPROVED => summary.approved += 1,
            TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => summary.needs_changes += 1,
            TEACHER_APPLICATION_STATUS_REJECTED => summary.rejected += 1,
            TEACHER_APPLICATION_STATUS_SUBMITTED => summary.submitted += 1,
            _ => {}
        }
    }
    summary
}

fn organization_application_matches_search(
    application: &OrganizationTeacherApplicationItem,
    search: &str,
) -> bool {
    application.applicant.name.to_lowercase().contains(search)
        || application.applicant.email.to_lowercase().contains(search)
        || application
            .experience_summary
            .to_lowercase()
            .contains(search)
        || application.status.to_lowercase().contains(search)
        || application.requested_scope.to_lowercase().contains(search)
        || application
            .requested_course
            .as_ref()
            .is_some_and(|course| course.title.to_lowercase().contains(search))
        || application
            .requested_organization
            .as_ref()
            .is_some_and(|organization| organization.name.to_lowercase().contains(search))
}

fn platform_application_matches_search(
    application: &PlatformTeacherApplicationItem,
    search: &str,
) -> bool {
    application.id.to_string().contains(search)
        || application.applicant.id.to_string().contains(search)
        || application.applicant.name.to_lowercase().contains(search)
        || application.applicant.email.to_lowercase().contains(search)
        || application
            .experience_summary
            .to_lowercase()
            .contains(search)
        || application.status.to_lowercase().contains(search)
        || application.requested_scope.to_lowercase().contains(search)
        || application.requested_course.as_ref().is_some_and(|course| {
            course.id.to_string().contains(search) || course.title.to_lowercase().contains(search)
        })
        || application
            .requested_organization
            .as_ref()
            .is_some_and(|organization| {
                organization.id.to_string().contains(search)
                    || organization.name.to_lowercase().contains(search)
            })
        || application
            .sponsor_organization
            .as_ref()
            .is_some_and(|organization| {
                organization.id.to_string().contains(search)
                    || organization.name.to_lowercase().contains(search)
            })
}
