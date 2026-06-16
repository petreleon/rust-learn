use diesel::prelude::*;
use diesel::{EscapeExpressionMethods, PgTextExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::learner_course_catalog::LearnerCourseCatalogError;
use crate::application::learning::list_learner_course_catalog::{
    LearnerCourseCatalogListStore, LearnerCourseCatalogOutput, LearnerCourseCatalogQuery,
};
use crate::domain::learning::course::is_generated_course_title;
use crate::infra::postgres::learning::{
    learner_course_access_queries, learner_course_catalog_item_queries,
};
use crate::infra::postgres::models::course::Course;
use crate::infra::postgres::schema::{courses, courses_organizations};

const LIKE_ESCAPE_CHAR: char = '\\';

pub struct PostgresLearnerCourseCatalogListStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresLearnerCourseCatalogListStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl LearnerCourseCatalogListStore for PostgresLearnerCourseCatalogListStore<'_> {
    fn list_catalog(
        &mut self,
        query: LearnerCourseCatalogQuery,
    ) -> BoxFuture<'_, Result<LearnerCourseCatalogOutput, LearnerCourseCatalogError>> {
        async move {
            let mut course_query = courses::table.into_boxed();
            if let Some(search) = query.search.as_deref() {
                let pattern = course_title_search_pattern(search);
                course_query =
                    course_query.filter(courses::title.ilike(pattern).escape(LIKE_ESCAPE_CHAR));
            }
            if let Some(organization_id) = query.organization_id {
                course_query = course_query.filter(
                    courses::id.eq_any(
                        courses_organizations::table
                            .filter(courses_organizations::organization_id.eq(organization_id))
                            .select(courses_organizations::course_id),
                    ),
                );
            }
            if let Some(status) = query.lifecycle_status.as_deref() {
                course_query = course_query.filter(courses::lifecycle_status.eq(status));
            }

            let candidate_courses = course_query
                .order(courses::id.asc())
                .load::<Course>(self.conn)
                .await
                .map_err(map_catalog_error)?;
            let mut items = Vec::new();
            for course in candidate_courses {
                if !learner_course_access_queries::course_visible_to_learner(
                    self.conn,
                    query.actor_user_id,
                    &course,
                )
                .await?
                {
                    continue;
                }
                let item = learner_course_catalog_item_queries::build_learner_course_catalog_item(
                    self.conn,
                    query.actor_user_id,
                    course,
                )
                .await?;
                if query
                    .reward_available
                    .is_some_and(|available| item.rewards.available != available)
                {
                    continue;
                }
                if query
                    .enrollment_status
                    .as_deref()
                    .is_some_and(|status| item.enrollment.state != status)
                {
                    continue;
                }
                if should_hide_generated_available_course(
                    &query,
                    &item.title,
                    &item.enrollment.state,
                ) {
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
            Ok(LearnerCourseCatalogOutput {
                courses,
                total,
                limit: query.limit,
                offset: query.offset,
                search: query.search,
                organization_id: query.organization_id,
                lifecycle_status: query.lifecycle_status,
                enrollment_status: query.enrollment_status,
                reward_available: query.reward_available,
            })
        }
        .boxed()
    }
}

fn should_hide_generated_available_course(
    query: &LearnerCourseCatalogQuery,
    title: &str,
    enrollment_state: &str,
) -> bool {
    query.search.is_none() && enrollment_state == "available" && is_generated_course_title(title)
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

fn map_catalog_error(error: diesel::result::Error) -> LearnerCourseCatalogError {
    match error {
        diesel::result::Error::NotFound => LearnerCourseCatalogError::NotFound,
        other => LearnerCourseCatalogError::Database(other.to_string()),
    }
}
