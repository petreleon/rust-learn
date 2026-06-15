use diesel::prelude::*;
use diesel::{EscapeExpressionMethods, PgTextExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::discover_courses::{
    CourseDiscoveryCourseOutput, CourseDiscoveryError, CourseDiscoveryOutput, CourseDiscoveryQuery,
    CourseDiscoveryStore,
};
use crate::infra::postgres::models::course::Course;
use crate::infra::postgres::schema::{courses, courses_organizations};

const LIKE_ESCAPE_CHAR: char = '\\';

pub struct PostgresCourseDiscoveryStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseDiscoveryStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseDiscoveryStore for PostgresCourseDiscoveryStore<'_> {
    fn discover(
        &mut self,
        query: CourseDiscoveryQuery,
    ) -> BoxFuture<'_, Result<CourseDiscoveryOutput, CourseDiscoveryError>> {
        async move { discover_courses(&mut *self.conn, query).await }.boxed()
    }
}

async fn discover_courses(
    conn: &mut AsyncPgConnection,
    query: CourseDiscoveryQuery,
) -> Result<CourseDiscoveryOutput, CourseDiscoveryError> {
    let mut count_query = courses::table.into_boxed();
    let mut list_query = courses::table.into_boxed();

    if let Some(search) = query.search.as_deref() {
        let pattern = course_title_search_pattern(search);
        count_query = count_query.filter(
            courses::title
                .ilike(pattern.clone())
                .escape(LIKE_ESCAPE_CHAR),
        );
        list_query = list_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
    }

    if let Some(organization_id) = query.organization_id {
        let course_ids_for_organization = || {
            courses_organizations::table
                .filter(courses_organizations::organization_id.eq(organization_id))
                .select(courses_organizations::course_id)
        };
        count_query = count_query.filter(courses::id.eq_any(course_ids_for_organization()));
        list_query = list_query.filter(courses::id.eq_any(course_ids_for_organization()));
    }

    let total = count_query
        .count()
        .get_result(conn)
        .await
        .map_err(map_course_discovery_error)?;
    let courses = list_query
        .order(courses::id.asc())
        .limit(query.limit)
        .offset(query.offset)
        .load::<Course>(conn)
        .await
        .map_err(map_course_discovery_error)?
        .into_iter()
        .map(course_output_from_model)
        .collect();

    Ok(CourseDiscoveryOutput {
        courses,
        total,
        limit: query.limit,
        offset: query.offset,
        search: query.search,
        organization_id: query.organization_id,
    })
}

fn map_course_discovery_error(error: diesel::result::Error) -> CourseDiscoveryError {
    CourseDiscoveryError::Database(error.to_string())
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

fn course_output_from_model(course: Course) -> CourseDiscoveryCourseOutput {
    CourseDiscoveryCourseOutput {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        description: course.description,
        topics: course.topics,
        prerequisites: course.prerequisites,
    }
}
