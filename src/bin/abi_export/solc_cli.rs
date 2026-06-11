use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{anyhow, bail, Context, Result};

use super::artifacts::write_abi_json;
use super::remappings::solc_remappings;

pub fn export_via_solc(
    source_path: &Path,
    contract_name: &str,
    sources_root: &Path,
    out_dir: &Path,
) -> Result<PathBuf> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create output dir {}", out_dir.display()))?;

    let mut cmd = Command::new("solc");
    cmd.arg("--abi").arg("--overwrite").arg("-o").arg(out_dir);
    for arg in solc_remappings(sources_root) {
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

    let candidates = [
        out_dir.join(format!("{}.abi", contract_name)),
        out_dir.join(format!("{}.abi.json", contract_name)),
    ];
    let found_file = candidates
        .iter()
        .find(|candidate| candidate.exists())
        .ok_or_else(|| anyhow!("ABI for {contract_name} not produced by solc fallback"))?;

    let contents = fs::read_to_string(found_file)
        .with_context(|| format!("failed to read generated ABI {}", found_file.display()))?;
    write_abi_json(out_dir, contract_name, &contents)
}
