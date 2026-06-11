#[test]
fn returns_both_when_approver_matches_submitter() {
    let candidate = test_candidate(5, Some(5));
    assert_eq!(candidate_teacher_user_ids(&candidate), vec![5]);
}

#[test]
fn returns_both_when_approver_differs() {
    let candidate = test_candidate(5, Some(10));
    assert_eq!(candidate_teacher_user_ids(&candidate), vec![5, 10]);
}

#[test]
fn deduplicates_teacher_ids() {
    let candidate = test_candidate(5, Some(5));
    let ids = candidate_teacher_user_ids(&candidate);
    assert_eq!(ids.len(), 1);
    assert_eq!(ids[0], 5);
}

// ── RewardCandidateError from diesel::Error ──

#[test]
fn diesel_not_found_maps_to_reward_not_found() {
    assert_eq!(
        RewardCandidateError::from(diesel::result::Error::NotFound),
        RewardCandidateError::NotFound
    );
}

#[test]
fn diesel_other_errors_map_to_database() {
    assert!(matches!(
        RewardCandidateError::from(diesel::result::Error::RollbackTransaction),
        RewardCandidateError::Database(_)
    ));
}
