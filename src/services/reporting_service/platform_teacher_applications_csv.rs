pub async fn platform_teacher_applications_csv(
    conn: &mut AsyncPgConnection,
) -> QueryResult<String> {
    let applications = teacher_applications::table
        .order(teacher_applications::created_at.desc())
        .limit(1000)
        .load::<TeacherApplication>(conn)
        .await?;

    let mut csv = String::from(
        "application_id,applicant_user_id,requested_scope,requested_organization_id,requested_course_id,organization_sponsor_id,status,reviewer_id,decision_reason,portfolio_links,created_at,updated_at,decided_at\n",
    );
    for application in applications {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            application.id,
            application.applicant_user_id,
            csv_value(&application.requested_scope),
            csv_optional(application.requested_organization_id),
            csv_optional(application.requested_course_id),
            csv_optional(application.organization_sponsor_id),
            csv_value(&application.status),
            csv_optional(application.reviewer_id),
            csv_value(application.decision_reason.as_deref().unwrap_or("")),
            csv_value(application.portfolio_links.to_string()),
            application.created_at,
            application.updated_at,
            csv_optional(application.decided_at)
        ));
    }

    Ok(csv)
}
