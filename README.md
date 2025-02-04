# Solana SDK for C Integration

This project provides a Rust-based SDK for integrating Solana blockchain functionality with C programs. The SDK enables developers to perform tasks such as creating wallets, transferring SOL and SPL tokens, interacting with smart contracts, and managing accounts on the Solana blockchain.

## Features

- **Wallet Management**: Create, load, and manage Solana wallets.
- **Token Management**: Transfer SOL and SPL tokens, mint new SPL tokens, and fetch token balances.
- **Smart Contract Integration**: Interact with smart contracts and send custom transactions.
- **Account Operations**: Fetch and manage account data.

## Prerequisites

- **Rust**: Ensure you have the latest version of Rust installed. [Install Rust](https://www.rust-lang.org/tools/install)
- **Solana CLI**: Install the Solana CLI for blockchain interaction. [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- **C Compiler**: Ensure you have a C compiler installed (e.g., `gcc` or `clang`).

## Building the Project

1. Clone the repository:
   ```bash
   git clone https://github.com/VAR-META-Tech/solana-c-sdk.git
   cd solana-c-sdk
   ```

2. Build the project:
   ```bash
   sh build.sh
   ```
   
## Test the Project
   ```bash
   sh test.sh
   ```

## Generate smart contract interface from IDL file

1. Place the IDL file in source folder (ex: anchor_counter.json)
   ```bash
   sh generate.sh anchor_counter.json
   ```

2. Locate the Generated Interface File:
   ```bash
   header/anchor_counter_interface.c
   ```

## Using the SDK in C

### Include the Generated Header File

Include the `solana_sdk.h` header file in your C project to access the SDK functions.

```c
#include "solana_sdk.h"
```

### Example: Creating and Loading a Wallet

```c
#include "solana_sdk.h"
#include <stdio.h>

int main() {
    // Create a new wallet
    SolKeyPair *wallet = new_keypair();
    if (wallet == NULL) {
        printf("Failed to create a wallet\n");
        return 1;
    }

    // Get wallet address
    char *address = get_wallet_address(wallet);
    printf("Wallet Address: %s\n", address);

    // Free allocated memory
    free(wallet);
    free(address);
    return 0;
}
```

### Example: Transferring SOL

```c
#include "solana_sdk.h"
#include <stdio.h>

int main() {
    // Initialize Solana client
    SolClient *client = new_sol_client("https://api.devnet.solana.com");
    if (client == NULL) {
        printf("Failed to initialize Solana client\n");
        return 1;
    }

    // Create sender wallet
    SolKeyPair *sender = new_keypair();
    char *sender_address = get_wallet_address(sender);
    printf("Sender Address: %s\n", sender_address);

    // Generate recipient public key
    SolPublicKey recipient;
    // Set recipient public key bytes (replace with actual public key)
    memset(recipient.data, 0, sizeof(recipient.data));

    // Transfer 1 SOL
    uint64_t lamports = 1000000000; // 1 SOL in lamports
    bool success = transfer_sol(client, sender, &recipient, lamports);
    if (success) {
        printf("Transfer successful\n");
    } else {
        printf("Transfer failed\n");
    }

    // Free allocated resources
    free_client(client);
    free(sender);
    free(sender_address);

    return 0;
}
```
### Example: Using generated interface from IDL

```c
void test_counter()
{
    // Try run test anchor program https://github.com/thanhngoc541/anchor-counter, deploy the project to get program id
    // RPC URL and paths
    const char *rpc_url = "https://api.devnet.solana.com";
    const char *payer_path = file_path;
    const char *program_id = "3CkKwWzHTvwnAURu8TD4JijeuYZkaPkU14QRGeGLHbSw";

    // Initialize Solana client and wallet
    SolClient *client = new_sol_client(rpc_url);
    SolKeyPair *payer = load_wallet_from_file(payer_path);
    SolKeyPair *account = new_keypair();

    // Get system program ID
    SolPublicKey SYSTEM_PROGRAM_ID = get_system_program_id();

    // Prepare to initialize account
    SolPublicKey initialize_accounts[3] = {
        account->pubkey,
        payer->pubkey,
        SYSTEM_PROGRAM_ID};

    SolKeyPair *initialize_signers[2] = {payer, account};

    // Call initialize
    char *initialize_result = anchor_counter_initialize_c(
        client,
        program_id,
        initialize_accounts,
        3,
        initialize_signers,
        2);

    if (initialize_result != NULL)
    {
        printf("Initialize Result: %s\n", initialize_result);
        free(initialize_result);
    }
    else
    {
        printf("❌ Failed to initialize account.\n");
    }

    // Call increment method
    SolPublicKey increment_accounts[2] = {
        account->pubkey,
        payer->pubkey};

    SolKeyPair *increment_signers[1] = {payer};

    for (int i = 0; i < 2; i++)
    {
        char *increment_result = anchor_counter_increment_c(
            client,
            program_id,
            increment_accounts,
            2,
            increment_signers,
            1);

        if (increment_result != NULL)
        {
            printf("Increment Result: %s\n", increment_result);
            free(increment_result);
        }
        else
        {
            printf("❌ Failed to increment account.\n");
        }

        // Fetch and print the updated account value
        uint8_t account_data[512]; // Larger buffer to handle future expansion
        size_t data_offset = 8;
        size_t bytes_copied = get_account_data_c(
            client,
            &account->pubkey,
            account_data,
            sizeof(account_data),
            data_offset);

        if (bytes_copied > 0)
        {
            printf("✅ Data Copied: %zu bytes\n", bytes_copied);
            uint64_t *counter = (uint64_t *)account_data;
            printf("🔢 Counter Value: %lu\n", *counter);
        }
        else
        {
            printf("❌ Failed to fetch account data.\n");
        }
    }

    // Clean up resources
    free_client(client);
    free_payer(payer);
}
```

## Documentation

### Public Functions

#### Client Management

- **`SolClient *new_sol_client(const char *url);`**
  
  Initializes a Solana client for the given RPC URL.

- **`void free_client(SolClient *client);`**
  
  Frees the memory allocated for the client.

#### Wallet Management

- **`SolKeyPair *new_keypair();`**
  
  Creates a new wallet (keypair).

- **`SolKeyPair *load_wallet_from_file(const char *file_path);`**
  
  Loads a wallet from a file.

- **`char *get_wallet_address(SolKeyPair *wallet);`**
  
  Retrieves the wallet's public address as a string.

- **`SolPublicKey *get_public_key(SolKeyPair *wallet);`**
  
  Retrieves the wallet's public key.

- **`SolSecretKey *get_secret_key(SolKeyPair *wallet);`**
  
  Retrieves the wallet's private key.

- **`struct SolPublicKey *get_pubkey_from_address(const char *address);`**
  
  Converts an address string to a public key.

- **`char *get_address_from_pubkey(const struct SolPublicKey *pubkey);`**
  
  Converts a public key to an address string.

- **`void free_payer(SolKeyPair *payer);`**
  
  Frees the memory allocated for the wallet.

#### Token Operations

- **`bool transfer_sol(SolClient *client, SolKeyPair *sender, SolPublicKey *recipient, uint64_t lamports);`**
  
  Transfers SOL from the sender to the recipient.

- **`bool transfer_spl(SolClient *client, SolKeyPair *sender, SolPublicKey *recipient, SolPublicKey *mint, uint64_t amount);`**
  
  Transfers SPL tokens from the sender to the recipient.

- **`uint64_t get_associated_token_balance(SolClient *client, SolPublicKey *owner, SolPublicKey *mint);`**
  
  Retrieves the balance of an associated token account.

- **`bool create_spl_token(SolClient *client, SolKeyPair *payer, SolKeyPair *mint);`**
  
  Creates a new SPL token.

- **`bool mint_spl(SolClient *client, SolKeyPair *payer, SolKeyPair *mint_authority, SolPublicKey *recipient, uint64_t amount);`**
  
  Mints new SPL tokens.

- **`struct SolPublicKey *get_or_create_associated_token_account(SolClient *client, SolKeyPair *payer, SolPublicKey *owner, SolKeyPair *mint);`**
  
  Gets or creates an associated token account for the owner and mint.

#### Account Operations

- **`uintptr_t get_account_data_c(struct SolClient *client, struct SolPublicKey *account_pubkey, uint8_t *data_ptr, uintptr_t data_len, uintptr_t data_offset);`**
  
  Fetches account data and copies it into a provided buffer.

- **`struct SolMint *get_mint_info(struct SolClient *client, struct SolPublicKey *mint_pubkey);`**
  
  Retrieves information about an SPL token mint.

#### Smart Contract Interaction

- **`char *send_generic_transaction_c(SolClient *client, const char *program_id, const char *method_name, const SolPublicKey *account_pubkeys, uintptr_t account_count, SolKeyPair *const *signers, uintptr_t signer_count, const uint8_t *data_ptr, uintptr_t data_len);`**
  
  Sends a generic transaction to a smart contract.

- **`void initialize_account_c(SolClient *client, SolKeyPair *payer, SolKeyPair *account, const char *program_id);`**
  
  Initializes an account for a program.

## Unreal Plugin
[UnrealSolSDK](https://github.com/VAR-META-Tech/UnrealSolanaSDK)

---

# 🚀 Solana Performance Benchmark: Rust (FFI via C) vs. C# (Solnet)

This repository benchmarks **Solana wallet operations** using **Rust (FFI via C) and C# (Solnet)** to compare performance and efficiency.

---

## 📌 **Overview**
| **Language** | **Description** |
|-------------|----------------|
| 🔗 **Rust (FFI via C)** | Uses C to call Rust functions via Foreign Function Interface (FFI). |
| 🎯 **C# (Solnet)** | Uses the **Solnet** .NET SDK to interact with the Solana blockchain. |

---

## 📊 **Performance Comparison (Execution Time in ms)**

| **Function**                      | **Rust (FFI via C)** | **C# (Solnet)**  |
|-----------------------------------|----------------|------------------|
| Wallet Creation                   | **0.05 - 1.7 ms**  | 50 - 80 ms        |
| Wallet Loading                     | **0.05 - 1.7 ms**  | 15 - 20 ms          |
| Airdrop Request                    | 3000 ms  (The function waits for confirmation)  |       59ms (The function does not wait for confirmation)  |
| Mint SPL Token                     | **~300 ms** | N/A             |
| Transfer SPL Token                 | **~280 ms** | N/A             |
| Transfer SOL                        | **230 - 280 ms**  | 300 - 500 ms      |

---

## 📌 **Observations**
1. **Rust (FFI via C) is significantly faster than C# (Solnet)** ⚡  
   - **Faster** in wallet operations (**creation, loading**).
   - **Lower overhead** than Solnet's **managed memory** approach.

2. **C# (Solnet) is easier for .NET developers** 🎯  
   - **High-level API** for easy Solana integration.
   - **Slower due to garbage collection** and runtime overhead.

3. **Network-bound operations (Airdrops, SPL Token Minting, Transfers) take a similar amount of time, as execution time is measured from the same starting point** 🌍  
   - **These operations rely on blockchain processing time**, meaning the execution speed is mostly determined by **Solana network conditions and validator processing**.  
   - **Language choice affects only transaction preparation** (e.g., constructing, signing, and submitting transactions), but the actual confirmation time depends on Solana.  
   - **If the network is congested, all languages will experience delays equally**.  

---


## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## Acknowledgments

This SDK uses the following crates:
- `solana-client`
- `solana-sdk`
- `spl-token`
- `borsh`
- `serde`

For detailed usage examples and advanced functionality, refer to the source code and the Solana documentation.

