use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{course_roles, role_permission_course, user_role_course};

pub async fn course_user_has_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    permission: &str,
) -> QueryResult<bool> {
    select(exists(
        user_role_course::table
            .inner_join(
                course_roles::table
                    .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
            )
            .inner_join(
                role_permission_course::table.on(course_roles::id
                    .nullable()
                    .eq(role_permission_course::course_role_id)),
            )
            .filter(user_role_course::user_id.eq(user_id))
            .filter(user_role_course::course_id.eq(course_id))
            .filter(role_permission_course::permission.eq(permission)),
    ))
    .get_result(conn)
    .await
}

pub async fn assign_course_role_to_user(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    course_role_id: i32,
) -> QueryResult<usize> {
    let new_user_role = (
        user_role_course::user_id.eq(user_id),
        user_role_course::course_role_id.eq(course_role_id),
        user_role_course::course_id.eq(course_id),
    );

    diesel::insert_into(user_role_course::table)
        .values(&new_user_role)
        .execute(conn)
        .await
}
