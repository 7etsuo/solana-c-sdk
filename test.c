#include <stdio.h>
#include <stdlib.h>
#include "header/solana_lib.h"

const char *file_path = "wallet_keypair.json";
const char *file_path_payer = "wallet_keypair.json";
const char *file_path_recipient = "wallet_keypair_recipient.json";
const char *file_path_recipient2 = "wallet_keypair_recipient2.json";
const char *file_path_mint = "wallet_keypair_mint.json";
const char *devnet_url = "https://api.devnet.solana.com";

SolKeyPair *test_create_and_save_wallet(const char *file_path)
{
    printf("=== Test: Create and Save Wallet ===\n");
    SolKeyPair *wallet = create_and_save_wallet(file_path);
    if (wallet != NULL)
    {
        printf("Wallet created and saved successfully.\n");
        printf("Wallet Address: %s\n", get_wallet_address(wallet));
    }
    else
    {
        printf("Failed to create wallet.\n");
    }
    printf("=== End Test: Create and Save Wallet ===\n");
    return wallet;
}

SolKeyPair *test_create_and_save_mint_wallet()
{
    printf("Create mint wallet");
    return test_create_and_save_wallet(file_path_mint);
}

SolKeyPair *test_create_and_save_recipient_wallet()
{
    printf("Create Recipient wallet");
    return test_create_and_save_wallet(file_path_recipient);
}

SolKeyPair *test_create_and_save_recipient2_wallet()
{
    printf("Create Recipient2 wallet");
    return test_create_and_save_wallet(file_path_recipient2);
}

SolKeyPair *test_load_wallet_from_file(const char *file_path)
{
    SolKeyPair *wallet = load_wallet_from_file(file_path);
    // Check if the wallet loading succeeded
    if (wallet != NULL)
    {
        SolPublicKey *pub = get_public_key(wallet);
        // Print the loaded public key
        printf("Loaded Solana Wallet Public Key: %s\n", pub->data);
        // Print the wallet address
        printf("Loaded Solana Wallet Address: %s\n", get_wallet_address(wallet));
    }
    else
    {
        printf("Failed to load wallet.\n");
    }
    return wallet;
}

SolClient *test_sol_client_new(const char *url)
{
    SolClient *client = new_sol_client(url);
    if (client != NULL)
    {
        printf("Solana Client created successfully.\n");
    }
    else
    {
        printf("Failed to create Solana Client.\n");
    }
    return client;
}

void test_sol_airdrop()
{
    SolClient *client = new_sol_client(devnet_url);
    if (client != NULL)
    {
        SolKeyPair *wallet = load_wallet_from_file(file_path);
        if (wallet != NULL)
        {
            SolPublicKey *pub = get_public_key(wallet);
            uint64_t lamports = 100000000;
            bool success = request_airdrop(client, pub, lamports);
            if (success)
            {
                printf("Airdrop successful.\n");
            }
            else
            {
                printf("Airdrop failed.\n");
            }
            // get balance
            uint64_t balance = get_balance(client, pub);
            printf("Balance: %lu\n", balance);
        }
        else
        {
            printf("Failed to load wallet.\n");
        }
    }
    else
    {
        printf("Failed to create Solana Client.\n");
    }
}

void test_create_spl_token()
{
    SolClient *client = new_sol_client(devnet_url);
    if (client != NULL)
    {
        SolKeyPair *payer = load_wallet_from_file(file_path_payer);
        SolKeyPair *mint = load_wallet_from_file(file_path_mint);

        if (payer != NULL && mint != NULL)
        {
            printf("Solana mint Wallet Address: %s\n", get_wallet_address(mint));
            bool success = create_spl_token(client, payer, mint);
            if (success)
            {
                printf("SPL Token created successfully.\n");
            }
            else
            {
                printf("Failed to create SPL Token.\n");
            }

            SolMint *mint_info = get_mint_info(client, mint);
            if (mint_info != NULL)
            {
                printf("Mint Supply: %lu\n", mint_info->supply);
                printf("Mint Decimals: %u\n", mint_info->decimals);
                printf("Mint is initialized: %s\n", mint_info->is_initialized ? "true" : "false");
            }
            else
            {
                printf("Failed to get mint info.\n");
            }
        }
        else
        {
            printf("Failed to create wallets.\n");
        }
    }
    else
    {
        printf("Failed to create Solana Client.\n");
    }
}

