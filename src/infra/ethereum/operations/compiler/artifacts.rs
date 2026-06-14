use ethers::abi::Abi;
use ethers::core::types::Bytes;
use std::fs;
use std::path::Path;

pub(super) fn load_contract_artifact(contract_name: &str) -> Option<(Abi, Bytes)> {
    let out_dir = Path::new("./ethereum/artifacts");
    let abi_path = out_dir.join(format!("{}.abi", contract_name));
    let bin_path = out_dir.join(format!("{}.bin", contract_name));

    if !abi_path.exists() || !bin_path.exists() {
        return None;
    }

    let abi_json = fs::read_to_string(&abi_path).ok()?;
    let abi = serde_json::from_str(&abi_json).ok()?;
    let bin_hex = fs::read_to_string(&bin_path).ok()?;
    let bin_bytes = hex::decode(bin_hex.trim()).ok()?;

    Some((abi, Bytes::from(bin_bytes)))
}
