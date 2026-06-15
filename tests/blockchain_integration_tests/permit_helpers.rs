use crate::support::*;

fn pad_u256(value: U256) -> [u8; 32] {
    let mut b = [0u8; 32];
    value.to_big_endian(&mut b);
    b
}

fn pad_address(addr: Address) -> [u8; 32] {
    let mut b = [0u8; 32];
    let bytes = addr.as_bytes();
    b[12..32].copy_from_slice(bytes);
    b
}

pub(crate) fn eip2612_permit_digest(
    domain_separator: ethers::core::types::H256,
    owner: Address,
    spender: Address,
    amount: U256,
    nonce: U256,
    deadline: U256,
) -> ethers::core::types::H256 {
    let typehash = ethers::utils::keccak256(
        "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"
            .as_bytes(),
    );

    let mut enc = Vec::with_capacity(32 * 6);
    enc.extend_from_slice(&typehash);
    enc.extend_from_slice(&pad_address(owner));
    enc.extend_from_slice(&pad_address(spender));
    enc.extend_from_slice(&pad_u256(amount));
    enc.extend_from_slice(&pad_u256(nonce));
    enc.extend_from_slice(&pad_u256(deadline));
    let struct_hash = ethers::utils::keccak256(&enc);

    let mut digest_input = Vec::with_capacity(2 + 32 + 32);
    digest_input.push(0x19u8);
    digest_input.push(0x01u8);
    digest_input.extend_from_slice(domain_separator.as_bytes());
    digest_input.extend_from_slice(&struct_hash);
    ethers::core::types::H256::from_slice(&ethers::utils::keccak256(&digest_input))
}

pub(crate) fn permit_signature_parts(sig: ethers::types::Signature) -> (u8, [u8; 32], [u8; 32]) {
    (sig.v as u8, pad_u256(sig.r), pad_u256(sig.s))
}
