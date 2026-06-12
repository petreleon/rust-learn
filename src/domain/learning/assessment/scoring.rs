use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentQuestionAnswer {
    pub id: i32,
    pub correct_answer: Option<String>,
    pub points: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AssessmentScore {
    pub score: i32,
    pub total_points: i32,
    pub percentage: i32,
}

pub fn score_answers(
    questions: &[AssessmentQuestionAnswer],
    answers: &HashMap<i32, String>,
) -> AssessmentScore {
    let total_points = questions.iter().map(|question| question.points).sum();
    let score = questions
        .iter()
        .filter(|question| {
            question.correct_answer.as_ref().is_some_and(|correct| {
                answers.get(&question.id).map(|answer| answer.trim()) == Some(correct.trim())
            })
        })
        .map(|question| question.points)
        .sum();

    let percentage = if total_points > 0 {
        (score as f64 / total_points as f64 * 100.0) as i32
    } else {
        100
    };

    AssessmentScore {
        score,
        total_points,
        percentage,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_answers_trims_input_and_counts_matching_questions() {
        let questions = vec![
            AssessmentQuestionAnswer {
                id: 1,
                correct_answer: Some("yes".to_string()),
                points: 2,
            },
            AssessmentQuestionAnswer {
                id: 2,
                correct_answer: Some("no".to_string()),
                points: 3,
            },
            AssessmentQuestionAnswer {
                id: 3,
                correct_answer: None,
                points: 5,
            },
        ];
        let answers = HashMap::from([(1, " yes ".to_string()), (2, "maybe".to_string())]);

        let score = score_answers(&questions, &answers);

        assert_eq!(
            score,
            AssessmentScore {
                score: 2,
                total_points: 10,
                percentage: 20
            }
        );
    }

    #[test]
    fn score_answers_returns_full_percentage_when_no_points_exist() {
        let score = score_answers(&[], &HashMap::new());

        assert_eq!(
            score,
            AssessmentScore {
                score: 0,
                total_points: 0,
                percentage: 100
            }
        );
    }
}