void test_mint_spl_token()
{
    printf("=== Test: Mint SPL Token ===\n");

    // Create a Solana client
    SolClient *client = new_sol_client(devnet_url);
    if (client == NULL)
    {
        printf("Error: Failed to create Solana Client.\n");
        return;
    }

    // Load the payer wallet
    SolKeyPair *payer = load_wallet_from_file(file_path_payer);
    if (payer == NULL)
    {
        printf("Error: Failed to load payer wallet from file: %s\n", file_path_payer);
        return;
    }

    // Load the mint wallet
    SolKeyPair *mint = load_wallet_from_file(file_path_mint);
    if (mint == NULL)
    {
        printf("Error: Failed to load mint wallet from file: %s\n", file_path_mint);
        return;
    }

    // Load the recipient wallet
    SolKeyPair *recipient = load_wallet_from_file(file_path_payer);
    if (recipient == NULL)
    {
        printf("Error: Failed to load recipient wallet from file: %s\n", file_path_payer);
        return;
    }

    // Print recipient wallet address
    printf("Recipient Wallet Address: %s\n", get_wallet_address(recipient));

    // Define the amount to mint
    uint64_t amount = 1000000000000;

    // Perform the mint operation
    printf("Minting %lu tokens to recipient wallet...\n", amount);
    bool success = mint_spl(client, payer, mint, get_public_key(recipient), amount);
    if (success)
    {
        // Get the recipient's token balance
        uint64_t balance = get_associated_token_balance(client, get_public_key(recipient), mint);
        printf("Success: SPL Token minted successfully.\n");
        printf("Recipient Token Balance: %lu\n", balance);
    }
    else
    {
        printf("Error: Failed to mint SPL Token.\n");
    }

    printf("=== End Test: Mint SPL Token ===\n");
}

void test_transfer_spl_token()
{
    SolClient *client = new_sol_client(devnet_url);
    if (client != NULL)
    {
        SolKeyPair *signer_wallet = load_wallet_from_file(file_path_payer);
        SolKeyPair *mint = load_wallet_from_file(file_path_mint);
        SolKeyPair *recipient = load_wallet_from_file(file_path_recipient);

        if (signer_wallet != NULL && mint != NULL && recipient != NULL)
        {
            SolPublicKey *sender_pubkey = get_public_key(signer_wallet);
            SolPublicKey *recipient_pubkey = get_public_key(recipient);
            uint64_t amount = 500000000; // Transfer 500 tokens
            printf("Solana Token Transfer to  Wallet Address: %s\n", get_wallet_address(recipient));
            bool success = transfer_spl(client, signer_wallet, sender_pubkey, recipient_pubkey, mint, amount);
            if (success)
            {
                printf("SPL Token transferred successfully.\n");
            }
            else
            {
                printf("Failed to transfer SPL Token.\n");
            }
        }
        else
        {
            printf("Failed to load wallets for transfer.\n");
        }
    }
    else
    {
        printf("Failed to create Solana Client.\n");
    }
}

void test_transfer_sol()
{
    SolClient *client = new_sol_client(devnet_url);
    if (client != NULL)
    {
        SolKeyPair *signer_wallet = load_wallet_from_file(file_path_payer);
        SolKeyPair *recipient_wallet = load_wallet_from_file(file_path_recipient);

        if (signer_wallet != NULL && recipient_wallet != NULL)
        {
            SolPublicKey *signer_pubkey = get_public_key(signer_wallet);
            SolPublicKey *recipient_pubkey = get_public_key(recipient_wallet);
            uint64_t lamports = 1000000; // Transfer 0.001 SOL

            printf("Transferring %lu lamports (%.9f SOL) to Wallet Address: %s\n", lamports, lamports / 1e9, get_wallet_address(recipient_wallet));

            bool success = transfer_sol(client, signer_wallet, signer_pubkey, recipient_pubkey, lamports);

            if (success)
            {
                printf("Successfully transferred %lu lamports (%.9f SOL).\n", lamports, lamports / 1e9);
            }
            else
            {
                printf("Failed to transfer SOL.\n");
            }

            // Check balances after transfer
            uint64_t signer_balance = get_balance(client, signer_pubkey);
            uint64_t recipient_balance = get_balance(client, recipient_pubkey);

            printf("Signer Balance: %lu lamports (%.9f SOL)\n", signer_balance, signer_balance / 1e9);
            printf("Recipient Balance: %lu lamports (%.9f SOL)\n", recipient_balance, recipient_balance / 1e9);
        }
        else
        {
            printf("Failed to load wallets for SOL transfer.\n");
        }
    }
    else
    {
        printf("Failed to create Solana Client.\n");
    }
}

