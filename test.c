#include <stdio.h>
#include <stdlib.h>
#include "tests/sol_lib.h"

void test_create_and_save_wallet(const char *file_path)
{
    char *public_key = create_and_save_wallet(file_path);
    // Check if the wallet creation succeeded
    if (public_key != NULL)
    {
        // Print the public key
        printf("Solana Wallet Public Key: %s\n", public_key);
        // Free the public key string to avoid memory leaks
        free_string(public_key);
    }
    else
    {
        printf("Failed to create wallet.\n");
    }
}

void test_load_wallet_from_file(const char *file_path)
{
    char *public_key = load_wallet_from_file(file_path);
    // Check if the wallet loading succeeded
    if (public_key != NULL)
    {
        // Print the loaded public key
        printf("Loaded Solana Wallet Public Key: %s\n", public_key);
        // Free the public key string to avoid memory leaks
        free_string(public_key);
    }
    else
    {
        printf("Failed to load wallet.\n");
    }
}

int main()
{
    const char *file_path = "wallet_keypair.json";

    // Create and save the wallet
    // test_create_and_save_wallet(file_path);

    // Load and verify the wallet
    test_load_wallet_from_file(file_path);

    return 0;
}