// A small utility to compile a Solidity contract and write its ABI to a file.
// Usage:
//   cargo run --bin abi_export --features tool-bin -- <path/to/Contract.sol> <ContractName> <out_dir>
// Example:
//   cargo run --bin abi_export --features tool-bin -- ethereum/contracts/LearnToken.sol LearnToken ethereum/artifacts

use std::{env, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use ethers_solc::{Project, ProjectPathsConfig};

#[path = "abi_export/artifacts.rs"]
mod artifacts;
#[path = "abi_export/remappings.rs"]
mod remappings;
#[path = "abi_export/solc_cli.rs"]
mod solc_cli;

use artifacts::{write_abi_json, write_existing_abi_artifact};
use remappings::ethers_solc_remappings;
use solc_cli::export_via_solc;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 3 {
        bail!("Usage: abi_export <path/to/Contract.sol> <ContractName> <out_dir>");
    }

    let source_path = PathBuf::from(&args[0]);
    let contract_name = args[1].clone();
    let out_dir = PathBuf::from(&args[2]);

    if !source_path.exists() {
        bail!("source file not found: {}", source_path.display());
    }

    let sources_root = PathBuf::from("./ethereum/contracts");

    let paths = ProjectPathsConfig::builder()
        .root(&sources_root)
        .sources(&sources_root)
        .remappings(ethers_solc_remappings(&sources_root)?)
        .build()
        .context("failed to configure solc project paths")?;
    let project = Project::builder()
        .paths(paths)
        .build()
        .context("failed to build solc project")?;

    let file_name = source_path
        .file_name()
        .ok_or_else(|| anyhow!("source path has no file name: {}", source_path.display()))?
        .to_string_lossy()
        .to_string();

    let output = match project.compile() {
        Ok(output) => output,
        Err(error) => {
            if let Some(out_file) = write_existing_abi_artifact(&contract_name, &out_dir)? {
                eprintln!(
                    "solc compile failed via ethers_solc; exported existing ABI artifact for {contract_name}: {error}"
                );
                println!("ABI written to {}", out_file.display());
                return Ok(());
            }

            return Err(anyhow!("solc compile failed: {error:?}"));
        }
    };

    if let Some(contract) = output.find(&contract_name, &file_name) {
        let abi = contract
            .abi
            .as_ref()
            .ok_or_else(|| anyhow!("no ABI found for {contract_name} in {file_name}"))?;

        let abi_json = serde_json::to_string(abi)
            .with_context(|| format!("failed to serialize ABI for {contract_name}"))?;
        let out_file = write_abi_json(&out_dir, &contract_name, &abi_json)?;

        println!("ABI written to {}", out_file.display());
        return Ok(());
    }

    if let Some(out_file) = write_existing_abi_artifact(&contract_name, &out_dir)? {
        eprintln!(
            "contract {contract_name} in {file_name} not found via ethers_solc; exported existing ABI artifact"
        );
        println!("ABI written to {}", out_file.display());
        return Ok(());
    }

    eprintln!("contract {contract_name} in {file_name} not found via ethers_solc; falling back to solc CLI");
    let out_file = export_via_solc(&source_path, &contract_name, &sources_root, &out_dir)?;
    println!("ABI written to {}", out_file.display());
    Ok(())
}
