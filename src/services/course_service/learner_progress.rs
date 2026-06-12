pub async fn save_learner_progress(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    content_id: i32,
) -> Result<CourseProgress, LearnerCourseCatalogError> {
    ensure_learner_progress_access(conn, user_id, course_id, Some(content_id)).await?;

    diesel::insert_into(course_progress::table)
        .values(NewCourseProgress {
            user_id,
            course_id,
            content_id,
        })
        .on_conflict((course_progress::user_id, course_progress::course_id))
        .do_update()
        .set((
            course_progress::content_id.eq(content_id),
            course_progress::viewed_at.eq(diesel::dsl::now),
        ))
        .get_result::<CourseProgress>(conn)
        .await
        .map_err(|e| {
            log::error!(
                "event=learner_progress_save_failed user_id={} course_id={} content_id={} error={}",
                user_id,
                course_id,
                content_id,
                e
            );
            LearnerCourseCatalogError::Database(e.to_string())
        })
}

pub async fn get_learner_progress(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<Option<CourseProgress>, LearnerCourseCatalogError> {
    ensure_learner_progress_access(conn, user_id, course_id, None).await?;

    let result = course_progress::table
        .filter(course_progress::user_id.eq(user_id))
        .filter(course_progress::course_id.eq(course_id))
        .first::<CourseProgress>(conn)
        .await;

    match result {
        Ok(progress) => Ok(Some(progress)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(e) => {
            log::error!(
                "event=learner_progress_fetch_failed user_id={} course_id={} error={}",
                user_id,
                course_id,
                e
            );
            Err(LearnerCourseCatalogError::Database(e.to_string()))
        }
    }
}

async fn ensure_learner_progress_access(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    content_id: Option<i32>,
) -> Result<(), LearnerCourseCatalogError> {
    let course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    if !course_visible_to_learner(conn, user_id, &course).await? {
        return Err(LearnerCourseCatalogError::NotFound);
    }

    if !learner_has_enrolled_course_state(conn, user_id, course_id).await? {
        return Err(LearnerCourseCatalogError::PermissionDenied(
            Permissions::VIEW_CONTENT.to_string(),
        ));
    }

    if let Some(content_id) = content_id {
        ensure_content_belongs_to_course(conn, course_id, content_id).await?;
    }

    Ok(())
}

async fn learner_has_enrolled_course_state(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<bool, LearnerCourseCatalogError> {
    let roles = load_actor_course_roles(conn, user_id, course_id).await?;
    if roles.iter().any(|role| role == "STUDENT") {
        return Ok(true);
    }

    course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::requester_user_id.eq(user_id))
        .filter(course_join_requests::status.eq(COURSE_JOIN_STATUS_APPROVED))
        .select(course_join_requests::id)
        .first::<i64>(conn)
        .await
        .optional()
        .map(|request_id| request_id.is_some())
        .map_err(LearnerCourseCatalogError::from)
}

async fn ensure_content_belongs_to_course(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    content_id: i32,
) -> Result<(), LearnerCourseCatalogError> {
    let content_course_id = contents::table
        .inner_join(chapters::table.on(contents::chapter_id.eq(chapters::id)))
        .filter(contents::id.eq(content_id))
        .select(chapters::course_id)
        .first::<i32>(conn)
        .await
        .optional()
        .map_err(LearnerCourseCatalogError::from)?;

    match content_course_id {
        Some(id) if id == course_id => Ok(()),
        _ => Err(LearnerCourseCatalogError::NotFound),
    }
}
