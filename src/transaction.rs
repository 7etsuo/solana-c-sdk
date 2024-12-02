use std::ffi::c_char;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    client,
    instruction::Instruction,
    program_pack::Pack,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use spl_associated_token_account;
use spl_token::state::Mint; // Add this line to import the module

use crate::wallet::SolKeyPair;
use crate::{client::SolClient, wallet::SolPublicKey};

#[repr(C)]
pub struct SolMint {
    pub mint_authority: *mut SolPublicKey,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: *mut SolPublicKey,
}

#[no_mangle]
pub extern "C" fn transfer_spl(
    client: *mut SolClient,
    signer_wallet: *mut SolKeyPair,
    sender: *mut SolPublicKey,
    recipient: *mut SolPublicKey,
    mint: *mut SolKeyPair,
    amount: u64,
) -> bool {
    // Safety: Ensure pointers are not null
    let client = unsafe {
        assert!(!client.is_null());
        &*client
    };

    let signer_wallet = unsafe {
        assert!(!signer_wallet.is_null());
        &*signer_wallet
    };

    let sender = unsafe {
        assert!(!sender.is_null());
        &*sender
    };

    let recipient = unsafe {
        assert!(!recipient.is_null());
        &*recipient
    };

    let mint = unsafe {
        assert!(!mint.is_null());
        &*mint
    };

    let sender_pubkey = Pubkey::new_from_array(sender.data);
    let recipient_pubkey = Pubkey::new_from_array(recipient.data);
    let mint_pubkey = mint.keypair.pubkey();

    // Step 1: Get or create recipient's associated token account
    let recipient_assoc = match _get_or_create_associated_token_account(
        client,
        signer_wallet,
        &recipient_pubkey,
        &mint_pubkey,
    ) {
        Ok(assoc) => assoc,
        Err(err) => {
            eprintln!(
                "Error managing recipient's associated token account: {}",
                err
            );
            return false;
        }
    };
    
    // Step 2: Derive sender's associated token account
    let sender_assoc =
        spl_associated_token_account::get_associated_token_address(&sender_pubkey, &mint_pubkey);

    // Step 3: Create the transfer instruction
    let transfer_instruction = match spl_token::instruction::transfer(
        &spl_token::id(),
        &sender_assoc,
        &recipient_assoc,
        &signer_wallet.keypair.pubkey(),
        &[&signer_wallet.keypair.pubkey()],
        amount,
    ) {
        Ok(instruction) => instruction,
        Err(err) => {
            eprintln!("Error creating transfer instruction: {:?}", err);
            return false;
        }
    };

    // Step 4: Fetch the recent blockhash
    let recent_blockhash = match client.rpc_client.get_latest_blockhash() {
        Ok(blockhash) => blockhash,
        Err(err) => {
            eprintln!("Error fetching latest blockhash: {:?}", err);
            return false;
        }
    };

    // Step 5: Create and sign the transaction
    let mut transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&signer_wallet.keypair.pubkey()), // Fee payer
        &[&signer_wallet.keypair],             // Required signers
        recent_blockhash,
    );

    // Step 6: Send and confirm the transaction
    match client.rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(_) => {
            println!(
                "Successfully transferred {} tokens from {} to {}",
                amount, sender_assoc, recipient_assoc
            );
            true
        }
        Err(err) => {
            eprintln!("Error sending and confirming transaction: {:?}", err);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn create_spl_token(
    client: *mut SolClient,
    payer: *mut SolKeyPair,
    mint: *mut SolKeyPair,
) -> bool {
    // Safety: Ensure the client pointer is not null
    let client = unsafe {
        assert!(!client.is_null());
        &*client
    };

    let payer = unsafe {
        assert!(!payer.is_null());
        &*payer
    };

    let mint = unsafe {
        assert!(!mint.is_null());
        &*mint
    };

    let minimum_balance_for_rent_exemption = match client
        .rpc_client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
    {
        Ok(balance) => balance,
        Err(err) => {
            eprintln!(
                "Error getting minimum balance for rent exemption: {:?}",
                err
            );
            return false;
        }
    };

    let create_account_instruction: Instruction = solana_sdk::system_instruction::create_account(
        &&payer.keypair.pubkey(),
        &mint.keypair.pubkey(),
        minimum_balance_for_rent_exemption,
        Mint::LEN as u64,
        &spl_token::ID,
    );

    // Create the mint instruction
    let mint_instruction = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint.keypair.pubkey(),
        &mint.keypair.pubkey(),
        None,
        9, // Decimals
    );

    let mint_instruction = match mint_instruction {
        Ok(instruction) => instruction,
        Err(err) => {
            eprintln!("Error creating mint instruction: {:?}", err);
            return false;
        }
    };

    let recent_blockhash = match client.rpc_client.get_latest_blockhash() {
        Ok(blockhash) => blockhash,
        Err(err) => {
            eprintln!("Error getting latest blockhash: {:?}", err);
            return false;
        }
    };

    // Create and sign the transaction
    let mut transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, mint_instruction],
        Some(&payer.keypair.pubkey()),
        &[&mint.keypair, &payer.keypair],
        recent_blockhash,
    );

    // Send the transaction
    match client.rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(_) => true,
        Err(err) => {
            eprintln!("Error sending and confirming transaction: {:?}", err);
            return false;
        }
    }
}

