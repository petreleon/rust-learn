use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::learner_progress::{
    LearnerProgressError, LearnerProgressOutput, LearnerProgressStore, ProgressCourse,
};
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::learning::learner_progress_queries;

pub struct PostgresLearnerProgressStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresLearnerProgressStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl LearnerProgressStore for PostgresLearnerProgressStore<'_> {
    fn course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<ProgressCourse, LearnerProgressError>> {
        async move {
            learner_progress_queries::load_course(self.conn, course_id)
                .await
                .map_err(map_progress_error)
        }
        .boxed()
    }

    fn course_organization_ids(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<i32>, LearnerProgressError>> {
        async move {
            learner_progress_queries::course_organization_ids(self.conn, course_id)
                .await
                .map_err(map_progress_error)
        }
        .boxed()
    }

    fn has_course_permission(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, LearnerProgressError>> {
        let permission = permission.to_string();
        async move {
            permission_checks::can_course_permission(
                self.conn,
                actor_user_id,
                course_id,
                &permission,
            )
            .await
            .map_err(map_progress_error)
        }
        .boxed()
    }

    fn has_organization_permission(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, LearnerProgressError>> {
        let permission = permission.to_string();
        async move {
            permission_checks::can_organization_permission(
                self.conn,
                actor_user_id,
                organization_id,
                &permission,
            )
            .await
            .map_err(map_progress_error)
        }
        .boxed()
    }

    fn actor_course_roles(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<String>, LearnerProgressError>> {
        async move {
            learner_progress_queries::actor_course_roles(self.conn, actor_user_id, course_id)
                .await
                .map_err(map_progress_error)
        }
        .boxed()
    }

    fn has_join_request_status(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        status: &str,
    ) -> BoxFuture<'_, Result<bool, LearnerProgressError>> {
        let status = status.to_string();
        async move {
            learner_progress_queries::has_join_request_status(
                self.conn,
                actor_user_id,
                course_id,
                status,
            )
            .await
            .map_err(map_progress_error)
        }
        .boxed()
    }

    fn content_belongs_to_course(
        &mut self,
        course_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<bool, LearnerProgressError>> {
        async move {
            learner_progress_queries::content_belongs_to_course(self.conn, course_id, content_id)
                .await
                .map_err(map_progress_error)
        }
        .boxed()
    }

    fn save_progress(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
        content_id: i32,
    ) -> BoxFuture<'_, Result<LearnerProgressOutput, LearnerProgressError>> {
        async move {
            learner_progress_queries::save_progress(self.conn, actor_user_id, course_id, content_id)
                .await
                .map(LearnerProgressOutput::from)
                .map_err(map_progress_error)
        }
        .boxed()
    }

    fn get_progress(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<LearnerProgressOutput>, LearnerProgressError>> {
        async move {
            learner_progress_queries::get_progress(self.conn, actor_user_id, course_id)
                .await
                .map(|progress| progress.map(LearnerProgressOutput::from))
                .map_err(map_progress_error)
        }
        .boxed()
    }
}

fn map_progress_error(error: diesel::result::Error) -> LearnerProgressError {
    match error {
        diesel::result::Error::NotFound => LearnerProgressError::NotFound,
        other => LearnerProgressError::Database(other.to_string()),
    }
}
