// Utility functions for Ethereum contract interaction
// src/utils/eth_utils.rs

use crate::utils::db_utils::persistent_state::{get_persistent_state, set_persistent_state};
use diesel::prelude::*;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use ethers::prelude::*;
use ethers::signers::coins_bip39::English;
use ethers::abi::Abi;
use ethers::core::types::Bytes;
use std::env;
use std::str::FromStr;
use std::path::Path;
use ethers::signers::MnemonicBuilder;

/// Loads wallet from ETH_MNEMONIC in .env
pub fn load_wallet_from_env() -> Wallet<k256::ecdsa::SigningKey> {
    dotenvy::dotenv().ok();
    let mnemonic = env::var("ETH_MNEMONIC").expect("ETH_MNEMONIC not set");
    MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .build()
        .expect("Failed to create wallet from mnemonic")
}

/// Connects to an Ethereum node using env configuration
pub fn get_provider() -> Provider<Http> {
    dotenvy::dotenv().ok();
    let url = std::env::var("ETH_RPC_URL").ok().unwrap_or_else(|| {
        let host = std::env::var("ETH_HOST").unwrap_or_else(|_| "geth".to_string());
        let port = std::env::var("ETH_PORT").unwrap_or_else(|_| "8545".to_string());
        format!("http://{}:{}", host, port)
    });
    Provider::<Http>::try_from(url).expect("Could not create provider from ETH_RPC_URL/ETH_HOST/ETH_PORT")
}