void test_get_all_tokens()
{
    printf("=== Test: Get All Tokens ===\n");
    SolClient *client = new_sol_client(devnet_url);
    if (client == NULL)
    {
        printf("Failed to create Solana Client.\n");
        return;
    }

    SolKeyPair *wallet = test_load_wallet_from_file(file_path_payer);
    if (wallet == NULL)
    {
        printf("Failed to load wallet.\n");
        return;
    }

    SolPublicKey *wallet_pubkey = get_public_key(wallet);
    if (wallet_pubkey == NULL)
    {
        printf("Failed to get wallet public key.\n");
        return;
    }

    TokenList *tokens = get_all_tokens(client, wallet_pubkey);
    if (!tokens)
    {
        printf("Failed to fetch tokens.\n");
        return;
    }

    uintptr_t len = token_list_get_len(tokens);
    TokenInfo *data = token_list_get_data(tokens);

    printf("Total Tokens: %lu\n", len);
    for (uintptr_t i = 0; i < len; i++)
    {
        printf("Token Mint: %s, Balance: %s, Owner: %s\n",
               data[i].mint, data[i].balance, data[i].owner);
    }

    free_token_list(tokens);
    printf("=== End Test: Get All Tokens ===\n");
}

void test()
{
    test_create_and_save_wallet(file_path_recipient2);
    test_load_wallet_from_file(file_path);

    test_create_and_save_mint_wallet();

    test_create_and_save_recipient_wallet();

    SolClient *client = test_sol_client_new(devnet_url);

    test_sol_airdrop();

    test_create_spl_token();

    test_mint_spl_token();
    test_mint_spl_token();

    test_transfer_spl_token();

    test_transfer_sol();

    test_get_all_tokens();
}

int main()
{
    // test();
    const char *rpc_url = "https://api.devnet.solana.com";
    const char *payer_path = file_path;
    const char *program_id = "DsfPR2teuRS9ABmqGqq5NobD8Y9A9KvzMVNVzsjSP8Dy";

    SolClient *client = new_sol_client(rpc_url);
    SolKeyPair *payer = load_wallet_from_file(payer_path);
    SolKeyPair *account = new_keypair();

    SolPublicKey SYSTEM_PROGRAM_ID = get_system_program_id();

    const char *initialize_method = "initialize";

    SolPublicKey initialize_accounts[3];
    initialize_accounts[0] = account->pubkey;
    initialize_accounts[1] = payer->pubkey;
    initialize_accounts[2] = SYSTEM_PROGRAM_ID;
    SolKeyPair *initialize_signers[2];
    initialize_signers[0] = payer;
    initialize_signers[1] = account;

    char *initialize_result = send_generic_transaction_c(
        client,
        program_id,
        initialize_method,
        initialize_accounts,
        3,
        initialize_signers,
        2,
        NULL,
        0);

    printf("initialize Result: %s\n", initialize_result);

    // Step 5: Call Increment on the initialized account
    const char *increment_method = "increment";

    SolPublicKey increment_accounts[2];
    increment_accounts[0] = account->pubkey;
    increment_accounts[1] = payer->pubkey;

    SolKeyPair *increment_signers[1];
    increment_signers[0] = payer;

    char *increment_result = send_generic_transaction_c(
        client,
        program_id,
        increment_method,
        increment_accounts,
        2,
        increment_signers,
        1,
        NULL,
        0);

    printf("Increment Result: %s\n", increment_result);

    uint8_t account_data[256]; // Buffer to hold account data
    size_t data_offset = 8;    // Skip discriminator (for Anchor programs)
    size_t bytes_copied = get_account_data_c(client, &account->pubkey, account_data, sizeof(account_data), data_offset);

    if (bytes_copied > 0)
    {
        printf("✅ Data Copied: %zu bytes\n", bytes_copied);
        // Example: Deserialize the first 8 bytes into u64
        uint64_t *counter = (uint64_t *)account_data;
        printf("🔢 Counter Value: %lu\n", *counter);
    }
    else
    {
        printf("❌ Failed to fetch account data.\n");
    }

    increment_result = send_generic_transaction_c(
        client,
        program_id,
        increment_method,
        increment_accounts,
        2,
        increment_signers,
        1,
        NULL,
        0);

    printf("Increment Result: %s\n", increment_result);

    account_data[256]; // Buffer to hold account data
    data_offset = 8;   // Skip discriminator (for Anchor programs)
    bytes_copied = get_account_data_c(client, &account->pubkey, account_data, sizeof(account_data), data_offset);

    if (bytes_copied > 0)
    {
        printf("✅ Data Copied: %zu bytes\n", bytes_copied);
        // Example: Deserialize the first 8 bytes into u64
        uint64_t *counter = (uint64_t *)account_data;
        printf("🔢 Counter Value: %lu\n", *counter);
    }
    else
    {
        printf("❌ Failed to fetch account data.\n");
    }

    return 0;
}