use crate::config::constants::permissions::Permissions;
use crate::db::schema::{courses, courses_organizations, organizations};
use crate::models::course::Course;
use diesel::prelude::*;
use diesel::{EscapeExpressionMethods, PgTextExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::catalog_dashboard_builders::build_organization_course_list_item;
use super::course_permission_summaries::build_organization_course_permissions;
use super::errors::OrganizationCourseListError;
use super::learner_course_types::LearnerCourseCatalogOrganization;
use super::query_types::OrganizationCourseListQuery;
use super::shared_helpers::course_title_search_pattern;
use super::teacher_course_types::OrganizationCourseListResponse;
use super::teacher_delegated_scope::user_has_platform_or_organization_permission;
use super::LIKE_ESCAPE_CHAR;

pub async fn discover_organization_courses(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
    list_query: OrganizationCourseListQuery,
) -> Result<OrganizationCourseListResponse, OrganizationCourseListError> {
    if !user_has_platform_or_organization_permission(
        conn,
        actor_user_id,
        organization_id,
        Permissions::VIEW_ORGANIZATION,
    )
    .await?
    {
        return Err(OrganizationCourseListError::PermissionDenied(
            Permissions::VIEW_ORGANIZATION.to_string(),
        ));
    }

    let (organization_id, organization_name) = organizations::table
        .find(organization_id)
        .select((organizations::id, organizations::name))
        .first::<(i32, String)>(conn)
        .await
        .map_err(OrganizationCourseListError::from)?;

    let mut course_query = courses_organizations::table
        .inner_join(courses::table.on(courses_organizations::course_id.eq(courses::id)))
        .filter(courses_organizations::organization_id.eq(organization_id))
        .into_boxed();

    if let Some(search) = list_query.search.as_deref() {
        let pattern = course_title_search_pattern(search);
        course_query = course_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
    }

    if let Some(status) = list_query.lifecycle_status.as_deref() {
        course_query = course_query.filter(courses::lifecycle_status.eq(status));
    }

    let candidate_courses = course_query
        .order(courses_organizations::order.asc())
        .then_order_by(courses::id.asc())
        .select(Course::as_select())
        .load::<Course>(conn)
        .await
        .map_err(OrganizationCourseListError::from)?;

    let permissions =
        build_organization_course_permissions(conn, actor_user_id, organization_id).await?;
    let mut items = Vec::new();
    for course in candidate_courses {
        let item = build_organization_course_list_item(conn, course, permissions.clone()).await?;
        if let Some(reward_available) = list_query.reward_available {
            if item.rewards.available != reward_available {
                continue;
            }
        }
        items.push(item);
    }

    let total = items.len() as i64;
    let courses = items
        .into_iter()
        .skip(list_query.offset as usize)
        .take(list_query.limit as usize)
        .collect();

    Ok(OrganizationCourseListResponse {
        organization: LearnerCourseCatalogOrganization {
            id: organization_id,
            name: organization_name,
        },
        courses,
        total,
        limit: list_query.limit,
        offset: list_query.offset,
        search: list_query.search,
        lifecycle_status: list_query.lifecycle_status,
        reward_available: list_query.reward_available,
    })
}
