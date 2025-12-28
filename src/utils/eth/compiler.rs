use ethers::abi::Abi;
use ethers::core::types::Bytes;
use std::env;
use std::str::FromStr;
use std::path::Path;

/// Compile a specific contract file+name using ethers-solc. Returns (Abi, Bytecode).
pub fn compile_contract(contract_file: &str, contract_name: &str) -> (Abi, Bytes) {
    use ethers_solc::{Project, ProjectPathsConfig, remappings::Remapping};
    use std::process::Command;
    use std::fs;
    
    // Configure remappings for OpenZeppelin
    let mut remappings: Vec<Remapping> = Vec::new();
    if let Ok(oz_env) = env::var("OZ_PATH") {
        let oz_env_path = Path::new(&oz_env).to_path_buf();
        let oz_env_str = match oz_env_path.canonicalize() {
            Ok(p) => p.display().to_string(),
            Err(_) => oz_env_path.display().to_string(),
        };
        remappings.push(
            Remapping::from_str(&format!("@openzeppelin/={}/", oz_env_str)).unwrap(),
        );
    } else {
        let oz_path = Path::new("./ethereum/contracts").join("lib/openzeppelin-contracts");
        if oz_path.exists() {
            let oz_path_str = match oz_path.canonicalize() {
                Ok(p) => p.display().to_string(),
                Err(_) => oz_path.display().to_string(),
            };
            remappings.push(
                Remapping::from_str(&format!("@openzeppelin/={}/", oz_path_str)).unwrap(),
            );
        }
    }

    let paths = ProjectPathsConfig::builder()
        .root("./ethereum/contracts")
        .sources("./ethereum/contracts")
        .artifacts("./ethereum/artifacts")
        .remappings(remappings)
        .build()
        .expect("Failed to build project paths");
    let project = Project::builder().paths(paths).build().expect("Failed to build project");
    let output = project.compile().expect("Failed to compile project");

    if let Some(contract) = output.find(contract_name, contract_file) {
        let abi = contract.abi.as_ref().expect("ABI not found").clone().into();
        let bytecode = contract
            .bytecode
            .as_ref()
            .expect("Bytecode not found")
            .object
            .clone()
            .into_bytes()
            .expect("Could not get bytecode");
        return (abi, bytecode);
    }

    // Fallback: use solc CLI
    eprintln!("[eth_utils] ethers_solc output did not contain {}; falling back to solc CLI", contract_name);

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
        fs::create_dir_all(&out_dir).expect("failed to create artifacts dir");
    }

    let mut cmd = Command::new("solc");
    cmd.arg("--abi").arg("--bin").arg("--overwrite").arg("-o").arg(out_dir);
    for arg in &solc_args {
        cmd.arg(arg);
    }
    cmd.arg(format!("./ethereum/contracts/{}", contract_file));

    let status = cmd.status().expect("failed to run solc CLI");
    if !status.success() {
        panic!("solc CLI failed to compile the contract");
    }

    let abi_path = out_dir.join(format!("{}.abi", contract_name));
    let bin_path = out_dir.join(format!("{}.bin", contract_name));

    if !abi_path.exists() || !bin_path.exists() {
        panic!("solc CLI did not produce expected artifacts");
    }

    let abi_json = fs::read_to_string(&abi_path).expect("failed to read abi file");
    let abi: Abi = serde_json::from_str(&abi_json).expect("failed to parse ABI JSON");

    let bin_hex = fs::read_to_string(&bin_path).expect("failed to read bin file");
    let bin_bytes = hex::decode(bin_hex.trim()).expect("failed to decode bin hex");
    let bytecode = Bytes::from(bin_bytes);

    (abi, bytecode)
}
