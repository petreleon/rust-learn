use rust_learn::utils::eth_utils::{compile_contract, deploy_contract, get_provider, load_wallet_from_env};
use ethers::prelude::*;
use ethers::signers::MnemonicBuilder;
use ethers::signers::coins_bip39::English;
use bip39::Mnemonic;
use getrandom::getrandom;

// This test requires a running Anvil node accessible via the .env configuration.
// It is ignored by default.
// Run it explicitly when your dev environment is up:
// `cargo test --test blockchain_integration_tests -- --ignored`
#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_deploy_and_mint() {
    // 1. Setup: Load wallet, provider, and compile contract
    // Use a dedicated derivation index to avoid nonce clashes with any persistent anvil state
    let mnemonic = std::env::var("ETH_MNEMONIC").expect("ETH_MNEMONIC not set for test");
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .index(2u32)
        .expect("failed to set derivation index")
        .build()
        .expect("failed to build deployer wallet")
        .with_chain_id(31337u64);
    let provider = get_provider();

    // Compile and deploy LearnToken
    let (abi, bytecode) = compile_contract("LearnToken.sol", "LearnToken");
    let token_addr = deploy_contract(
        wallet.clone(),
        provider.clone(),
        abi.clone(),
        bytecode,
        ("Test Token".to_string(), "TST".to_string(), 18u8),
    )
    .await;

    let client = std::sync::Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
    let token = Contract::new(token_addr, abi.clone(), client.clone());

    // Mint tokens to the test wallet
    let my_addr = wallet.address();
    let amount_to_mint = U256::from(100) * U256::from(10).pow(U256::from(18)); // 100 tokens
    let mint_call: ContractCall<_, ()> = token.method("mint", (my_addr, amount_to_mint)).unwrap();
    let pending = mint_call.send().await.expect("mint send");
    let _ = pending.await.expect("mint confirm").unwrap();

    // Verify minted balance
    let balance: U256 = token.method::<_, U256>("balanceOf", my_addr).unwrap().call().await.expect("balanceOf");
    assert_eq!(balance, amount_to_mint);

    // Deploy LearnTokenPresigner
    let (presigner_abi, presigner_bytecode) = compile_contract("LearnTokenPresigner.sol", "LearnTokenPresigner");
    let presigner_addr = deploy_contract(
        wallet.clone(),
        provider.clone(),
        presigner_abi.clone(),
        presigner_bytecode,
        (token_addr,),
    )
    .await;
    let presigner = Contract::new(presigner_addr, presigner_abi.clone(), client.clone());

    // Approve presigner to transfer tokens and deposit
    let approve_call = token.method::<_, bool>("approve", (presigner_addr, amount_to_mint)).unwrap();
    let p = approve_call.send().await.expect("approve send");
    let _ = p.await.expect("approve confirm").unwrap();

    // Deposit into presigner
    let deposit_call = presigner.method::<_, ()>("deposit", amount_to_mint).unwrap();
    let d = deposit_call.send().await.expect("deposit send");
    let _ = d.await.expect("deposit confirm").unwrap();

    // Withdraw back
    let withdraw_call = presigner.method::<_, ()>("withdraw", amount_to_mint).unwrap();
    let w = withdraw_call.send().await.expect("withdraw send");
    let _ = w.await.expect("withdraw confirm").unwrap();

    // Deploy PlatformImporter (we won't test permit here)
    let (importer_abi, importer_bytecode) = compile_contract("PlatformImporter.sol", "PlatformImporter");
    let treasury = my_addr; // use self as treasury for test
    let importer_addr = deploy_contract(
        wallet.clone(),
        provider.clone(),
        importer_abi.clone(),
        importer_bytecode,
        (treasury,),
    )
    .await;

    // Sanity: deployed addresses are non-zero
    assert_ne!(token_addr, Address::zero());
    assert_ne!(presigner_addr, Address::zero());
    assert_ne!(importer_addr, Address::zero());
}

