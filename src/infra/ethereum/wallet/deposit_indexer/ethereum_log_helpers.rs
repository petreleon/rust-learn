use ethers::types::{Address, H256};
use std::str::FromStr;

pub(super) fn address_from_topic(topic: H256) -> Option<Address> {
    Some(Address::from_slice(&topic.as_bytes()[12..]))
}

pub(super) fn event_signature(signature: &str) -> H256 {
    H256::from_slice(&ethers::utils::keccak256(signature.as_bytes()))
}

pub(super) fn parse_address(value: &str) -> Result<Address, String> {
    Address::from_str(value.trim())
        .map_err(|error| format!("invalid address '{}': {error}", value.trim()))
}
