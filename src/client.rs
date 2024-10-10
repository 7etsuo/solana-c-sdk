use std::ffi::{c_char, CStr};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use crate::wallet::SolPublicKey;

pub struct SolClient {
    pub rpc_client: RpcClient,
}

#[no_mangle]
pub extern "C" fn new_sol_client(url: *const c_char) -> *mut SolClient {
    // Convert the C string to a Rust string
    let c_str = unsafe { CStr::from_ptr(url) };
    let url_str = match c_str.to_str() {
        Ok(str) => str,
        Err(_) => return std::ptr::null_mut(),
    };

    // Create a new Solana client
    let rpc_client = RpcClient::new(url_str.to_string());
    let client = SolClient { rpc_client };
    Box::into_raw(Box::new(client))
}

#[no_mangle]
pub extern "C" fn get_balance(client: *mut SolClient, pubkey: *mut SolPublicKey) -> u64 {
    let client = unsafe {
        assert!(!client.is_null());
        &mut *client
    };

    let pubkey = unsafe {
        assert!(!pubkey.is_null());
        &*pubkey
    };

    let pubkey = Pubkey::new_from_array(pubkey.data);
    client.rpc_client.get_balance(&pubkey).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn request_airdrop(client: *mut SolClient, pubkey: *mut SolPublicKey, lamports: u64) -> bool {
    let client = unsafe {
        assert!(!client.is_null());
        &mut *client
    };

    let pubkey = unsafe {
        assert!(!pubkey.is_null());
        &*pubkey
    };

    let pubkey = Pubkey::new_from_array(pubkey.data);
    println!("Requesting airdrop of {} lamports to pubkey: {:?}", lamports, pubkey);
    match client.rpc_client.request_airdrop(&pubkey, lamports) {
        Ok(signature) => {
            println!("Airdrop requested successfully. Signature: {:?}", signature);
            true
        },
        Err(err) => {
            println!("Failed to request airdrop: {:?}", err);
            false
        }
    }
}