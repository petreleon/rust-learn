use diesel::prelude::*;
use diesel::{EscapeExpressionMethods, PgTextExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::list_organization_courses::{
    OrganizationCourseListError, OrganizationCourseListOutput, OrganizationCourseListQuery,
    OrganizationCourseListStore, OrganizationCourseSummaryOutput,
};
use crate::config::constants::permissions::Permissions;
use crate::infra::postgres::models::course::Course;
use crate::infra::postgres::organizations::{
    organization_course_permission_queries, organization_course_summary_queries,
};
use crate::infra::postgres::schema::{courses, courses_organizations, organizations};

const LIKE_ESCAPE_CHAR: char = '\\';

pub struct PostgresOrganizationCourseListStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationCourseListStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationCourseListStore for PostgresOrganizationCourseListStore<'_> {
    fn list_organization_courses(
        &mut self,
        query: OrganizationCourseListQuery,
    ) -> BoxFuture<'_, Result<OrganizationCourseListOutput, OrganizationCourseListError>> {
        async move {
            if !organization_course_permission_queries::can_view_organization_courses(
                self.conn,
                query.actor_user_id,
                query.organization_id,
            )
            .await?
            {
                return Err(OrganizationCourseListError::PermissionDenied(
                    Permissions::VIEW_ORGANIZATION.to_string(),
                ));
            }

            let organization = load_organization(self.conn, query.organization_id).await?;
            let candidate_courses = load_candidate_courses(self.conn, &query).await?;
            let permissions =
                organization_course_permission_queries::build_organization_course_permissions(
                    self.conn,
                    query.actor_user_id,
                    organization.id,
                )
                .await?;
            let mut items = Vec::new();
            for course in candidate_courses {
                let item =
                    organization_course_summary_queries::build_organization_course_list_item(
                        self.conn,
                        course,
                        permissions.clone(),
                    )
                    .await?;
                if query
                    .reward_available
                    .is_some_and(|reward_available| item.rewards.available != reward_available)
                {
                    continue;
                }
                items.push(item);
            }

            let total = items.len() as i64;
            let courses = items
                .into_iter()
                .skip(query.offset as usize)
                .take(query.limit as usize)
                .collect();

            Ok(OrganizationCourseListOutput {
                organization,
                courses,
                total,
                limit: query.limit,
                offset: query.offset,
                search: query.search,
                lifecycle_status: query.lifecycle_status,
                reward_available: query.reward_available,
            })
        }
        .boxed()
    }
}

async fn load_organization(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationCourseSummaryOutput, OrganizationCourseListError> {
    organizations::table
        .find(organization_id)
        .select((organizations::id, organizations::name))
        .first::<(i32, String)>(conn)
        .await
        .map(|(id, name)| OrganizationCourseSummaryOutput { id, name })
        .map_err(map_organization_error)
}

async fn load_candidate_courses(
    conn: &mut AsyncPgConnection,
    query: &OrganizationCourseListQuery,
) -> Result<Vec<Course>, OrganizationCourseListError> {
    let mut course_query = courses_organizations::table
        .inner_join(courses::table.on(courses_organizations::course_id.eq(courses::id)))
        .filter(courses_organizations::organization_id.eq(query.organization_id))
        .into_boxed();

    if let Some(search) = query.search.as_deref() {
        let pattern = course_title_search_pattern(search);
        course_query = course_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
    }
    if let Some(status) = query.lifecycle_status.as_deref() {
        course_query = course_query.filter(courses::lifecycle_status.eq(status));
    }

    course_query
        .order(courses_organizations::order.asc())
        .then_order_by(courses::id.asc())
        .select(Course::as_select())
        .load::<Course>(conn)
        .await
        .map_err(map_organization_error)
}

fn course_title_search_pattern(search: &str) -> String {
    let mut escaped = String::with_capacity(search.len());
    for ch in search.chars() {
        match ch {
            LIKE_ESCAPE_CHAR | '%' | '_' => {
                escaped.push(LIKE_ESCAPE_CHAR);
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }

    format!("%{}%", escaped)
}

fn map_organization_error(error: diesel::result::Error) -> OrganizationCourseListError {
    match error {
        diesel::result::Error::NotFound => OrganizationCourseListError::NotFound,
        other => OrganizationCourseListError::Database(other.to_string()),
    }
}
