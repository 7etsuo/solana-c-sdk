// use borsh::{BorshDeserialize, BorshSerialize};
// use sha2::{Digest, Sha256};
// use solana_client::rpc_client::RpcClient;
// use solana_sdk::{
//     instruction::{AccountMeta, Instruction},
//     signature::{Keypair, Signer},
//     signer::EncodableKey,
//     system_program,
//     transaction::Transaction,
// };
// use solana_program::pubkey::Pubkey;
// use std::str::FromStr;

// // ==================== Functional Utility Functions ==================== //

// // Create a new Solana client and payer keypair
// fn create_client_and_payer(rpc_url: &str, payer_path: &str) -> (RpcClient, Keypair) {
//     let client = RpcClient::new(rpc_url.to_string());
//     let payer = Keypair::read_from_file(payer_path).expect("Failed to load keypair");
//     (client, payer)
// }

// // Compute the Anchor method discriminator
// fn get_discriminator(method_name: &str) -> Vec<u8> {
//     let mut hasher = Sha256::new();
//     hasher.update(format!("global:{}", method_name).as_bytes());
//     hasher.finalize()[..8].to_vec()
// }

// // Create an instruction for an Anchor method
// fn create_instruction(
//     program_id: &str,
//     method_name: &str,
//     accounts: Vec<AccountMeta>,
// ) -> Instruction {
//     let discriminator = get_discriminator(method_name);
//     let program_id = Pubkey::from_str(program_id).expect("Invalid program ID");
//     Instruction::new_with_bytes(program_id, &discriminator, accounts)
// }

// // Send a transaction
// fn send_transaction(
//     client: &RpcClient,
//     instructions: &[Instruction],
//     signers: &[&Keypair],
// ) -> String {
//     let blockhash = client
//         .get_latest_blockhash()
//         .expect("Failed to get blockhash");

//     let transaction = Transaction::new_signed_with_payer(
//         instructions,
//         Some(&signers[0].pubkey()),
//         signers,
//         blockhash,
//     );

//     client
//         .send_and_confirm_transaction(&transaction)
//         .expect("Transaction failed")
//         .to_string()
// }

// // Fetch and deserialize account data
// fn fetch_account_data<T: BorshDeserialize>(client: &RpcClient, pubkey: &Pubkey) -> T {
//     let account = client
//         .get_account(pubkey)
//         .expect("Failed to fetch account data");

//     T::try_from_slice(&account.data[8..]).expect("Failed to deserialize account data")
// }

// // ==================== Functional Contract Operations ==================== //

// // Initialize an account
// fn initialize_account(client: &RpcClient, payer: &Keypair, program_id: &str, account: &Keypair) {
//     let accounts = vec![
//         AccountMeta::new(account.pubkey(), true),
//         AccountMeta::new(payer.pubkey(), true),
//         AccountMeta::new_readonly(system_program::ID, false),
//     ];

//     let instruction = create_instruction(program_id, "initialize", accounts);
//     let signature = send_transaction(client, &[instruction], &[payer, account]);
//     println!(
//         "✅ Account initialized at: {} (Signature: {})",
//         account.pubkey(),
//         signature
//     );
// }

// // Call a method on the Anchor contract
// fn call_method(
//     client: &RpcClient,
//     payer: &Keypair,
//     program_id: &str,
//     method_name: &str,
//     accounts: Vec<AccountMeta>,
// ) {
//     let instruction = create_instruction(program_id, method_name, accounts);
//     let signature = send_transaction(client, &[instruction], &[payer]);
//     println!(
//         "✅ Method '{}' called (Signature: {})",
//         method_name, signature
//     );
// }

// // ==================== Example Usage ==================== //

// #[tokio::main]
// async fn main() {
//     // RPC URL and payer keypair path
//     let rpc_url = "https://api.devnet.solana.com";
//     let payer_path = "~/.config/solana/id.json";
//     let program_id = "DsfPR2teuRS9ABmqGqq5NobD8Y9A9KvzMVNVzsjSP8Dy";

//     // Create Solana client and payer
//     let (client, payer) = create_client_and_payer(rpc_url, &*shellexpand::tilde(payer_path));

//     // Initialize a new account
//     let account = Keypair::new();
//     initialize_account(&client, &payer, program_id, &account);

//     // Increment counter
//     call_method(
//         &client,
//         &payer,
//         program_id,
//         "increment",
//         vec![
//             AccountMeta::new(account.pubkey(), false),
//             AccountMeta::new_readonly(payer.pubkey(), true),
//         ],
//     );

//     // Fetch and display counter value
//     let counter_pubkey = account.pubkey();
//     let counter: Counter = fetch_account_data(&client, &counter_pubkey);
//     println!("🔢 Counter Value: {}", counter.count);

//     // Decrement counter
//     call_method(
//         &client,
//         &payer,
//         program_id,
//         "decrement",
//         vec![
//             AccountMeta::new(counter_pubkey, false),
//             AccountMeta::new_readonly(payer.pubkey(), true),
//         ],
//     );

//     // Fetch and display counter value
//     let counter: Counter = fetch_account_data(&client, &counter_pubkey);
//     println!("🔢 Counter Value: {}", counter.count);
// }

// // ==================== Counter Struct ==================== //

// #[derive(BorshDeserialize, BorshSerialize, Debug)]
// pub struct Counter {
//     pub count: u64,
// }
