use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::learning::manage_assessments::{
    AssessmentAuthoringError, AssessmentAuthoringStore, AssessmentDraft, AuthoredAssessmentOutput,
};
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::learning::assessment_authoring_queries::{
    load_assessment_with_questions, load_assessments_with_questions, map_authoring_error,
};
use crate::infra::postgres::learning::assessment_authoring_writes::{
    insert_assessment, replace_questions, update_assessment_row,
};
use crate::infra::postgres::models::assessment::Assessment;
use crate::infra::postgres::schema::assessments;

pub struct PostgresAssessmentAuthoringStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresAssessmentAuthoringStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl AssessmentAuthoringStore for PostgresAssessmentAuthoringStore<'_> {
    fn list_for_course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AuthoredAssessmentOutput>, AssessmentAuthoringError>> {
        async move {
            let assessments = assessments::table
                .filter(assessments::course_id.eq(course_id))
                .order(assessments::created_at.asc())
                .load::<Assessment>(self.conn)
                .await
                .map_err(map_authoring_error)?;

            load_assessments_with_questions(self.conn, assessments).await
        }
        .boxed()
    }

    fn create_assessment(
        &mut self,
        course_id: i32,
        draft: AssessmentDraft,
    ) -> BoxFuture<'_, Result<AuthoredAssessmentOutput, AssessmentAuthoringError>> {
        async move {
            self.conn
                .transaction::<_, diesel::result::Error, _>(|conn| {
                    Box::pin(async move {
                        let assessment = insert_assessment(conn, course_id, draft).await?;
                        Ok(load_assessment_with_questions(conn, assessment).await?)
                    })
                })
                .await
                .map_err(map_authoring_error)
        }
        .boxed()
    }

    fn update_assessment(
        &mut self,
        course_id: i32,
        assessment_id: i32,
        draft: AssessmentDraft,
    ) -> BoxFuture<'_, Result<AuthoredAssessmentOutput, AssessmentAuthoringError>> {
        async move {
            self.conn
                .transaction::<_, diesel::result::Error, _>(|conn| {
                    Box::pin(async move {
                        let assessment =
                            update_assessment_row(conn, course_id, assessment_id, &draft).await?;
                        replace_questions(conn, assessment_id, draft.questions).await?;
                        Ok(load_assessment_with_questions(conn, assessment).await?)
                    })
                })
                .await
                .map_err(map_authoring_error)
        }
        .boxed()
    }
}

impl AccessDecisionStore for PostgresAssessmentAuthoringStore<'_> {
    type Error = AssessmentAuthoringError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, AssessmentAuthoringError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(map_authoring_error)
        }
        .boxed()
    }
}
