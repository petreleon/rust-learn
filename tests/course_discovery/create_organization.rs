use crate::support::*;

pub(crate) async fn create_organization(conn: &mut AsyncPgConnection, name: &str) -> Organization {
    diesel::insert_into(organizations::table)
        .values(NewOrganization {
            name: name.to_string(),
            website_link: None,
            profile_url: None,
        })
        .get_result(conn)
        .await
        .expect("failed to create organization")
}

pub(crate) async fn link_course_to_org(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    organization_id: i32,
    order: i32,
) {
    diesel::insert_into(courses_organizations::table)
        .values(NewCourseOrganization {
            course_id,
            organization_id,
            order,
        })
        .execute(conn)
        .await
        .expect("failed to link course and organization");
}

pub(crate) async fn create_chapter(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    title: &str,
    order: i32,
) -> i32 {
    diesel::insert_into(chapters::table)
        .values(NewChapter {
            course_id,
            title: title.to_string(),
            order,
        })
        .returning(chapters::id)
        .get_result(conn)
        .await
        .expect("failed to create chapter")
}

pub(crate) async fn create_content(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
    content_type: &str,
    order: i32,
) {
    create_content_with_data(conn, chapter_id, content_type, order, None).await;
}

pub(crate) async fn create_content_with_data(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
    content_type: &str,
    order: i32,
    data: Option<&str>,
) -> i32 {
    diesel::insert_into(contents::table)
        .values(NewContent {
            chapter_id,
            order,
            content_type: content_type.to_string(),
            data: data.map(ToString::to_string),
            publication_status: "published".to_string(),
        })
        .returning(contents::id)
        .get_result(conn)
        .await
        .expect("failed to create content")
}

pub(crate) async fn create_upload_job(
    conn: &mut AsyncPgConnection,
    object_key: &str,
    status: &str,
    last_error: Option<&str>,
) {
    let job_id = diesel::insert_into(upload_jobs::table)
        .values(NewUploadJob {
            bucket: "course-materials",
            object: object_key,
            user_id: None,
        })
        .returning(upload_jobs::id)
        .get_result::<i64>(conn)
        .await
        .expect("failed to create upload job");

    diesel::update(upload_jobs::table.find(job_id))
        .set((
            upload_jobs::status.eq(status),
            upload_jobs::last_error.eq(last_error.map(ToString::to_string)),
        ))
        .execute(conn)
        .await
        .expect("failed to update upload job");
}