// Test EIP-2612 permit flow with PlatformImporter.importWithPermit
#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_permit_import() {
    // Generate distinct mnemonics for deployer and owner
    let mut entropy_deployer = [0u8; 16];
    getrandom(&mut entropy_deployer).expect("failed to get randomness for deployer");
    let mnemonic_deployer = Mnemonic::from_entropy(&entropy_deployer).expect("failed to build deployer mnemonic");
    let phrase_deployer = mnemonic_deployer.to_string();
    eprintln!("[test_permit_import] generated deployer mnemonic: {}", phrase_deployer);

    let mut entropy_owner = [0u8; 16];
    getrandom(&mut entropy_owner).expect("failed to get randomness for owner");
    let mnemonic_owner = Mnemonic::from_entropy(&entropy_owner).expect("failed to build owner mnemonic");
    let phrase_owner = mnemonic_owner.to_string();
    eprintln!("[test_permit_import] generated owner mnemonic: {}", phrase_owner);

    // Deployer and owner come from different mnemonics (both at index 0)
    let deployer_wallet = MnemonicBuilder::<English>::default()
        .phrase(phrase_deployer.as_str())
        .index(0u32)
        .expect("failed to set derivation index")
        .build()
        .expect("failed to build deployer wallet")
        .with_chain_id(31337u64);
    let provider = get_provider();

    // Fund the freshly-generated deployer wallet from the default Anvil-funded faucet
    let faucet = load_wallet_from_env().with_chain_id(31337u64);
    let faucet_client = std::sync::Arc::new(SignerMiddleware::new(provider.clone(), faucet));
    let fund_value = U256::from(10u64) * U256::from(10).pow(U256::from(18)); // 10 ETH
    let tx = TransactionRequest::pay(deployer_wallet.address(), fund_value);
    let pending = faucet_client
        .send_transaction(tx, None)
        .await
        .expect("funding deployer tx send");
    let _ = pending.await.expect("funding deployer tx confirm");

    // Deploy token
    let (abi, bytecode) = compile_contract("LearnToken.sol", "LearnToken");
    let token_addr = deploy_contract(
        deployer_wallet.clone(),
        provider.clone(),
        abi.clone(),
        bytecode,
        ("Permit Token".to_string(), "PTKN".to_string(), 18u8),
    )
    .await;

    let client = std::sync::Arc::new(SignerMiddleware::new(provider.clone(), deployer_wallet.clone()));
    let token = Contract::new(token_addr, abi.clone(), client.clone());

    // Derive a separate owner wallet from the same mnemonic at index 1 (this wallet will sign the permit)
    let owner_wallet = MnemonicBuilder::<English>::default()
        .phrase(phrase_owner.as_str())
        .index(0u32)
        .expect("failed to set derivation index")
        .build()
        .expect("failed to build owner wallet");
    let owner = owner_wallet.address();

    // Mint tokens to owner
    let amount = U256::from(50) * U256::from(10).pow(U256::from(18));
    let _ = token.method::<_, ()>("mint", (owner, amount)).unwrap().send().await.unwrap().await.unwrap();

    // Deploy importer with treasury = random address
    let (importer_abi, importer_bytecode) = compile_contract("PlatformImporter.sol", "PlatformImporter");
    let treasury = Address::random();
    let importer_addr = deploy_contract(
        deployer_wallet.clone(),
        provider.clone(),
        importer_abi.clone(),
        importer_bytecode,
        (treasury,),
    )
    .await;
    let importer = Contract::new(importer_addr, importer_abi.clone(), client.clone());

    // Build permit signature following EIP-2612
    use ethers::utils::keccak256;
    use ethers::core::types::H256;

    // typehash
    let typehash = keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)".as_bytes());

    // fetch nonce and domain separator from token
    let nonce: U256 = token.method::<_, U256>("nonces", owner).unwrap().call().await.unwrap();
    let domain_separator: H256 = token.method::<_, H256>("DOMAIN_SEPARATOR", ()).unwrap().call().await.unwrap();

    // deadline
    let deadline = U256::from(9999999999u64);

    // helper to make 32-byte arrays
    fn pad_u256(value: U256) -> [u8; 32] {
        let mut b = [0u8; 32];
        value.to_big_endian(&mut b);
        b
    }
    fn pad_address(addr: Address) -> [u8; 32] {
        let mut b = [0u8; 32];
        let bytes = addr.as_bytes();
        b[12..32].copy_from_slice(bytes);
        b
    }

    // struct hash = keccak256(typehash || owner || spender || value || nonce || deadline)
    let mut enc = Vec::with_capacity(32 * 6);
    enc.extend_from_slice(&typehash);
    enc.extend_from_slice(&pad_address(owner));
    enc.extend_from_slice(&pad_address(importer_addr));
    enc.extend_from_slice(&pad_u256(amount));
    enc.extend_from_slice(&pad_u256(nonce));
    enc.extend_from_slice(&pad_u256(deadline));
    let struct_hash = keccak256(&enc);

    // digest = keccak256("\x19\x01" || domain_separator || struct_hash)
    let mut digest_input = Vec::with_capacity(2 + 32 + 32);
    digest_input.push(0x19u8);
    digest_input.push(0x01u8);
    digest_input.extend_from_slice(domain_separator.as_bytes());
    digest_input.extend_from_slice(&struct_hash);
    let digest = keccak256(&digest_input);
    let digest_h256 = H256::from_slice(&digest);

    // Sign digest with owner's wallet (synchronous return)
    let sig = owner_wallet.sign_hash(digest_h256).unwrap();
    let v = sig.v as u8;
    let r = sig.r;
    let s = sig.s;
    // Convert to raw 32-byte arrays to match bytes32 exactly
    let r_bytes: [u8; 32] = pad_u256(r);
    let s_bytes: [u8; 32] = pad_u256(s);

    // Call importWithPermit
    let tx = importer
        .method::<_, ()>("importWithPermit", (token_addr, owner, amount, U256::from(9999999999u64), v, r_bytes, s_bytes))
        .unwrap()
        .send()
        .await
        .expect("import tx send")
        .await
        .expect("import tx confirm");

    // Check treasury balance increased
    let bal: U256 = token.method::<_, U256>("balanceOf", treasury).unwrap().call().await.unwrap();
    assert_eq!(bal, amount);
}
