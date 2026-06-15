use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::learning::create_course::{
    CourseCreationError, CourseCreationOutput, CourseCreationStore,
};
use crate::db::schema::{courses, courses_organizations, pending_course_organization_invites};
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::models::course::{Course, NewCourse};
use crate::infra::postgres::models::courses_organizations::NewCourseOrganization;
use crate::infra::postgres::models::pending_course_organization_invites::NewPendingCourseOrganizationInvite;

pub struct PostgresCourseCreationStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseCreationStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseCreationStore for PostgresCourseCreationStore<'_> {
    fn create_course(
        &mut self,
        title: String,
        organization_ids: Vec<i32>,
    ) -> BoxFuture<'_, Result<CourseCreationOutput, CourseCreationError>> {
        async move {
            self.conn
                .transaction::<_, diesel::result::Error, _>(|conn| {
                    Box::pin(async move {
                        let course = insert_course(conn, title).await?;
                        link_course_organizations(conn, course.id, organization_ids).await?;
                        Ok(CourseCreationOutput::from(course))
                    })
                })
                .await
                .map_err(map_course_creation_error)
        }
        .boxed()
    }
}

impl AccessDecisionStore for PostgresCourseCreationStore<'_> {
    type Error = CourseCreationError;

    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, CourseCreationError>> {
        async move {
            permission_checks::can(self.conn, actor, action, scope)
                .await
                .map_err(map_course_creation_error)
        }
        .boxed()
    }
}

async fn insert_course(conn: &mut AsyncPgConnection, title: String) -> diesel::QueryResult<Course> {
    diesel::insert_into(courses::table)
        .values(NewCourse {
            title,
            description: None,
            topics: None,
            prerequisites: None,
        })
        .get_result::<Course>(conn)
        .await
}

async fn link_course_organizations(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_ids: Vec<i32>,
) -> diesel::QueryResult<()> {
    if let Some(first_org_id) = organization_ids.as_slice().first() {
        diesel::insert_into(courses_organizations::table)
            .values(NewCourseOrganization {
                course_id,
                organization_id: *first_org_id,
                order: 0,
            })
            .execute(conn)
            .await?;

        for org_id in organization_ids.iter().skip(1) {
            create_course_organization_invite(conn, course_id, *org_id).await?;
        }
    }

    Ok(())
}

async fn create_course_organization_invite(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
) -> diesel::QueryResult<usize> {
    let max_order_active = max_active_organization_order(conn, course_id).await?;
    let max_order_pending = max_pending_organization_order(conn, course_id).await?;
    let next_order = next_course_organization_order(max_order_active, max_order_pending);

    diesel::insert_into(pending_course_organization_invites::table)
        .values(NewPendingCourseOrganizationInvite {
            course_id,
            organization_id,
            order: next_order,
        })
        .execute(conn)
        .await
}

async fn max_active_organization_order(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> diesel::QueryResult<Option<i32>> {
    courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(diesel::dsl::max(courses_organizations::order))
        .first(conn)
        .await
        .optional()
        .map(Option::flatten)
}

async fn max_pending_organization_order(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> diesel::QueryResult<Option<i32>> {
    pending_course_organization_invites::table
        .filter(pending_course_organization_invites::course_id.eq(course_id))
        .select(diesel::dsl::max(pending_course_organization_invites::order))
        .first(conn)
        .await
        .optional()
        .map(Option::flatten)
}

fn next_course_organization_order(active: Option<i32>, pending: Option<i32>) -> i32 {
    match (active, pending) {
        (Some(active), Some(pending)) => std::cmp::max(active, pending) + 1,
        (Some(active), None) => active + 1,
        (None, Some(pending)) => pending + 1,
        (None, None) => 0,
    }
}

fn map_course_creation_error(error: diesel::result::Error) -> CourseCreationError {
    CourseCreationError::Database(error.to_string())
}
