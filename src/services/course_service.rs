use crate::db::schema::{courses, courses_organizations, pending_course_organization_invites};
use crate::models::course::{Course, NewCourse};
use crate::models::courses_organizations::NewCourseOrganization;
use crate::models::pending_course_organization_invites::{
    NewPendingCourseOrganizationInvite, PendingCourseOrganizationInvite,
};
use diesel::prelude::*;
use diesel::PgTextExpressionMethods;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::Serialize;

const DEFAULT_COURSE_LIMIT: i64 = 25;
const MAX_COURSE_LIMIT: i64 = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseDiscoveryQuery {
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize)]
pub struct CourseDiscoveryResponse {
    pub courses: Vec<Course>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub organization_id: Option<i32>,
}

impl CourseDiscoveryQuery {
    pub fn new(
        search: Option<String>,
        organization_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        let search = search
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let limit = limit
            .unwrap_or(DEFAULT_COURSE_LIMIT)
            .clamp(1, MAX_COURSE_LIMIT);
        let offset = offset.unwrap_or(0).max(0);

        CourseDiscoveryQuery {
            search,
            organization_id,
            limit,
            offset,
        }
    }
}

pub async fn discover_courses(
    conn: &mut AsyncPgConnection,
    discovery: CourseDiscoveryQuery,
) -> QueryResult<CourseDiscoveryResponse> {
    let mut count_query = courses::table.into_boxed();
    let mut list_query = courses::table.into_boxed();

    if let Some(search) = discovery.search.as_deref() {
        let pattern = format!("%{}%", search);
        count_query = count_query.filter(courses::title.ilike(pattern.clone()));
        list_query = list_query.filter(courses::title.ilike(pattern));
    }

    if let Some(organization_id) = discovery.organization_id {
        let course_ids = courses_organizations::table
            .filter(courses_organizations::organization_id.eq(organization_id))
            .select(courses_organizations::course_id);
        count_query = count_query.filter(courses::id.eq_any(course_ids));

        let course_ids = courses_organizations::table
            .filter(courses_organizations::organization_id.eq(organization_id))
            .select(courses_organizations::course_id);
        list_query = list_query.filter(courses::id.eq_any(course_ids));
    }

    let total = count_query.count().get_result(conn).await?;
    let courses = list_query
        .order(courses::id.asc())
        .limit(discovery.limit)
        .offset(discovery.offset)
        .load::<Course>(conn)
        .await?;

    Ok(CourseDiscoveryResponse {
        courses,
        total,
        limit: discovery.limit,
        offset: discovery.offset,
        search: discovery.search,
        organization_id: discovery.organization_id,
    })
}

pub async fn create_course_with_invites(
    conn: &mut AsyncPgConnection,
    title: String,
    organization_ids: Vec<i32>,
) -> QueryResult<Course> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let new_course = NewCourse { title: title };

            let course = diesel::insert_into(courses::table)
                .values(&new_course)
                .get_result::<Course>(conn)
                .await?;

            if let Some(first_org_id) = organization_ids.as_slice().first() {
                // Add first organization directly
                let new_link = NewCourseOrganization {
                    course_id: course.id,
                    organization_id: *first_org_id,
                    order: 0,
                };
                diesel::insert_into(courses_organizations::table)
                    .values(&new_link)
                    .execute(conn)
                    .await?;

                // Add remaining organizations as pending invites
                for org_id in organization_ids.iter().skip(1) {
                    create_course_organization_invite(conn, course.id, *org_id).await?;
                }
            }

            Ok(course)
        })
    })
    .await
}

pub async fn create_course_organization_invite(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
) -> QueryResult<usize> {
    let max_order_active: Option<i32> = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(diesel::dsl::max(courses_organizations::order))
        .first(conn)
        .await
        .optional()?
        .flatten();

    let max_order_pending: Option<i32> = pending_course_organization_invites::table
        .filter(pending_course_organization_invites::course_id.eq(course_id))
        .select(diesel::dsl::max(pending_course_organization_invites::order))
        .first(conn)
        .await
        .optional()?
        .flatten();

    let next_order = match (max_order_active, max_order_pending) {
        (Some(a), Some(b)) => std::cmp::max(a, b) + 1,
        (Some(a), None) => a + 1,
        (None, Some(b)) => b + 1,
        (None, None) => 0,
    };

    let new_invite = NewPendingCourseOrganizationInvite {
        course_id,
        organization_id,
        order: next_order,
    };
    diesel::insert_into(pending_course_organization_invites::table)
        .values(&new_invite)
        .execute(conn)
        .await
}

pub async fn accept_course_organization_invite(
    conn: &mut AsyncPgConnection,
    invite_id: i32,
) -> QueryResult<usize> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let invite = pending_course_organization_invites::table
                .find(invite_id)
                .first::<PendingCourseOrganizationInvite>(conn)
                .await?;

            let new_link = NewCourseOrganization {
                course_id: invite.course_id,
                organization_id: invite.organization_id,
                order: invite.order,
            };

            diesel::insert_into(courses_organizations::table)
                .values(&new_link)
                .execute(conn)
                .await?;

            diesel::delete(pending_course_organization_invites::table.find(invite_id))
                .execute(conn)
                .await
        })
    })
    .await
}
