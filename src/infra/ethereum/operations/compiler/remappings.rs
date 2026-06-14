use ethers_solc::remappings::Remapping;
use std::env;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub(super) fn openzeppelin_remappings() -> Result<Vec<Remapping>, String> {
    openzeppelin_path()
        .map(|path| {
            let path = display_path(&path);
            Remapping::from_str(&format!("@openzeppelin/={path}/"))
                .map_err(|error| format!("invalid OpenZeppelin remapping: {error}"))
        })
        .transpose()
        .map(|remapping| remapping.into_iter().collect())
}

pub(super) fn openzeppelin_solc_args() -> Vec<String> {
    openzeppelin_path()
        .map(|path| vec![format!("@openzeppelin={}", display_path(&path))])
        .unwrap_or_default()
}

fn openzeppelin_path() -> Option<PathBuf> {
    env::var("OZ_PATH").ok().map(PathBuf::from).or_else(|| {
        let path = Path::new("./ethereum/contracts").join("lib/openzeppelin-contracts");
        path.exists().then_some(path)
    })
}

fn display_path(path: &Path) -> String {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .display()
        .to_string()
}
