// use borsh::{BorshDeserialize, BorshSerialize};
// use sha2::{Digest, Sha256};
// use solana_client::rpc_client::RpcClient;
// use solana_program::pubkey::Pubkey;
// use solana_sdk::{
//     instruction::{AccountMeta, Instruction},
//     signature::{Keypair, Signer},
//     signer::EncodableKey,
//     system_program,
//     transaction::Transaction,
// };
// use std::ffi::{CStr, CString};
// use std::os::raw::c_char;
// use std::str::FromStr;
// ==================== Functional Utility Functions ==================== //

// Create a new Solana client and payer keypair
// #[no_mangle]
// pub extern "C" fn create_client_and_payer(
//     rpc_url: *const c_char,
//     payer_path: *const c_char,
// ) -> *mut SolClient {
//     let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_str().unwrap() };
//     let payer_path = unsafe { CStr::from_ptr(payer_path).to_str().unwrap() };

//     let client = RpcClient::new(rpc_url.to_string());
//     let payer = Keypair::read_from_file(payer_path).expect("Failed to load keypair");

//     let sol_client = SolClient {
//         rpc_client: client,
//         payer,
//     };
//     Box::into_raw(Box::new(sol_client))
// }

// #[repr(C)]
// pub struct SolClient {
//     rpc_client: RpcClient,
//     payer: Keypair,
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
// #[no_mangle]
// pub extern "C" fn send_transaction_c(
//     client: *mut SolClient,
//     program_id: *const c_char,
//     method_name: *const c_char,
//     account_pubkey: *const c_char,
// ) -> *mut c_char {
//     let client = unsafe { &mut *client };
//     let program_id = unsafe { CStr::from_ptr(program_id).to_str().unwrap() };
//     let method_name = unsafe { CStr::from_ptr(method_name).to_str().unwrap() };
//     let account_pubkey = unsafe { CStr::from_ptr(account_pubkey).to_str().unwrap() };

//     let pubkey = Pubkey::from_str(account_pubkey).unwrap();

//     let instruction = create_instruction(
//         program_id,
//         method_name,
//         vec![
//             AccountMeta::new(pubkey, false),
//             AccountMeta::new_readonly(client.payer.pubkey(), true),
//         ],
//     );

//     let blockhash = client.rpc_client.get_latest_blockhash().unwrap();
//     let transaction = Transaction::new_signed_with_payer(
//         &[instruction],
//         Some(&client.payer.pubkey()),
//         &[&client.payer],
//         blockhash,
//     );

//     let result = client.rpc_client.send_and_confirm_transaction(&transaction);
//     match result {
//         Ok(sig) => CString::new(sig.to_string()).unwrap().into_raw(),
//         Err(_) => CString::new("Transaction failed").unwrap().into_raw(),
//     }
// }

// // Initialize an account
// #[no_mangle]
// pub extern "C" fn initialize_account_c(client: *mut SolClient, program_id: *const c_char) {
//     let client = unsafe { &mut *client };
//     let program_id = unsafe { CStr::from_ptr(program_id).to_str().unwrap() };

//     let account = Keypair::new();
//     let instruction = create_instruction(
//         program_id,
//         "initialize",
//         vec![
//             AccountMeta::new(account.pubkey(), true),
//             AccountMeta::new(client.payer.pubkey(), true),
//             AccountMeta::new_readonly(system_program::ID, false),
//         ],
//     );

//     let blockhash = client.rpc_client.get_latest_blockhash().unwrap();
//     let transaction = Transaction::new_signed_with_payer(
//         &[instruction],
//         Some(&client.payer.pubkey()),
//         &[&client.payer, &account],
//         blockhash,
//     );

//     client
//         .rpc_client
//         .send_and_confirm_transaction(&transaction)
//         .unwrap();
//     println!("Account initialized: {}", account.pubkey());
// }
