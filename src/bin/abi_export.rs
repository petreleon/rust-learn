// A small utility to compile a Solidity contract and write its ABI to a file.
// Usage:
//   cargo run --bin abi_export --features tool-bin -- <path/to/Contract.sol> <ContractName> <out_dir>
// Example:
//   cargo run --bin abi_export --features tool-bin -- ethereum/contracts/LearnToken.sol LearnToken ethereum/artifacts

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{anyhow, bail, Context, Result};
use ethers_solc::{remappings::Remapping, Project, ProjectPathsConfig};

const ARTIFACTS_ROOT: &str = "./ethereum/artifacts";

fn main() {
    if let Err(error) = run() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

fn write_abi_json(out_dir: &Path, contract_name: &str, abi_json: &str) -> Result<PathBuf> {
    let abi: serde_json::Value = serde_json::from_str(abi_json)
        .with_context(|| format!("failed to parse ABI JSON for {contract_name}"))?;
    let pretty_abi = serde_json::to_string_pretty(&abi)
        .with_context(|| format!("failed to serialize ABI for {contract_name}"))?;

    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create output dir {}", out_dir.display()))?;
    let out_file = out_dir.join(format!("{}.abi.json", contract_name));
    fs::write(&out_file, pretty_abi)
        .with_context(|| format!("failed to write ABI file {}", out_file.display()))?;

    Ok(out_file)
}

fn write_existing_abi_artifact(contract_name: &str, out_dir: &Path) -> Result<Option<PathBuf>> {
    let candidates = [
        Path::new(ARTIFACTS_ROOT).join(format!("{}.abi", contract_name)),
        Path::new(ARTIFACTS_ROOT).join(format!("{}.abi.json", contract_name)),
    ];

    let Some(artifact_file) = candidates.into_iter().find(|candidate| candidate.exists()) else {
        return Ok(None);
    };

    let abi_json = fs::read_to_string(&artifact_file)
        .with_context(|| format!("failed to read ABI artifact {}", artifact_file.display()))?;
    write_abi_json(out_dir, contract_name, &abi_json)
        .with_context(|| format!("failed to export ABI artifact {}", artifact_file.display()))
        .map(Some)
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

    // Determine project root and sources dir
    let sources_root = PathBuf::from("./ethereum/contracts");

    // Configure remappings (OpenZeppelin) if present
    let mut remappings: Vec<Remapping> = Vec::new();
    if let Ok(oz_env) = env::var("OZ_PATH") {
        let remap: Remapping = format!("@openzeppelin/={}/", Path::new(&oz_env).display())
            .parse()
            .context("invalid OZ_PATH remapping")?;
        remappings.push(remap);
    } else {
        let oz_path = sources_root.join("lib/openzeppelin-contracts");
        if oz_path.exists() {
            let remap: Remapping = format!("@openzeppelin/={}/", oz_path.display())
                .parse()
                .context("invalid OpenZeppelin remapping")?;
            remappings.push(remap);
        }
    }

    // Build project
    let paths = ProjectPathsConfig::builder()
        .root(&sources_root)
        .sources(&sources_root)
        .remappings(remappings)
        .build()
        .context("failed to configure solc project paths")?;
    let project = Project::builder()
        .paths(paths)
        .build()
        .context("failed to build solc project")?;

    // Find the compiled artifact for the provided file + contract name
    let file_name = source_path
        .file_name()
        .ok_or_else(|| anyhow!("source path has no file name: {}", source_path.display()))?
        .to_string_lossy()
        .to_string();

    // Compile via ethers_solc project first
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

        // Write ABI JSON
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

    // Fallback: use solc CLI (useful when ethers_solc path lookup differs)
    eprintln!("contract {contract_name} in {file_name} not found via ethers_solc; falling back to solc CLI");

    // Determine remapping for @openzeppelin
    let mut solc_args: Vec<String> = Vec::new();
    if let Ok(oz_env) = env::var("OZ_PATH") {
        solc_args.push(format!("@openzeppelin={}", oz_env));
    } else {
        let oz_path = sources_root.join("lib/openzeppelin-contracts");
        if oz_path.exists() {
            solc_args.push(format!("@openzeppelin={}", oz_path.display()));
        }
    }

    // Ensure out dir exists
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("failed to create output dir {}", out_dir.display()))?;

    // Call solc
    let mut cmd = Command::new("solc");
    cmd.arg("--abi").arg("--overwrite").arg("-o").arg(&out_dir);
    for arg in &solc_args {
        // remapping syntax is passed directly
        cmd.arg(arg);
    }
    cmd.arg(source_path.as_os_str());

    let status = cmd.status().context("failed to run solc CLI")?;
    if !status.success() {
        bail!(
            "solc CLI failed to compile {contract_name} from {}",
            source_path.display()
        );
    }

    // solc writes <ContractName>.abi (or other names). Try several candidates and write final .abi.json
    let candidates = [
        out_dir.join(format!("{}.abi", contract_name)),
        out_dir.join(format!("{}.abi.json", contract_name)),
    ];

    let found_file = candidates
        .iter()
        .find(|candidate| candidate.exists())
        .ok_or_else(|| anyhow!("ABI for {contract_name} not produced by solc fallback"))?;

    // Read and (re)write as .abi.json
    let contents = fs::read_to_string(found_file)
        .with_context(|| format!("failed to read generated ABI {}", found_file.display()))?;
    let out_file = write_abi_json(&out_dir, &contract_name, &contents)?;
    println!("ABI written to {}", out_file.display());
    Ok(())
}
