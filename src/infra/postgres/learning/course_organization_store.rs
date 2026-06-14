use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::list_course_organizations::{
    CourseOrganizationOutput, CourseOrganizationReadError, CourseOrganizationStore,
};
use crate::db::schema::{courses_organizations, organizations};
use crate::models::organization::Organization;

pub struct PostgresCourseOrganizationStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseOrganizationStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseOrganizationStore for PostgresCourseOrganizationStore<'_> {
    fn list_for_course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<CourseOrganizationOutput>, CourseOrganizationReadError>> {
        async move {
            courses_organizations::table
                .filter(courses_organizations::course_id.eq(course_id))
                .inner_join(organizations::table)
                .select(organizations::all_columns)
                .load::<Organization>(self.conn)
                .await
                .map(|organizations| {
                    organizations
                        .into_iter()
                        .map(course_organization_output_from_model)
                        .collect()
                })
                .map_err(|error| CourseOrganizationReadError::Database(error.to_string()))
        }
        .boxed()
    }
}

fn course_organization_output_from_model(organization: Organization) -> CourseOrganizationOutput {
    CourseOrganizationOutput {
        id: organization.id,
        name: organization.name,
        website_link: organization.website_link,
        profile_url: organization.profile_url,
    }
}
