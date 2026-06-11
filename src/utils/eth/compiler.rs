use ethers::abi::Abi;
use ethers::core::types::Bytes;
use std::env;

mod artifacts;
mod remappings;
mod solc;

use artifacts::load_contract_artifact;
use remappings::openzeppelin_remappings;
use solc::compile_with_solc_cli;

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
    use ethers_solc::{Project, ProjectPathsConfig};

    if !compile_from_source_requested() {
        if let Some(artifact) = load_contract_artifact(contract_name) {
            log::info!(
                "event=eth_compile_artifact_loaded contract={}",
                contract_name
            );
            return Ok(artifact);
        }
    }

    let remappings = openzeppelin_remappings()?;

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

    log::warn!(
        "event=eth_compile_fallback reason=missing_contract_in_ethers_solc contract={}",
        contract_name
    );

    compile_with_solc_cli(contract_file, contract_name)
}
