use serde_json;
use solana_sdk::signature::{Keypair, Signer};
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::os::raw::c_char;

// Generate and save a Solana wallet, returning the public key as a C string
#[no_mangle]
pub extern "C" fn create_and_save_wallet(file_path: *const c_char) -> *mut c_char {
    // Convert the C string to a Rust string
    let c_str = unsafe { CStr::from_ptr(file_path) };
    let file_path_str = match c_str.to_str() {
        Ok(str) => str,
        Err(_) => return std::ptr::null_mut(),
    };

    // Create a new keypair (Solana wallet)
    let keypair = Keypair::new();
    let public_key = keypair.pubkey().to_string();

    // Save the private key in Solana CLI format (JSON array)
    match save_wallet_to_file(&keypair, file_path_str) {
        Ok(_) => {
            // Return the public key as a C string
            let c_str_public_key = CString::new(public_key).unwrap();
            c_str_public_key.into_raw()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

// Save the wallet's private key to a file in Solana CLI format
fn save_wallet_to_file(keypair: &Keypair, file_path: &str) -> std::io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    // Convert the keypair's secret key to a byte array and serialize to JSON
    let secret_key_bytes = keypair.to_bytes();
    let json_data = serde_json::to_string(&secret_key_bytes.to_vec())?;

    // Write the JSON data to the file
    writer.write_all(json_data.as_bytes())?;
    writer.flush()?;
    Ok(())
}

// Load a Solana wallet from the file, returning the public key as a C string
#[no_mangle]
pub extern "C" fn load_wallet_from_file(file_path: *const c_char) -> *mut c_char {
    // Convert the C string to a Rust string
    let c_str = unsafe { CStr::from_ptr(file_path) };
    let file_path_str = match c_str.to_str() {
        Ok(str) => str,
        Err(_) => return std::ptr::null_mut(),
    };

    // Load the private key from the file in Solana CLI format
    match load_wallet(file_path_str) {
        Ok(keypair) => {
            // Return the public key as a C string
            let public_key = keypair.pubkey().to_string();
            let c_str_public_key = CString::new(public_key).unwrap();
            c_str_public_key.into_raw()
        }
        Err(_) => std::ptr::null_mut(),
    }
}

// Load the wallet's private key from the file in Solana CLI format
fn load_wallet(file_path: &str) -> Result<Keypair, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Deserialize the JSON back into a byte array
    let secret_key_bytes: Vec<u8> = serde_json::from_reader(reader)?;

    // Ensure the byte array is exactly 64 bytes long
    if secret_key_bytes.len() != 64 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid key length",
        ));
    }

    // Create a keypair from the secret key bytes
    let keypair = Keypair::from_bytes(&secret_key_bytes).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to load keypair from bytes",
        )
    })?;

    Ok(keypair)
}

// Free a C string passed back to C/C++ from Rust
#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        CString::from_raw(s);
    }
}