/// Compile a specific contract file+name using ethers-solc. Returns (Abi, Bytecode).
pub fn compile_contract(contract_file: &str, contract_name: &str) -> (Abi, Bytes) {
    use ethers_solc::{Project, ProjectPathsConfig, remappings::Remapping};
    use std::process::Command;
    use std::fs;
    // Configure remappings for OpenZeppelin. Prefer OZ_PATH env var if set,
    // otherwise fall back to the local `lib/openzeppelin-contracts` path under sources.
    let mut remappings: Vec<Remapping> = Vec::new();
    if let Ok(oz_env) = env::var("OZ_PATH") {
        // match abi_export.rs style: @openzeppelin/={path}/
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

    // Try to find the contract via ethers_solc output first
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

    // Fallback: use solc CLI to produce ABI and BIN artifacts into ./ethereum/artifacts
    // (matches the fallback used by abi_export.rs)
    eprintln!("[eth_utils] ethers_solc output did not contain LearnToken; falling back to solc CLI");

    // Build solc args for remappings
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
    // produce both abi and bin
    cmd.arg("--abi").arg("--bin").arg("--overwrite").arg("-o").arg(out_dir);
    for arg in &solc_args {
        cmd.arg(arg);
    }
    cmd.arg(format!("./ethereum/contracts/{}", contract_file));

    let status = cmd.status().expect("failed to run solc CLI");
    if !status.success() {
        panic!("solc CLI failed to compile the contract");
    }

    // solc writes <ContractName>.abi and <ContractName>.bin in out_dir
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

/// Deploys the LearnToken contract
pub async fn deploy_contract(
    wallet: Wallet<k256::ecdsa::SigningKey>,
    provider: Provider<Http>,
    abi: Abi,
    bytecode: Bytes,
    // constructor args will be passed by caller via the factory.deploy(...) call
    deploy_args: impl ethers::core::abi::Tokenize,
) -> Address {
    let client = std::sync::Arc::new(SignerMiddleware::new(provider, wallet));
    let factory = ContractFactory::new(abi, bytecode, client);
    let deployer = factory.deploy(deploy_args).unwrap();
    let contract = deployer.send().await.unwrap();
    contract.address()
}

/// Deploy LearnToken and store its address in persistent_states under `learn_token_address`
pub async fn deploy_learn_token_and_save(
    conn: &mut AsyncPgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<Address> {
    // Load wallet and provider
    let wallet = load_wallet_from_env();
    let provider = get_provider();

    // Compile and deploy
    let (abi, bytecode) = compile_contract("LearnToken.sol", "LearnToken");
    let addr = deploy_contract(
        wallet,
        provider,
        abi,
        bytecode,
        (name.to_string(), symbol.to_string(), decimals),
    )
    .await;

    // Persist address as 0x-prefixed hex
    let addr_hex = format!("{:#x}", addr);
    set_persistent_state(conn, "learn_token_address", &addr_hex).await?;

    Ok(addr)
}

/// Deploy LearnTokenPresigner and save its address under `learn_token_presigner_address`.
pub async fn deploy_learn_token_presigner_and_save(
    conn: &mut AsyncPgConnection,
    learn_token_addr: Address,
) -> QueryResult<Address> {
    let wallet = load_wallet_from_env();
    let provider = get_provider();

    let (abi, bytecode) = compile_contract("LearnTokenPresigner.sol", "LearnTokenPresigner");
    let addr = deploy_contract(wallet, provider, abi, bytecode, (learn_token_addr,)).await;

    let addr_hex = format!("{:#x}", addr);
    set_persistent_state(conn, "learn_token_presigner_address", &addr_hex).await?;
    Ok(addr)
}

/// Deploy PlatformImporter and save its address under `platform_importer_address`.
pub async fn deploy_platform_importer_and_save(
    conn: &mut AsyncPgConnection,
    treasury: Address,
) -> QueryResult<Address> {
    let wallet = load_wallet_from_env();
    let provider = get_provider();

    let (abi, bytecode) = compile_contract("PlatformImporter.sol", "PlatformImporter");
    let addr = deploy_contract(wallet, provider, abi, bytecode, (treasury,)).await;

    let addr_hex = format!("{:#x}", addr);
    set_persistent_state(conn, "platform_importer_address", &addr_hex).await?;
    Ok(addr)
}

/// Ensure all required contracts are deployed and persisted. Returns tuple of addresses.
pub async fn deploy_all_startup(
    conn: &mut AsyncPgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<(Address, Option<Address>, Option<Address>)> {
    // Deploy LearnToken if missing
    let token_addr = if let Some(a) = get_persistent_state(conn, "learn_token_address").await? {
        Address::from_str(a.trim_start_matches("0x")).map_err(|_| diesel::result::Error::NotFound)?
    } else {
        deploy_learn_token_and_save(conn, name, symbol, decimals).await?;
        let s = get_persistent_state(conn, "learn_token_address").await?.unwrap();
        Address::from_str(s.trim_start_matches("0x")).map_err(|_| diesel::result::Error::NotFound)?
    };

    // Deploy LearnTokenPresigner if missing
    let presigner_addr = if let Some(a) = get_persistent_state(conn, "learn_token_presigner_address").await? {
        Some(Address::from_str(a.trim_start_matches("0x")).map_err(|_| diesel::result::Error::NotFound)?)
    } else {
        // deploy using token_addr
        let addr = deploy_learn_token_presigner_and_save(conn, token_addr).await?;
        Some(addr)
    };

    // Deploy PlatformImporter if missing (use PLATFORM_TREASURY env or deployer address)
    let importer_addr = if let Some(a) = get_persistent_state(conn, "platform_importer_address").await? {
        Some(Address::from_str(a.trim_start_matches("0x")).map_err(|_| diesel::result::Error::NotFound)?)
    } else {
        let treasury_addr = if let Ok(t) = env::var("PLATFORM_TREASURY") {
            Address::from_str(t.trim_start_matches("0x")).map_err(|_| diesel::result::Error::NotFound)?
        } else {
            // fallback to account derived from mnemonic
            let wallet = load_wallet_from_env();
            wallet.address()
        };
        let addr = deploy_platform_importer_and_save(conn, treasury_addr).await?;
        Some(addr)
    };

    Ok((token_addr, presigner_addr, importer_addr))
}

/// Retrieve the stored LearnToken address if present
pub async fn get_learn_token_address(conn: &mut AsyncPgConnection) -> QueryResult<Option<Address>> {
    if let Some(s) = get_persistent_state(conn, "learn_token_address").await? {
        let parsed = Address::from_str(s.trim_start_matches("0x"))
            .map_err(|_| diesel::result::Error::NotFound)?;
        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}

/// On startup, ensure LearnToken is deployed. If already stored, reuse it; otherwise deploy and save.
pub async fn deploy_startup(
    conn: &mut AsyncPgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<Address> {
    if let Some(addr) = get_learn_token_address(conn).await? {
        Ok(addr)
    } else {
        deploy_learn_token_and_save(conn, name, symbol, decimals).await
    }
}