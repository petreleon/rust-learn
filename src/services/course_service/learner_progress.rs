pub async fn save_learner_progress(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    content_id: i32,
) -> Result<CourseProgress, LearnerCourseCatalogError> {
    let _course = courses::table
        .find(course_id)
        .first::<Course>(conn)
        .await
        .map_err(LearnerCourseCatalogError::from)?;

    let _content = contents::table
        .find(content_id)
        .first::<crate::models::content::Content>(conn)
        .await
        .map_err(|_| LearnerCourseCatalogError::NotFound)?;

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
