// A small utility to compile a Solidity contract and write its ABI to a file.
// Usage:
//   cargo run --bin abi_export -- <path/to/Contract.sol> <ContractName> <out_dir>
// Example:
//   cargo run --bin abi_export -- ethereum/contracts/LearnToken.sol LearnToken ethereum/artifacts

use std::{env, fs, path::{Path, PathBuf}};

use ethers_solc::{Project, ProjectPathsConfig, remappings::Remapping};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("Usage: abi_export <path/to/Contract.sol> <ContractName> <out_dir>");
        std::process::exit(1);
    }

    let source_path = PathBuf::from(&args[0]);
    let contract_name = args[1].clone();
    let out_dir = PathBuf::from(&args[2]);

    if !source_path.exists() {
        eprintln!("Source file not found: {}", source_path.display());
        std::process::exit(1);
    }

    // Determine project root and sources dir
    let sources_root = PathBuf::from("./ethereum/contracts");

    // Configure remappings (OpenZeppelin) if present
    let mut remappings: Vec<Remapping> = Vec::new();
    if let Ok(oz_env) = env::var("OZ_PATH") {
        let remap: Remapping = format!("@openzeppelin/={}/", Path::new(&oz_env).display())
            .parse()
            .expect("invalid OZ_PATH remapping");
        remappings.push(remap);
    } else {
        let oz_path = sources_root.join("lib/openzeppelin-contracts");
        if oz_path.exists() {
            let remap: Remapping = format!("@openzeppelin/={}/", oz_path.display())
                .parse()
                .expect("invalid OZ remapping");
            remappings.push(remap);
        }
    }

    // Build project
    let paths = ProjectPathsConfig::builder()
        .root(&sources_root)
        .sources(&sources_root)
        .remappings(remappings)
        .build()
        .expect("failed to configure solc project paths");
    let project = Project::builder()
        .paths(paths)
        .build()
        .expect("failed to build solc project");

    // Compile
    let output = project.compile().expect("solc compile failed");

    // Find the compiled artifact for the provided file + contract name
    let file_name = source_path
        .file_name()
        .expect("invalid source path")
        .to_string_lossy()
        .to_string();

    let contract = output
        .find(&contract_name, &file_name)
        .unwrap_or_else(|| panic!("contract {contract_name} in {file_name} not found in output"));

    let abi = contract
        .abi
        .as_ref()
        .unwrap_or_else(|| panic!("no ABI found for {contract_name}"));

    // Write ABI JSON
    let abi_json = serde_json::to_string_pretty(abi).expect("failed to serialize ABI");
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
    }
    let out_file = out_dir.join(format!("{}.abi.json", contract_name));
    fs::write(&out_file, abi_json).expect("failed to write ABI file");

    println!("ABI written to {}", out_file.display());
}
