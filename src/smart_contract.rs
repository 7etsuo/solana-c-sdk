use borsh::{BorshDeserialize, BorshSerialize};
use sha2::{Digest, Sha256};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    signer::EncodableKey,
    system_program,
    transaction::Transaction,
};
use spl_associated_token_account::tools::account;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::str::FromStr;

use crate::{
    client::SolClient,
    wallet::{SolKeyPair, SolPublicKey},
};

// ==================== Utility Functions ==================== //

// Load Payer Keypair

// Compute Discriminator
fn get_discriminator(method_name: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", method_name).as_bytes());
    hasher.finalize()[..8].to_vec()
}

// Create Instruction for Anchor Methods
fn create_instruction(
    program_id: &str,
    method_name: &str,
    accounts: Vec<AccountMeta>,
) -> Instruction {
    let discriminator = get_discriminator(method_name);
    let program_id = Pubkey::from_str(program_id).expect("Invalid program ID");
    Instruction::new_with_bytes(program_id, &discriminator, accounts)
}

// ==================== Transaction Functions ==================== //

// Send Transaction
#[no_mangle]
pub extern "C" fn send_transaction_c(
    client: *mut SolClient,
    payer: *mut SolKeyPair,
    program_id: *const c_char,
    method_name: *const c_char,
    account_pubkey: *mut SolPublicKey,
) -> *mut c_char {
    let client = unsafe { &mut *client };
    let payer = unsafe { &mut *payer };

    let program_id = unsafe { CStr::from_ptr(program_id).to_str().unwrap() };
    let method_name = unsafe { CStr::from_ptr(method_name).to_str().unwrap() };

    // Handle pubkey parsing safely
    let pubkey = unsafe { (*account_pubkey).to_pubkey() };

    let instruction = create_instruction(
        program_id,
        method_name,
        vec![
            AccountMeta::new(pubkey, false),
            AccountMeta::new_readonly(payer.to_keypair().pubkey(), true),
        ],
    );

    let blockhash = match client.rpc_client.get_latest_blockhash() {
        Ok(bh) => bh,
        Err(e) => {
            eprintln!("❌ Failed to fetch blockhash: {:?}", e);
            return CString::new("Failed to fetch blockhash")
                .unwrap()
                .into_raw();
        }
    };

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.to_keypair().pubkey()),
        &[&payer.to_keypair()],
        blockhash,
    );

    let result = client.rpc_client.send_and_confirm_transaction(&transaction);
    match result {
        Ok(sig) => CString::new(sig.to_string()).unwrap().into_raw(),
        Err(err) => {
            eprintln!("❌ Transaction failed: {:?}", err);
            CString::new(format!("Transaction failed: {:?}", err))
                .unwrap()
                .into_raw()
        }
    }
}

// Initialize Account
#[no_mangle]
pub extern "C" fn initialize_account_c(
    client: *mut SolClient,
    payer: *mut SolKeyPair,
    account: *mut SolKeyPair,
    program_id: *const c_char,
) {
    let client = unsafe { &mut *client };
    let payer = unsafe { &mut *payer };

    let program_id = unsafe { CStr::from_ptr(program_id).to_str().unwrap() };
    let account = &unsafe { &mut *account }.to_keypair();

    let instruction = create_instruction(
        program_id,
        "initialize",
        vec![
            AccountMeta::new(account.pubkey(), true),
            AccountMeta::new(payer.to_keypair().pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );

    let blockhash = match client.rpc_client.get_latest_blockhash() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Failed to fetch latest blockhash: {:?}", e);
            return;
        }
    };

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.to_keypair().pubkey()),
        &[&payer.to_keypair(), &account],
        blockhash,
    );

    match client.rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(sig) => {
            println!("✅ Account initialized: {}", account.pubkey());
            println!("Transaction Signature: {}", sig);
        }
        Err(err) => {
            eprintln!("❌ Failed to initialize account: {:?}", err);
        }
    }
}

// ==================== Free Memory ==================== //
#[no_mangle]
pub extern "C" fn free_client(client: *mut SolClient) {
    if client.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(client);
    }
}

#[no_mangle]
pub extern "C" fn free_payer(payer: *mut SolKeyPair) {
    if payer.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(payer);
    }
}
