use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::assign_course_role::{
    CourseRoleAssignmentError, CourseRoleAssignmentStore,
};
use crate::infra::postgres::schema::{course_roles, role_course_hierarchy, user_role_course};

pub struct PostgresCourseRoleAssignmentStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseRoleAssignmentStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseRoleAssignmentStore for PostgresCourseRoleAssignmentStore<'_> {
    fn actor_min_level(
        &mut self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>> {
        async move {
            course_min_level(self.conn, actor_user_id, course_id)
                .await
                .map_err(map_assignment_error)
        }
        .boxed()
    }

    fn target_min_level(
        &mut self,
        target_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>> {
        async move {
            course_min_level(self.conn, target_user_id, course_id)
                .await
                .map_err(map_assignment_error)
        }
        .boxed()
    }

    fn role_id_by_name(
        &mut self,
        role_name: &str,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>> {
        let role_name = role_name.to_string();
        async move {
            course_roles::table
                .filter(course_roles::name.eq(role_name))
                .select(course_roles::id)
                .first::<i32>(self.conn)
                .await
                .optional()
                .map_err(map_assignment_error)
        }
        .boxed()
    }

    fn role_hierarchy_level(
        &mut self,
        role_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseRoleAssignmentError>> {
        async move {
            role_course_hierarchy::table
                .filter(role_course_hierarchy::course_role_id.eq(role_id))
                .select(role_course_hierarchy::hierarchy_level)
                .first::<i32>(self.conn)
                .await
                .optional()
                .map_err(map_assignment_error)
        }
        .boxed()
    }

    fn assign_role(
        &mut self,
        target_user_id: i32,
        course_id: i32,
        role_id: i32,
    ) -> BoxFuture<'_, Result<(), CourseRoleAssignmentError>> {
        async move {
            diesel::insert_into(user_role_course::table)
                .values((
                    user_role_course::user_id.eq(target_user_id),
                    user_role_course::course_id.eq(course_id),
                    user_role_course::course_role_id.eq(role_id),
                ))
                .execute(self.conn)
                .await
                .map(|_| ())
                .map_err(map_assignment_error)
        }
        .boxed()
    }
}

async fn course_min_level(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> diesel::QueryResult<Option<i32>> {
    role_course_hierarchy::table
        .inner_join(
            user_role_course::table
                .on(role_course_hierarchy::course_role_id.eq(user_role_course::course_role_id)),
        )
        .filter(user_role_course::user_id.eq(user_id))
        .filter(user_role_course::course_id.eq(course_id))
        .select(diesel::dsl::min(role_course_hierarchy::hierarchy_level))
        .first::<Option<i32>>(conn)
        .await
}

fn map_assignment_error(error: diesel::result::Error) -> CourseRoleAssignmentError {
    match error {
        diesel::result::Error::NotFound => CourseRoleAssignmentError::NotFound,
        other => CourseRoleAssignmentError::Database(other.to_string()),
    }
}