#[no_mangle]
pub extern "C" fn get_mint_info(client: *mut SolClient, mint: *mut SolKeyPair) -> *mut SolMint {
    // Safety: Ensure the client pointer is not null
    let client = unsafe {
        assert!(!client.is_null());
        &*client
    };

    let mint = unsafe {
        assert!(!mint.is_null());
        &*mint
    };

    let mint_info = match client.rpc_client.get_account_data(&mint.keypair.pubkey()) {
        Ok(data) => data,
        Err(_) => return std::ptr::null_mut(),
    };

    let mint_info = match Mint::unpack(&mint_info) {
        Ok(mint_info) => mint_info,
        Err(_) => return std::ptr::null_mut(),
    };

    let mint_authority = SolPublicKey {
        data: mint_info
            .mint_authority
            .map_or([0u8; 32], |pubkey| pubkey.to_bytes()),
    };

    let freeze_authority = SolPublicKey {
        data: mint_info.freeze_authority.unwrap_or_default().to_bytes(),
    };

    let sol_mint = SolMint {
        mint_authority: Box::into_raw(Box::new(mint_authority)),
        supply: mint_info.supply,
        decimals: mint_info.decimals,
        is_initialized: mint_info.is_initialized,
        freeze_authority: Box::into_raw(Box::new(freeze_authority)),
    };

    Box::into_raw(Box::new(sol_mint))
}

#[no_mangle]
pub extern "C" fn get_or_create_associated_token_account(
    client: *mut SolClient,
    payer: *mut SolKeyPair,
    owner: *mut SolPublicKey,
    mint: *mut SolKeyPair,
) -> *mut SolPublicKey {
    // Safety: Ensure pointers are not null
    let client = unsafe {
        assert!(!client.is_null());
        &*client
    };
    let payer = unsafe {
        assert!(!payer.is_null());
        &*payer
    };
    let owner = unsafe {
        assert!(!owner.is_null());
        &*owner
    };
    let mint = unsafe {
        assert!(!mint.is_null());
        &*mint
    };

    // Extract public keys
    let owner_pubkey = Pubkey::new_from_array(owner.data);
    let mint_pubkey = mint.keypair.pubkey();

    // Call the helper function to get or create the associated token account
    match _get_or_create_associated_token_account(client, payer, &owner_pubkey, &mint_pubkey) {
        Ok(assoc) => Box::into_raw(Box::new(SolPublicKey {
            data: assoc.to_bytes(),
        })),
        Err(err) => {
            eprintln!("Error managing associated token account: {}", err);
            std::ptr::null_mut()
        }
    }
}

pub fn _get_or_create_associated_token_account(
    client: &SolClient,
    signer_wallet: &SolKeyPair,
    recipient_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
) -> Result<Pubkey, String> {
    let assoc =
        spl_associated_token_account::get_associated_token_address(recipient_pubkey, mint_pubkey);

    match client.rpc_client.get_account(&assoc) {
        Ok(account) => {
            // Associated token account exists
            println!("Associated token account already exists at: {}", assoc);
            Ok(assoc)
        }
        Err(ref err)
            if matches!(
                err.kind(),
                solana_client::client_error::ClientErrorKind::RpcError(_)
            ) =>
        {
            // Create the associated token account
            println!("Associated token account does not exist. Proceeding to create...");
            let assoc_instruction =
                spl_associated_token_account::instruction::create_associated_token_account(
                    &signer_wallet.keypair.pubkey(),
                    recipient_pubkey,
                    mint_pubkey,
                    &spl_token::id(),
                );

            let recent_blockhash = client
                .rpc_client
                .get_latest_blockhash()
                .map_err(|err| format!("Error fetching latest blockhash: {:?}", err))?;

            let mut assoc_transaction = Transaction::new_signed_with_payer(
                &[assoc_instruction],
                Some(&signer_wallet.keypair.pubkey()),
                &[&signer_wallet.keypair],
                recent_blockhash,
            );

            client
                .rpc_client
                .send_and_confirm_transaction(&assoc_transaction)
                .map_err(|err| format!("Error creating associated token account: {:?}", err))?;

            println!(
                "Associated token account created successfully at: {}",
                assoc
            );
            Ok(assoc)
        }
        Err(err) => Err(format!(
            "Unexpected error checking associated token account: {:?}",
            err
        )),
    }
}

