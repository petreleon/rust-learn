async fn load_teacher_student_lesson_progress(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    total_content_count: usize,
) -> Result<TeacherStudentProgressSummary, TeacherCourseDashboardError> {
    let row = course_progress::table
        .inner_join(contents::table.on(course_progress::content_id.eq(contents::id)))
        .inner_join(chapters::table.on(contents::chapter_id.eq(chapters::id)))
        .filter(course_progress::course_id.eq(course_id))
        .filter(course_progress::user_id.eq(student_user_id))
        .select((
            course_progress::content_id,
            contents::content_type,
            chapters::order,
            contents::order,
            course_progress::viewed_at,
        ))
        .first::<(i32, String, i32, i32, DateTime<Utc>)>(conn)
        .await
        .optional()
        .map_err(TeacherCourseDashboardError::from)?;

    let Some((content_id, content_type, chapter_order, content_order, viewed_at)) = row else {
        return Ok(TeacherStudentProgressSummary {
            supported: true,
            completed_content_count: Some(0),
            total_content_count,
            completion_percentage: Some(0.0),
            current_content_id: None,
            current_content_label: None,
            last_activity_at: None,
            note: "No lesson progress has been saved yet.".to_string(),
        });
    };

    let completed = if total_content_count == 0 { 0_i64 } else { 1_i64 };
    let percentage = if total_content_count == 0 {
        0.0
    } else {
        (completed as f64 / total_content_count as f64 * 100.0).round()
    };
    let current_content_label = format!(
        "Module {}, lesson {}: {}",
        chapter_order + 1,
        content_order + 1,
        content_type
    );

    Ok(TeacherStudentProgressSummary {
        supported: true,
        completed_content_count: Some(completed),
        total_content_count,
        completion_percentage: Some(percentage),
        current_content_id: Some(content_id),
        current_content_label: Some(current_content_label),
        last_activity_at: Some(viewed_at),
        note: "Latest viewed lesson is saved from the learner course route.".to_string(),
    })
}
