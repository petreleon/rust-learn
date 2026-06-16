pub type AssessmentQuestionOptions = serde_json::Value;

mod scoring;

pub use scoring::{score_answers, AssessmentQuestionAnswer, AssessmentScore};
