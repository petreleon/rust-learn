use crate::db::schema::{
    courses, courses_organizations, notifications, organizations, user_role_course,
    user_role_organization, users, wallets,
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PlatformReportSummary {
    pub total_users: i64,
    pub total_organizations: i64,
    pub total_courses: i64,
    pub total_wallets: i64,
    pub total_notifications: i64,
}

#[derive(Debug, Serialize)]
pub struct OrganizationReportSummary {
    pub organization_id: i32,
    pub organization_name: String,
    pub course_count: i64,
    pub member_count: i64,
    pub wallet_count: i64,
    pub course_role_assignment_count: i64,
}

fn csv_value(value: impl AsRef<str>) -> String {
    let value = value.as_ref();
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

pub async fn platform_report_summary(
    conn: &mut AsyncPgConnection,
) -> QueryResult<PlatformReportSummary> {
    Ok(PlatformReportSummary {
        total_users: users::table.count().get_result(conn).await?,
        total_organizations: organizations::table.count().get_result(conn).await?,
        total_courses: courses::table.count().get_result(conn).await?,
        total_wallets: wallets::table.count().get_result(conn).await?,
        total_notifications: notifications::table.count().get_result(conn).await?,
    })
}

pub async fn organization_report_summary(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> QueryResult<OrganizationReportSummary> {
    let organization_name = organizations::table
        .find(organization_id)
        .select(organizations::name)
        .first::<String>(conn)
        .await?;

    let course_ids = courses_organizations::table
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select(courses_organizations::course_id)
        .load::<i32>(conn)
        .await?;
    let course_count = course_ids.len() as i64;

    let member_ids = user_role_organization::table
        .filter(user_role_organization::organization_id.eq(organization_id))
        .select(user_role_organization::user_id)
        .distinct()
        .load::<Option<i32>>(conn)
        .await?;
    let member_count = member_ids.into_iter().flatten().count() as i64;

    let wallet_count = wallets::table
        .filter(wallets::organization_id.eq(organization_id))
        .count()
        .get_result(conn)
        .await?;

    let course_role_assignment_count = if course_ids.is_empty() {
        0
    } else {
        user_role_course::table
            .filter(user_role_course::course_id.eq_any(course_ids))
            .count()
            .get_result(conn)
            .await?
    };

    Ok(OrganizationReportSummary {
        organization_id,
        organization_name,
        course_count,
        member_count,
        wallet_count,
        course_role_assignment_count,
    })
}

pub fn platform_report_csv(summary: &PlatformReportSummary) -> String {
    format!(
        "metric,value\nusers,{}\norganizations,{}\ncourses,{}\nwallets,{}\nnotifications,{}\n",
        summary.total_users,
        summary.total_organizations,
        summary.total_courses,
        summary.total_wallets,
        summary.total_notifications
    )
}

pub fn organization_report_csv(summary: &OrganizationReportSummary) -> String {
    format!(
        "metric,value\norganization_id,{}\norganization_name,{}\ncourses,{}\nmembers,{}\nwallets,{}\ncourse_role_assignments,{}\n",
        summary.organization_id,
        csv_value(&summary.organization_name),
        summary.course_count,
        summary.member_count,
        summary.wallet_count,
        summary.course_role_assignment_count
    )
}
