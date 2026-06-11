use std::{env, path::Path};

use anyhow::{Context, Result};
use ethers_solc::remappings::Remapping;

pub fn ethers_solc_remappings(sources_root: &Path) -> Result<Vec<Remapping>> {
    let mut remappings = Vec::new();
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

    Ok(remappings)
}

pub fn solc_remappings(sources_root: &Path) -> Vec<String> {
    if let Ok(oz_env) = env::var("OZ_PATH") {
        return vec![format!("@openzeppelin={oz_env}")];
    }

    let oz_path = sources_root.join("lib/openzeppelin-contracts");
    if oz_path.exists() {
        return vec![format!("@openzeppelin={}", oz_path.display())];
    }

    Vec::new()
}
