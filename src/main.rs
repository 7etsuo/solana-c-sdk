use borsh::BorshDeserialize; // Use Borsh for deserialization
use sha2::{Digest, Sha256};
use shellexpand;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    signer::EncodableKey,
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;

const DISCRIMINATOR: usize = 8;
// Define the Counter account structure
#[derive(BorshDeserialize, Debug)]
pub struct Counter {
    pub count: u64, // Actual counter value stored in the account
}

fn initialize_counter(client: &RpcClient, payer: &Keypair, program_id: &Pubkey, counter: &Keypair) {
    println!("🔧 Initializing counter...");

    // Compute discriminator for "initialize"
    let method_discriminator = get_discriminator("initialize");

    // Instruction to initialize the counter account
    let instruction = Instruction::new_with_bytes(
        *program_id,
        &method_discriminator,
        vec![
            AccountMeta::new(counter.pubkey(), true), // Mark counter as signer (fix)
            AccountMeta::new(payer.pubkey(), true),   // Payer (signer)
            AccountMeta::new_readonly(system_program::ID, false),
        ],
    );

    // Pass both the payer and the counter as signers
    send_transaction(client, &[instruction], &[payer, &counter]);
    println!("✅ Counter initialized at: {}", counter.pubkey());
}

#[tokio::main]
async fn main() {
    // Step 1: Connect to Solana Devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);
    println!("✅ Connected to Solana Devnet");

    // Step 2: Load Payer Wallet
    let payer = Keypair::read_from_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Failed to load keypair");
    println!("✅ Loaded payer wallet: {}", payer.pubkey());

    // Step 3: Program ID of the deployed Anchor program
    let program_id = Pubkey::from_str("DsfPR2teuRS9ABmqGqq5NobD8Y9A9KvzMVNVzsjSP8Dy").unwrap();
    println!("✅ Program ID: {}", program_id);
    let counter = Keypair::new(); // New keypair for the counter
    println!("🔑 New Counter Pubkey: {}", counter.pubkey());

    initialize_counter(&client, &payer, &program_id, &counter);

    // Step 4: Use the existing counter account pubkey
    let counter_pubkey = counter.pubkey();
    println!("🚀 Using existing Counter Account: {}", counter_pubkey);

    // Step 5: Increment the Counter
    call_anchor_method(&client, &payer, &program_id, &counter_pubkey, "increment");
    fetch_counter_data(&client, &counter_pubkey); // Fetch and display counter value

    // Step 6: Decrement the Counter
    call_anchor_method(&client, &payer, &program_id, &counter_pubkey, "decrement");
    fetch_counter_data(&client, &counter_pubkey); // Fetch and display counter value

    println!("🎉 Program calls completed successfully!");
}

/// Compute the 8-byte Anchor discriminator for a method
fn get_discriminator(method_name: &str) -> Vec<u8> {
    let formatted_name = format!("global:{}", method_name);
    let mut hasher = Sha256::new();
    hasher.update(formatted_name.as_bytes());
    hasher.finalize()[..8].to_vec()
}

/// Call an Anchor method (e.g., "increment" or "decrement")
fn call_anchor_method(
    client: &RpcClient,
    payer: &Keypair,
    program_id: &Pubkey,
    counter_pubkey: &Pubkey,
    method: &str,
) {
    println!("🔨 Calling '{}' method...", method);

    // Compute the method discriminator dynamically
    let method_discriminator = get_discriminator(method);
    println!("Discriminator for '{}': {:?}", method, method_discriminator);

    // Create the instruction for the Anchor method
    let instruction = Instruction::new_with_bytes(
        *program_id,
        &method_discriminator,
        vec![
            AccountMeta::new(*counter_pubkey, false), // Writable counter account
            AccountMeta::new_readonly(payer.pubkey(), true), // Payer (signer)
        ],
    );

    // Send the transaction
    send_transaction(client, &[instruction], &[payer]);
    println!("✅ '{}' method called successfully!", method);
}

/// Fetch and display the counter account data
fn fetch_counter_data(client: &RpcClient, counter_pubkey: &Pubkey) {
    println!("📥 Fetching Counter Account Data...");

    // Fetch the account data
    let account_data = client
        .get_account(counter_pubkey)
        .expect("Failed to fetch account data");

    // Skip the first 8 bytes (Anchor discriminator) and deserialize the data
    let counter = Counter::try_from_slice(&account_data.data[8..])
        .expect("Failed to deserialize account data");

    println!("✅ Current Counter Value: {}", counter.count);
}

/// Send a Solana transaction
fn send_transaction(client: &RpcClient, instructions: &[Instruction], signers: &[&Keypair]) {
    let blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");

    let transaction = Transaction::new_signed_with_payer(
        instructions,
        Some(&signers[0].pubkey()),
        signers,
        blockhash,
    );

    match client.send_and_confirm_transaction(&transaction) {
        Ok(sig) => println!("✅ Transaction successful! Signature: {}", sig),
        Err(e) => {
            eprintln!("❌ Transaction failed: {:?}", e);
            std::process::exit(1);
        }
    }
}
