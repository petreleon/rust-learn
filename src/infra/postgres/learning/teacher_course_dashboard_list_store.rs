use diesel::prelude::*;
use diesel::{EscapeExpressionMethods, PgTextExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::list_teacher_course_dashboard::{
    TeacherCourseDashboardListOutput, TeacherCourseDashboardListQuery,
    TeacherCourseDashboardListStore,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::infra::postgres::learning::{
    teacher_course_dashboard_permissions, teacher_course_dashboard_scope,
    teacher_course_dashboard_summary_queries,
};
use crate::infra::postgres::models::course::Course;
use crate::infra::postgres::schema::courses;

const LIKE_ESCAPE_CHAR: char = '\\';

pub struct PostgresTeacherCourseDashboardListStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTeacherCourseDashboardListStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl TeacherCourseDashboardListStore for PostgresTeacherCourseDashboardListStore<'_> {
    fn list_teacher_course_dashboard(
        &mut self,
        query: TeacherCourseDashboardListQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseDashboardListOutput, TeacherCourseDashboardError>> {
        async move {
            let candidate_scope = teacher_course_dashboard_scope::teacher_course_candidate_scope(
                self.conn,
                query.actor_user_id,
            )
            .await?;
            if matches!(
                candidate_scope,
                teacher_course_dashboard_scope::TeacherCourseCandidateScope::CourseIds(ref ids)
                    if ids.is_empty()
            ) {
                return Ok(empty_dashboard(query));
            }

            let mut count_query = courses::table.into_boxed();
            let mut list_query = courses::table.into_boxed();
            match &candidate_scope {
                teacher_course_dashboard_scope::TeacherCourseCandidateScope::All => {}
                teacher_course_dashboard_scope::TeacherCourseCandidateScope::CourseIds(ids) => {
                    count_query = count_query.filter(courses::id.eq_any(ids));
                    list_query = list_query.filter(courses::id.eq_any(ids));
                }
            }
            if let Some(search) = query.search.as_deref() {
                let pattern = course_title_search_pattern(search);
                count_query = count_query.filter(
                    courses::title
                        .ilike(pattern.clone())
                        .escape(LIKE_ESCAPE_CHAR),
                );
                list_query =
                    list_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
            }
            if let Some(status) = query.lifecycle_status.as_deref() {
                count_query = count_query.filter(courses::lifecycle_status.eq(status));
                list_query = list_query.filter(courses::lifecycle_status.eq(status));
            }

            let total = count_query
                .count()
                .get_result(self.conn)
                .await
                .map_err(map_dashboard_error)?;
            let candidate_courses = list_query
                .order(courses::id.asc())
                .limit(query.limit)
                .offset(query.offset)
                .load::<Course>(self.conn)
                .await
                .map_err(map_dashboard_error)?;

            let mut items = Vec::new();
            for course in candidate_courses {
                let permissions =
                    teacher_course_dashboard_permissions::build_teacher_course_permissions(
                        self.conn,
                        query.actor_user_id,
                        course.id,
                    )
                    .await?;
                if !permissions.has_teacher_access() {
                    continue;
                }
                items.push(
                    teacher_course_dashboard_summary_queries::build_teacher_course_dashboard_item(
                        self.conn,
                        course,
                        permissions,
                    )
                    .await?,
                );
            }

            Ok(TeacherCourseDashboardListOutput {
                courses: items,
                total,
                limit: query.limit,
                offset: query.offset,
                search: query.search,
                lifecycle_status: query.lifecycle_status,
            })
        }
        .boxed()
    }
}

fn empty_dashboard(query: TeacherCourseDashboardListQuery) -> TeacherCourseDashboardListOutput {
    TeacherCourseDashboardListOutput {
        courses: Vec::new(),
        total: 0,
        limit: query.limit,
        offset: query.offset,
        search: query.search,
        lifecycle_status: query.lifecycle_status,
    }
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

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
