use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

const ARTIFACTS_ROOT: &str = "./ethereum/artifacts";

pub fn write_abi_json(out_dir: &Path, contract_name: &str, abi_json: &str) -> Result<PathBuf> {
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

pub fn write_existing_abi_artifact(contract_name: &str, out_dir: &Path) -> Result<Option<PathBuf>> {
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
