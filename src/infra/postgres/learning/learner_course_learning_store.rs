use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::get_learner_course_learning::{
    LearnerCourseLearningError, LearnerCourseLearningOutput, LearnerCourseLearningQuery,
    LearnerCourseLearningStore,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::courses;
use crate::infra::postgres::learning::{
    learner_course_access_queries, learner_course_catalog_item_queries,
    learner_course_learning_content_queries,
};
use crate::models::course::Course;

pub struct PostgresLearnerCourseLearningStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresLearnerCourseLearningStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl LearnerCourseLearningStore for PostgresLearnerCourseLearningStore<'_> {
    fn get_learning(
        &mut self,
        query: LearnerCourseLearningQuery,
    ) -> BoxFuture<'_, Result<LearnerCourseLearningOutput, LearnerCourseLearningError>> {
        async move {
            let course = courses::table
                .find(query.course_id)
                .first::<Course>(self.conn)
                .await
                .map_err(map_learning_error)?;

            if !learner_course_access_queries::course_visible_to_learner(
                self.conn,
                query.actor_user_id,
                &course,
            )
            .await?
            {
                return Err(LearnerCourseLearningError::NotFound);
            }

            let course = learner_course_catalog_item_queries::build_learner_course_catalog_item(
                self.conn,
                query.actor_user_id,
                course,
            )
            .await?;
            if !course.access.can_view_content {
                return Err(LearnerCourseLearningError::PermissionDenied(
                    Permissions::VIEW_CONTENT.to_string(),
                ));
            }

            let chapters =
                learner_course_learning_content_queries::load_learner_course_learning_chapters(
                    self.conn,
                    query.course_id,
                )
                .await?;
            let active_content_id = chapters
                .iter()
                .flat_map(|chapter| {
                    chapter
                        .contents
                        .iter()
                        .map(move |content| (chapter.order, chapter.id, content.order, content.id))
                })
                .min_by_key(|(chapter_order, chapter_id, content_order, content_id)| {
                    (*chapter_order, *chapter_id, *content_order, *content_id)
                })
                .map(|(_, _, _, content_id)| content_id);
            let progress_supported = course.enrollment.state == "enrolled";

            Ok(LearnerCourseLearningOutput {
                course,
                chapters,
                active_content_id,
                progress_supported,
            })
        }
        .boxed()
    }
}

fn map_learning_error(error: diesel::result::Error) -> LearnerCourseLearningError {
    match error {
        diesel::result::Error::NotFound => LearnerCourseLearningError::NotFound,
        other => LearnerCourseLearningError::Database(other.to_string()),
    }
}
