use crate::domain::rewards::audit::RewardAuditEventType;
use crate::domain::rewards::candidate::event_type::RewardEventType;
use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::domain::rewards::policy::RewardPaymentStrategy;

pub(super) fn parse_candidate_status<E>(
    value: &str,
    map_error: impl FnOnce(String) -> E,
) -> Result<RewardCandidateStatus, E> {
    RewardCandidateStatus::parse(value).map_err(|error| map_error(error.to_string()))
}

pub(super) fn parse_candidate_source_scope<E>(
    value: &str,
    map_error: impl FnOnce(String) -> E,
) -> Result<RewardCandidateSourceScope, E> {
    RewardCandidateSourceScope::parse(value).map_err(|error| map_error(error.to_string()))
}

pub(super) fn parse_reward_event_type<E>(
    value: &str,
    map_error: impl FnOnce(String) -> E,
) -> Result<RewardEventType, E> {
    RewardEventType::parse(value).map_err(|error| map_error(error.to_string()))
}

pub(super) fn parse_audit_event_type<E>(
    value: &str,
    map_error: impl FnOnce(String) -> E,
) -> Result<RewardAuditEventType, E> {
    RewardAuditEventType::parse(value).map_err(|error| map_error(error.to_string()))
}

pub(super) fn parse_payment_strategy<E>(
    value: &str,
    map_error: impl FnOnce(String) -> E,
) -> Result<RewardPaymentStrategy, E> {
    RewardPaymentStrategy::parse(value).map_err(|error| map_error(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::{
        parse_audit_event_type, parse_candidate_source_scope, parse_payment_strategy,
        parse_reward_event_type,
    };
    use crate::domain::rewards::audit::RewardAuditEventType;
    use crate::domain::rewards::candidate::event_type::RewardEventType;
    use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;
    use crate::domain::rewards::policy::RewardPaymentStrategy;

    #[test]
    fn parses_candidate_vocabulary_into_domain_types() {
        assert_eq!(
            parse_candidate_source_scope("course", String::from).unwrap(),
            RewardCandidateSourceScope::Course
        );
        assert_eq!(
            parse_reward_event_type("course_completion", String::from).unwrap(),
            RewardEventType::CourseCompletion
        );
    }

    #[test]
    fn parses_audit_and_payment_vocabulary_into_domain_types() {
        assert_eq!(
            parse_audit_event_type("teacher_decision", String::from).unwrap(),
            RewardAuditEventType::TeacherDecision
        );
        assert_eq!(
            parse_payment_strategy("mint", String::from).unwrap(),
            RewardPaymentStrategy::Mint
        );
    }

    #[test]
    fn rejects_unknown_persisted_reward_vocabulary() {
        assert_eq!(
            parse_candidate_source_scope("platform", String::from).unwrap_err(),
            "unknown reward candidate source scope 'platform'"
        );
        assert_eq!(
            parse_reward_event_type("lesson_started", String::from).unwrap_err(),
            "unknown reward event type 'lesson_started'"
        );
        assert_eq!(
            parse_audit_event_type("teacher_reviewed", String::from).unwrap_err(),
            "unknown reward audit event type 'teacher_reviewed'"
        );
        assert_eq!(
            parse_payment_strategy("direct", String::from).unwrap_err(),
            "unknown reward payment strategy 'direct'"
        );
    }
}