#[no_mangle]
pub extern "C" fn mint_spl(
    client: *mut SolClient,
    signer_wallet: *mut SolKeyPair,
    mint_authority: *mut SolKeyPair,
    recipient: *mut SolPublicKey,
    amount: u64,
) -> bool {
    // Safety: Ensure pointers are not null
    let client = unsafe {
        assert!(!client.is_null());
        &*client
    };
    let signer_wallet = unsafe {
        assert!(!signer_wallet.is_null());
        &*signer_wallet
    };
    let mint_authority = unsafe {
        assert!(!mint_authority.is_null());
        &*mint_authority
    };
    let recipient = unsafe {
        assert!(!recipient.is_null());
        &*recipient
    };

    let mint_authority_pubkey = mint_authority.keypair.pubkey();
    let recipient_pubkey = Pubkey::new_from_array(recipient.data);

    // Get or create associated token account
    let assoc = match _get_or_create_associated_token_account(
        client,
        signer_wallet,
        &recipient_pubkey,
        &mint_authority_pubkey,
    ) {
        Ok(assoc) => assoc,
        Err(err) => {
            eprintln!("Error managing associated token account: {}", err);
            return false;
        }
    };

    // Step 3: Create the mint_to instruction
    let mint_instruction = match spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_authority_pubkey,
        &assoc,
        &mint_authority.keypair.pubkey(),
        &[&mint_authority.keypair.pubkey()],
        amount,
    ) {
        Ok(instruction) => instruction,
        Err(err) => {
            eprintln!("Error creating mint instruction: {:?}", err);
            return false;
        }
    };

    // Step 4: Fetch the recent blockhash
    let recent_blockhash = match client.rpc_client.get_latest_blockhash() {
        Ok(blockhash) => blockhash,
        Err(err) => {
            eprintln!("Error fetching latest blockhash: {:?}", err);
            return false;
        }
    };

    // Step 5: Create and sign the mint transaction
    let mut transaction = Transaction::new_signed_with_payer(
        &[mint_instruction],
        Some(&signer_wallet.keypair.pubkey()), // Fee payer
        &[&mint_authority.keypair, &signer_wallet.keypair], // Required signers
        recent_blockhash,
    );

    // Step 6: Send and confirm the mint transaction
    match client.rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(_) => {
            println!("Successfully minted {} tokens to {}", amount, assoc);
            true
        }
        Err(err) => {
            eprintln!("Error minting tokens: {:?}", err);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn get_associated_token_balance(
    client: *mut SolClient,
    owner: *mut SolPublicKey,
    mint: *mut SolKeyPair,
) -> u64 {
    // Safety: Ensure the client pointer is not null
    let client = unsafe {
        assert!(!client.is_null());
        &*client
    };

    let owner = unsafe {
        assert!(!owner.is_null());
        &*owner
    };

    let mint = unsafe {
        assert!(!mint.is_null());
        &*mint
    };

    let owner_pubkey = Pubkey::new_from_array(owner.data);
    let mint_pubkey = mint.keypair.pubkey();

    let assoc =
        spl_associated_token_account::get_associated_token_address(&owner_pubkey, &mint_pubkey);

    let balance = match client.rpc_client.get_token_account_balance(&assoc) {
        Ok(balance) => balance,
        Err(err) => {
            eprintln!("Error getting token account balance: {:?}", err);
            return 0;
        }
    };

    match balance.amount.parse::<u64>() {
        Ok(amount) => amount,
        Err(err) => {
            eprintln!("Error parsing token account balance: {:?}", err);
            0
        }
    }
}
