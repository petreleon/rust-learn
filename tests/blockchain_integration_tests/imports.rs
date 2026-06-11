use anyhow::{Context, Result};
use bip39::Mnemonic;
use ethers::prelude::*;
use ethers::signers::coins_bip39::English;
use ethers::signers::MnemonicBuilder;
use getrandom::getrandom;
use rust_learn::utils::eth_utils::{
    try_compile_contract, try_deploy_contract, try_get_provider, try_load_wallet_from_env,
};
