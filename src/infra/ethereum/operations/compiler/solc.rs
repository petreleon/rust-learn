use super::remappings::openzeppelin_solc_args;
use ethers::abi::Abi;
use ethers::core::types::Bytes;
use std::fs;
use std::path::Path;
use std::process::Command;

pub(super) fn compile_with_solc_cli(
    contract_file: &str,
    contract_name: &str,
) -> Result<(Abi, Bytes), String> {
    let out_dir = Path::new("./ethereum/artifacts");
    if !out_dir.exists() {
        fs::create_dir_all(out_dir)
            .map_err(|error| format!("failed to create artifacts dir: {error}"))?;
    }

    let mut cmd = Command::new("solc");
    cmd.arg("--abi")
        .arg("--bin")
        .arg("--overwrite")
        .arg("-o")
        .arg(out_dir);
    for arg in openzeppelin_solc_args() {
        cmd.arg(arg);
    }
    cmd.arg(format!("./ethereum/contracts/{contract_file}"));

    let status = cmd
        .status()
        .map_err(|error| format!("failed to run solc CLI: {error}"))?;
    if !status.success() {
        return Err(format!(
            "solc CLI failed to compile {contract_name} from {contract_file}"
        ));
    }

    read_solc_artifacts(out_dir, contract_name)
}

fn read_solc_artifacts(out_dir: &Path, contract_name: &str) -> Result<(Abi, Bytes), String> {
    let abi_path = out_dir.join(format!("{contract_name}.abi"));
    let bin_path = out_dir.join(format!("{contract_name}.bin"));

    if !abi_path.exists() || !bin_path.exists() {
        return Err(format!(
            "solc CLI did not produce expected artifacts for {contract_name}"
        ));
    }

    let abi_json = fs::read_to_string(&abi_path)
        .map_err(|error| format!("failed to read abi file {}: {error}", abi_path.display()))?;
    let abi = serde_json::from_str(&abi_json)
        .map_err(|error| format!("failed to parse ABI JSON {}: {error}", abi_path.display()))?;

    let bin_hex = fs::read_to_string(&bin_path)
        .map_err(|error| format!("failed to read bin file {}: {error}", bin_path.display()))?;
    let bin_bytes = hex::decode(bin_hex.trim())
        .map_err(|error| format!("failed to decode bin hex {}: {error}", bin_path.display()))?;

    Ok((abi, Bytes::from(bin_bytes)))
}
