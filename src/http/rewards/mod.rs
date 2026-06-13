pub mod dto;
mod handlers;
mod routes;

pub use routes::{
    configure_routes, reward_fraud_block_scope, reward_policy_scope,
    student_reward_history_resource,
};
