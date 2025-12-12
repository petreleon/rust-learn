use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, AsyncConnection};
use crate::models::course::{Course, NewCourse};
use crate::models::courses_organizations::NewCourseOrganization;
use crate::models::pending_course_organization_invites::NewPendingCourseOrganizationInvite;
use crate::db::schema::{courses, courses_organizations, pending_course_organization_invites};

pub async fn create_course_with_invites(
    conn: &mut AsyncPgConnection,
    title: String,
    organization_ids: Vec<i32>,
) -> QueryResult<Course> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| Box::pin(async move {
        let new_course = NewCourse {
            title: title,
        };

        let course = diesel::insert_into(courses::table)
            .values(&new_course)
            .get_result::<Course>(conn)
            .await?;

        if let Some(first_org_id) = organization_ids.as_slice().first() {
            // Add first organization directly
            let new_link = NewCourseOrganization {
                course_id: course.id,
                organization_id: *first_org_id,
                order: 0,
            };
            diesel::insert_into(courses_organizations::table)
                .values(&new_link)
                .execute(conn)
                .await?;

            // Add remaining organizations as pending invites
            for org_id in organization_ids.iter().skip(1) {
                create_course_organization_invite(conn, course.id, *org_id).await?;
            }
        }

        Ok(course)
    })).await
}

pub async fn create_course_organization_invite(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
) -> QueryResult<usize> {
    use diesel::dsl::max;

    let max_order_active: Option<i32> = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(max(courses_organizations::order))
        .first(conn)
        .await
        .optional()?
        .flatten();

    let max_order_pending: Option<i32> = pending_course_organization_invites::table
        .filter(pending_course_organization_invites::course_id.eq(course_id))
        .select(max(pending_course_organization_invites::order))
        .first(conn)
        .await
        .optional()?
        .flatten();

    let next_order = match (max_order_active, max_order_pending) {
        (Some(a), Some(b)) => std::cmp::max(a, b) + 1,
        (Some(a), None) => a + 1,
        (None, Some(b)) => b + 1,
        (None, None) => 0,
    };

    let new_invite = NewPendingCourseOrganizationInvite {
        course_id,
        organization_id,
        order: next_order,
    };
    diesel::insert_into(pending_course_organization_invites::table)
        .values(&new_invite)
        .execute(conn)
        .await
}

pub async fn accept_course_organization_invite(
    conn: &mut AsyncPgConnection,
    invite_id: i32,
) -> QueryResult<usize> {
    use crate::models::pending_course_organization_invites::PendingCourseOrganizationInvite;
    
    conn.transaction::<_, diesel::result::Error, _>(|conn| Box::pin(async move {
        let invite = pending_course_organization_invites::table
            .find(invite_id)
            .first::<PendingCourseOrganizationInvite>(conn)
            .await?;

        let new_link = NewCourseOrganization {
            course_id: invite.course_id,
            organization_id: invite.organization_id,
            order: invite.order,
        };

        diesel::insert_into(courses_organizations::table)
            .values(&new_link)
            .execute(conn)
            .await?;

        diesel::delete(pending_course_organization_invites::table.find(invite_id))
            .execute(conn)
            .await
    })).await
}
