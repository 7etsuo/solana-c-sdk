// Auto-generated C interface for Solana Program: counter

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include "solana_sdk.h"

// Function to call 'decrement' dynamically
char *counter_decrement_c(SolClient *client, const char *program_id, SolPublicKey *accounts, size_t account_count, SolKeyPair **signers, size_t signer_count) {
    return send_generic_transaction_c(
        client,
        program_id,
        "decrement",
        accounts,
        account_count,
        signers,
        signer_count,
        NULL,
        0);
}

// Function to call 'increment' dynamically
char *counter_increment_c(SolClient *client, const char *program_id, SolPublicKey *accounts, size_t account_count, SolKeyPair **signers, size_t signer_count) {
    return send_generic_transaction_c(
        client,
        program_id,
        "increment",
        accounts,
        account_count,
        signers,
        signer_count,
        NULL,
        0);
}

// Function to call 'initialize' dynamically
char *counter_initialize_c(SolClient *client, const char *program_id, SolPublicKey *accounts, size_t account_count, SolKeyPair **signers, size_t signer_count) {
    return send_generic_transaction_c(
        client,
        program_id,
        "initialize",
        accounts,
        account_count,
        signers,
        signer_count,
        NULL,
        0);
}

