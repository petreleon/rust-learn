use crate::application::learning::assessment::{AssessmentOutput, AssessmentReadError};
use crate::application::learning::ports::AssessmentReadStore;
use std::collections::BTreeMap;

pub async fn list_published_course_assessments(
    store: &mut impl AssessmentReadStore,
    course_id: i32,
) -> Result<Vec<AssessmentOutput>, AssessmentReadError> {
    let mut assessments = store.list_published_for_course(course_id).await?;
    let assessment_ids = assessments.iter().map(|assessment| assessment.id).collect();
    let questions = store.list_questions_for_assessments(assessment_ids).await?;
    let mut questions_by_assessment: BTreeMap<i32, Vec<_>> = BTreeMap::new();

    for question in questions {
        questions_by_assessment
            .entry(question.assessment_id)
            .or_default()
            .push(question);
    }

    for assessment in &mut assessments {
        assessment.questions = questions_by_assessment
            .remove(&assessment.id)
            .unwrap_or_default();
    }

    Ok(assessments)
}
