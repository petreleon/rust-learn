async fn list_course_assessments(
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let course_id = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };
    let mut store = PostgresAssessmentReadStore::new(&mut conn);

    match list_published_course_assessments(&mut store, course_id).await {
        Ok(list) => HttpResponse::Ok().json(
            list.into_iter()
                .map(AssessmentResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(e) => {
            log::error!(
                "event=assessments_list_failed course_id={} error={}",
                course_id,
                assessment_read_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to list assessments")
        }
    }
}

#[derive(serde::Deserialize)]
struct SubmitAssessmentBody {
    answers: std::collections::HashMap<i32, String>,
}

async fn submit_assessment_attempt(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    body: web::Json<SubmitAssessmentBody>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    use crate::db::schema::{assessment_attempts, assessment_questions, assessments};
    use crate::models::assessment::{Assessment, AssessmentAttempt, AssessmentQuestion};
    use chrono::Utc;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    let user_id = match authenticated_user_id(&req) {
        Ok(id) => id,
        Err(response) => return response,
    };
    let (course_id, assessment_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("DB unavailable"),
    };

    let assessment = match assessments::table
        .filter(assessments::id.eq(assessment_id))
        .filter(assessments::course_id.eq(course_id))
        .filter(assessments::published.eq(true))
        .first::<Assessment>(&mut conn)
        .await
    {
        Ok(a) => a,
        Err(diesel::result::Error::NotFound) => {
            return HttpResponse::NotFound().body("Assessment not found")
        }
        Err(e) => {
            log::error!(
                "event=assessment_submit_lookup_failed assessment_id={} error={}",
                assessment_id,
                e
            );
            return HttpResponse::InternalServerError().body("Failed to load assessment");
        }
    };

    let previous_attempts: i64 = assessment_attempts::table
        .filter(assessment_attempts::assessment_id.eq(assessment_id))
        .filter(assessment_attempts::user_id.eq(user_id))
        .filter(assessment_attempts::completed_at.is_not_null())
        .count()
        .get_result(&mut conn)
        .await
        .unwrap_or(0);

    if previous_attempts >= assessment.max_attempts as i64 {
        return HttpResponse::Forbidden().body("Maximum attempts reached");
    }

    let questions = assessment_questions::table
        .filter(assessment_questions::assessment_id.eq(assessment_id))
        .order(assessment_questions::order.asc())
        .load::<AssessmentQuestion>(&mut conn)
        .await
        .unwrap_or_default();

    let mut score = 0;
    let total_points: i32 = questions.iter().map(|q| q.points).sum();

    for question in &questions {
        if let Some(correct) = &question.correct_answer {
            if body.answers.get(&question.id).map(|a| a.trim()) == Some(correct.trim()) {
                score += question.points;
            }
        }
    }

    let percentage = if total_points > 0 {
        (score as f64 / total_points as f64 * 100.0) as i32
    } else {
        100
    };
    let passed = percentage >= assessment.passing_score;
    let now = Utc::now();

    let attempt_result = diesel::insert_into(assessment_attempts::table)
        .values((
            assessment_attempts::assessment_id.eq(assessment_id),
            assessment_attempts::user_id.eq(user_id),
            assessment_attempts::score.eq(score),
            assessment_attempts::passed.eq(passed),
            assessment_attempts::completed_at.eq(now),
        ))
        .get_result::<AssessmentAttempt>(&mut conn)
        .await;

    let attempt = match attempt_result {
        Ok(a) => a,
        Err(e) => {
            log::error!("event=assessment_submit_failed error={}", e);
            return HttpResponse::InternalServerError().body("Failed to save attempt");
        }
    };

    HttpResponse::Ok().json(serde_json::json!({
        "attempt": attempt,
        "score": score,
        "total_points": total_points,
        "percentage": percentage,
        "passed": passed,
    }))
}

fn assessment_read_error_log(error: &AssessmentReadError) -> String {
    match error {
        AssessmentReadError::Database(message) => message.clone(),
    }
}
