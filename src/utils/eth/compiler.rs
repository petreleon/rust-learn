use ethers::abi::Abi;
use ethers::core::types::Bytes;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn load_contract_artifact(contract_name: &str) -> Option<(Abi, Bytes)> {
    let out_dir = Path::new("./ethereum/artifacts");
    let abi_path = out_dir.join(format!("{}.abi", contract_name));
    let bin_path = out_dir.join(format!("{}.bin", contract_name));

    if !abi_path.exists() || !bin_path.exists() {
        return None;
    }

    let abi_json = fs::read_to_string(&abi_path).ok()?;
    let abi: Abi = serde_json::from_str(&abi_json).ok()?;
    let bin_hex = fs::read_to_string(&bin_path).ok()?;
    let bin_bytes = hex::decode(bin_hex.trim()).ok()?;

    Some((abi, Bytes::from(bin_bytes)))
}

fn compile_from_source_requested() -> bool {
    env::var("ETH_CONTRACT_COMPILE_FROM_SOURCE")
        .map(|value| matches!(value.trim(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false)
}

/// Compile a specific contract file+name using ethers-solc. Returns (Abi, Bytecode).
pub fn try_compile_contract(
    contract_file: &str,
    contract_name: &str,
) -> Result<(Abi, Bytes), String> {
    use ethers_solc::{remappings::Remapping, Project, ProjectPathsConfig};
    use std::process::Command;

    if !compile_from_source_requested() {
        if let Some(artifact) = load_contract_artifact(contract_name) {
            log::info!(
                "event=eth_compile_artifact_loaded contract={}",
                contract_name
            );
            return Ok(artifact);
        }
    }

    // Configure remappings for OpenZeppelin
    let mut remappings: Vec<Remapping> = Vec::new();
    if let Ok(oz_env) = env::var("OZ_PATH") {
        let oz_env_path = Path::new(&oz_env).to_path_buf();
        let oz_env_str = match oz_env_path.canonicalize() {
            Ok(p) => p.display().to_string(),
            Err(_) => oz_env_path.display().to_string(),
        };
        remappings.push(
            Remapping::from_str(&format!("@openzeppelin/={}/", oz_env_str))
                .map_err(|error| format!("invalid OZ_PATH remapping: {error}"))?,
        );
    } else {
        let oz_path = Path::new("./ethereum/contracts").join("lib/openzeppelin-contracts");
        if oz_path.exists() {
            let oz_path_str = match oz_path.canonicalize() {
                Ok(p) => p.display().to_string(),
                Err(_) => oz_path.display().to_string(),
            };
            remappings.push(
                Remapping::from_str(&format!("@openzeppelin/={}/", oz_path_str))
                    .map_err(|error| format!("invalid OpenZeppelin remapping: {error}"))?,
            );
        }
    }

    let paths = ProjectPathsConfig::builder()
        .root("./ethereum/contracts")
        .sources("./ethereum/contracts")
        .artifacts("./ethereum/artifacts")
        .remappings(remappings)
        .build()
        .map_err(|error| format!("Failed to build project paths: {error}"))?;
    let project = Project::builder()
        .paths(paths)
        .build()
        .map_err(|error| format!("Failed to build project: {error}"))?;
    let output = match project.compile() {
        Ok(output) => output,
        Err(err) => {
            if let Some(artifact) = load_contract_artifact(contract_name) {
                log::warn!(
                    "event=eth_compile_artifact_fallback reason=compile_failed contract={} error={}",
                    contract_name,
                    err
                );
                return Ok(artifact);
            }

            return Err(format!("Failed to compile project: {err:?}"));
        }
    };

    if let Some(contract) = output.find(contract_name, contract_file) {
        let abi = contract
            .abi
            .as_ref()
            .ok_or_else(|| format!("ABI not found for {contract_name}"))?
            .clone()
            .into();
        let bytecode = contract
            .bytecode
            .as_ref()
            .ok_or_else(|| format!("Bytecode not found for {contract_name}"))?
            .object
            .clone()
            .into_bytes()
            .ok_or_else(|| format!("Could not get bytecode for {contract_name}"))?;
        return Ok((abi, bytecode));
    }

    if let Some(artifact) = load_contract_artifact(contract_name) {
        log::warn!(
            "event=eth_compile_artifact_fallback reason=missing_contract_in_ethers_solc contract={}",
            contract_name
        );
        return Ok(artifact);
    }

    // Fallback: use solc CLI
    log::warn!(
        "event=eth_compile_fallback reason=missing_contract_in_ethers_solc contract={}",
        contract_name
    );

    let mut solc_args: Vec<String> = Vec::new();
    if let Ok(oz_env) = env::var("OZ_PATH") {
        let oz_env_path = Path::new(&oz_env).to_path_buf();
        let oz_env_str = match oz_env_path.canonicalize() {
            Ok(p) => p.display().to_string(),
            Err(_) => oz_env_path.display().to_string(),
        };
        solc_args.push(format!("@openzeppelin={}", oz_env_str));
    } else {
        let oz_path = Path::new("./ethereum/contracts").join("lib/openzeppelin-contracts");
        if oz_path.exists() {
            let oz_path_str = match oz_path.canonicalize() {
                Ok(p) => p.display().to_string(),
                Err(_) => oz_path.display().to_string(),
            };
            solc_args.push(format!("@openzeppelin={}", oz_path_str));
        }
    }

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
    for arg in &solc_args {
        cmd.arg(arg);
    }
    cmd.arg(format!("./ethereum/contracts/{}", contract_file));

    let status = cmd
        .status()
        .map_err(|error| format!("failed to run solc CLI: {error}"))?;
    if !status.success() {
        return Err(format!(
            "solc CLI failed to compile {contract_name} from {contract_file}"
        ));
    }

    let abi_path = out_dir.join(format!("{}.abi", contract_name));
    let bin_path = out_dir.join(format!("{}.bin", contract_name));

    if !abi_path.exists() || !bin_path.exists() {
        return Err(format!(
            "solc CLI did not produce expected artifacts for {contract_name}"
        ));
    }

    let abi_json = fs::read_to_string(&abi_path)
        .map_err(|error| format!("failed to read abi file {}: {error}", abi_path.display()))?;
    let abi: Abi = serde_json::from_str(&abi_json)
        .map_err(|error| format!("failed to parse ABI JSON {}: {error}", abi_path.display()))?;

    let bin_hex = fs::read_to_string(&bin_path)
        .map_err(|error| format!("failed to read bin file {}: {error}", bin_path.display()))?;
    let bin_bytes = hex::decode(bin_hex.trim())
        .map_err(|error| format!("failed to decode bin hex {}: {error}", bin_path.display()))?;
    let bytecode = Bytes::from(bin_bytes);

    Ok((abi, bytecode))
}

/// Compile a specific contract file+name using ethers-solc. Returns (Abi, Bytecode).
pub fn compile_contract(contract_file: &str, contract_name: &str) -> (Abi, Bytes) {
    try_compile_contract(contract_file, contract_name).expect("Failed to compile contract")
}
