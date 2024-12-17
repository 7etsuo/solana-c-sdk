use std::ffi::{c_char, CString};

use solana_account_decoder::UiAccountData;
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::{
    instruction::Instruction, program_pack::Pack, pubkey::Pubkey, signer::Signer,
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

#[repr(C)]
pub struct TokenInfo {
    mint: *const c_char,    // Token mint as a string
    balance: *const c_char, // Token balance as a string
    owner: *const c_char,   // Token owner as a string
}

#[repr(C)]
pub struct TokenList {
    data: *mut TokenInfo, // Pointer to an array of `TokenInfo`
    len: usize,           // Length of the array
}

#[no_mangle]
pub extern "C" fn get_all_tokens(
    client: *mut SolClient,
    wallet: *mut SolPublicKey,
) -> *mut TokenList {
    // Safety: Ensure pointers are not null
    let client = unsafe {
        assert!(!client.is_null());
        &*client
    };

    let wallet = unsafe {
        assert!(!wallet.is_null());
        &*wallet
    };

    let wallet_pubkey = Pubkey::new_from_array(wallet.data);

    // Fetch all token accounts owned by the wallet
    let token_accounts = match client.rpc_client.get_token_accounts_by_owner(
        &wallet_pubkey,
        TokenAccountsFilter::ProgramId(spl_token::id()),
    ) {
        Ok(accounts) => accounts,
        Err(err) => {
            eprintln!(
                "Error fetching token accounts for wallet {}: {:?}",
                wallet_pubkey, err
            );
            return std::ptr::null_mut();
        }
    };

    let mut tokens: Vec<TokenInfo> = Vec::new();

    for keyed_account in token_accounts {
        if let UiAccountData::Json(parsed_data) = keyed_account.account.data {
            if let Some(info) = parsed_data.parsed.get("info").and_then(|v| v.as_object()) {
                if let (Some(mint), Some(balance), Some(owner)) = (
                    info.get("mint").and_then(|v| v.as_str()),
                    info.get("tokenAmount")
                        .and_then(|v| v.get("uiAmountString"))
                        .and_then(|v| v.as_str()),
                    info.get("owner").and_then(|v| v.as_str()),
                ) {
                    let mint_c = CString::new(mint).unwrap();
                    let balance_c = CString::new(balance).unwrap();
                    let owner_c = CString::new(owner).unwrap();

                    tokens.push(TokenInfo {
                        mint: mint_c.into_raw(),
                        balance: balance_c.into_raw(),
                        owner: owner_c.into_raw(),
                    });
                }
            }
        } else {
            eprintln!(
                "Unexpected account data format for account: {}",
                keyed_account.pubkey
            );
        }
    }

    let token_list = Box::new(TokenList {
        data: tokens.as_mut_ptr(),
        len: tokens.len(),
    });

    std::mem::forget(tokens); // Prevent Rust from deallocating the vector
    Box::into_raw(token_list) // Pass ownership to C
}

#[no_mangle]
pub extern "C" fn token_list_get_data(list: *const TokenList) -> *mut TokenInfo {
    if list.is_null() {
        std::ptr::null_mut()
    } else {
        unsafe { (*list).data }
    }
}

#[no_mangle]
pub extern "C" fn token_list_get_len(list: *const TokenList) -> usize {
    if list.is_null() {
        0
    } else {
        unsafe { (*list).len }
    }
}

#[no_mangle]
pub extern "C" fn free_token_list(list: *mut TokenList) {
    if list.is_null() {
        return;
    }

    unsafe {
        let list = Box::from_raw(list);
        for i in 0..list.len {
            let token_info = &mut *list.data.add(i);
            CString::from_raw(token_info.mint as *mut c_char);
            CString::from_raw(token_info.balance as *mut c_char);
            CString::from_raw(token_info.owner as *mut c_char);
        }
        Vec::from_raw_parts(list.data, list.len, list.len);
    }
}

#[no_mangle]
pub extern "C" fn transfer_sol(
    client: *mut SolClient,
    signer_wallet: *mut SolKeyPair,
    sender: *mut SolPublicKey,
    recipient: *mut SolPublicKey,
    lamports: u64,
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

    let sender_pubkey = Pubkey::new_from_array(sender.data);
    let recipient_pubkey = Pubkey::new_from_array(recipient.data);

    // Verify that the sender's account exists
    match client.rpc_client.get_account(&sender_pubkey) {
        Ok(account) => {
            if account.lamports < lamports {
                eprintln!(
                    "Sender does not have enough SOL. Balance: {}, Required: {}",
                    account.lamports, lamports
                );
                return false;
            }
        }
        Err(err) => {
            eprintln!("Error checking sender's account: {:?}", err);
            return false;
        }
    }

    // Step 1: Create the transfer instruction
    let transfer_instruction =
        solana_sdk::system_instruction::transfer(&sender_pubkey, &recipient_pubkey, lamports);

    // Step 2: Fetch the recent blockhash
    let recent_blockhash = match client.rpc_client.get_latest_blockhash() {
        Ok(blockhash) => blockhash,
        Err(err) => {
            eprintln!("Error fetching latest blockhash: {:?}", err);
            return false;
        }
    };

    // Step 3: Create and sign the transaction
    let mut transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&signer_wallet.keypair.pubkey()), // Fee payer
        &[&signer_wallet.keypair],             // Required signer
        recent_blockhash,
    );

    // Step 4: Send and confirm the transaction
    match client.rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(_) => {
            println!(
                "Successfully transferred {} lamports from {} to {}",
                lamports, sender_pubkey, recipient_pubkey
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
