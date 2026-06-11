#[test]
fn diesel_other_errors_map_to_database() {
    assert!(matches!(
        RewardExecutionError::from(diesel::result::Error::RollbackTransaction),
        RewardExecutionError::Database(_)
    ));
}
