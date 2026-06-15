#[path = "repository_reward_tests/support.rs"]
mod support;
#[path = "repository_reward_tests/test_candidate_create_and_find.rs"]
mod test_candidate_create_and_find;
#[path = "repository_reward_tests/test_candidate_filter_by_status.rs"]
mod test_candidate_filter_by_status;
#[path = "repository_reward_tests/test_execution_job_enqueue_is_idempotent.rs"]
mod test_execution_job_enqueue_is_idempotent;
#[path = "repository_reward_tests/test_fraud_block_create_and_find.rs"]
mod test_fraud_block_create_and_find;
#[path = "repository_reward_tests/test_fraud_block_revoke.rs"]
mod test_fraud_block_revoke;
#[path = "repository_reward_tests/test_policy_create_and_list.rs"]
mod test_policy_create_and_list;
#[path = "repository_reward_tests/test_policy_next_version.rs"]
mod test_policy_next_version;
