#include <stdio.h>
#include <stdlib.h>
#include "header/solana_lib.h"

const char *file_path = "wallet_keypair.json";
const char *file_path_payer = "wallet_keypair.json";
const char *file_path_mint = "wallet_keypair_mint.json";
const char *devnet_url = "https://api.devnet.solana.com";

void test_create_and_save_wallet(const char *file_path)
{
    SolKeyPair *wallet = create_and_save_wallet(file_path);
    SolKeyPair *payer = create_and_save_wallet(file_path_payer);
    SolClient *mint = create_and_save_wallet(file_path_mint);
    // Check if the all wallet creation succeeded
    if (wallet != NULL && payer != NULL && mint != NULL)
    {
        // Print the wallet address
        printf("Solana Wallet Address: %s\n", get_wallet_address(wallet));
        printf("Solana payer Wallet Address: %s\n", get_wallet_address(payer));
        printf("Solana mint Wallet Address: %s\n", get_wallet_address(mint));
    }
    else
    {
        printf("Failed to create wallet.\n");
    }
}

void test_create_and_save_mint_wallet()
{
    SolClient *mint = create_and_save_wallet(file_path_mint);
    // Check if the all wallet creation succeeded
    if (mint != NULL)
    {
        // Print the wallet address
        printf("Solana mint Wallet Address: %s\n", get_wallet_address(mint));
    }
    else
    {
        printf("Failed to create wallet.\n");
    }
}

void test_load_wallet_from_file(const char *file_path)
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
}

void test_sol_client_new(const char *url)
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
    SolClient *client = new_sol_client(devnet_url);
    if (client != NULL)
    {
        SolKeyPair *payer = load_wallet_from_file(file_path_payer);
        SolClient *mint = load_wallet_from_file(file_path_mint);

        if (payer != NULL && mint != NULL)
        {
            SolKeyPair *recipient = load_wallet_from_file("wallet_keypair_recipient.json");
            if (recipient != NULL)
            {
                printf("Solana recipient Wallet Address: %s\n", get_wallet_address(recipient));
                uint64_t amount = 1000000000000;
                bool success = mint_spl(client, payer, mint, get_public_key(recipient), amount);
                // get balance
                uint64_t balance = get_associated_token_balance(client, get_public_key(recipient), mint);
                if (success)
                {
                    printf("SPL Token minted successfully.\n");
                    printf("Recipient Balance: %lu\n", balance);
                }
                else
                {
                    printf("Failed to mint SPL Token.\n");
                }
            }
            else
            {
                printf("Failed to create recipient wallet.\n");
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

int main()
{
    // Create and save the wallet
    // test_create_and_save_wallet(file_path);

    test_create_and_save_mint_wallet();
    // Load and verify the wallet
    test_load_wallet_from_file(file_path);

    test_sol_client_new(devnet_url);

    test_sol_airdrop();

    test_create_spl_token();

    test_mint_spl_token();
    test_mint_spl_token();

    return 0;
}