use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_learner_course_detail::{
    LearnerCourseDetailOutput, LearnerCourseDetailQuery, LearnerCourseDetailStore,
};
use crate::application::learning::learner_course_catalog::LearnerCourseCatalogError;
use crate::infra::postgres::learning::{
    learner_course_access_queries, learner_course_catalog_chapter_queries,
    learner_course_catalog_item_queries,
};
use crate::infra::postgres::models::course::Course;
use crate::infra::postgres::schema::courses;

pub struct PostgresLearnerCourseDetailStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresLearnerCourseDetailStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl LearnerCourseDetailStore for PostgresLearnerCourseDetailStore<'_> {
    fn get_detail(
        &mut self,
        query: LearnerCourseDetailQuery,
    ) -> BoxFuture<'_, Result<LearnerCourseDetailOutput, LearnerCourseCatalogError>> {
        async move {
            let course = courses::table
                .find(query.course_id)
                .first::<Course>(self.conn)
                .await
                .map_err(map_catalog_error)?;

            if !learner_course_access_queries::course_visible_to_learner(
                self.conn,
                query.actor_user_id,
                &course,
            )
            .await?
            {
                return Err(LearnerCourseCatalogError::NotFound);
            }

            let course = learner_course_catalog_item_queries::build_learner_course_catalog_item(
                self.conn,
                query.actor_user_id,
                course,
            )
            .await?;
            let chapters =
                learner_course_catalog_chapter_queries::load_learner_course_catalog_chapters(
                    self.conn,
                    query.course_id,
                )
                .await?;
            let prerequisites = course.prerequisites.clone();

            Ok(LearnerCourseDetailOutput {
                course,
                chapters,
                prerequisites,
            })
        }
        .boxed()
    }
}

fn map_catalog_error(error: diesel::result::Error) -> LearnerCourseCatalogError {
    match error {
        diesel::result::Error::NotFound => LearnerCourseCatalogError::NotFound,
        other => LearnerCourseCatalogError::Database(other.to_string()),
    }
}
