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
   git clone https://github.com/VAR-META-Tech/solana-c-sdk
   cd solana-c-sdk
   ```

2. Build the project:
   ```bash
   sh build.sh
   ```

3. Locate the generated shared library (`.so`, `.dll`, or `.dylib`):
   ```bash
   target/release/libsolana_sdk.so
   ```
4. Locate the Generated Header File (`.h`):
   ```bash
   header/solana_sdk.h
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

## Documentation

### Public Functions

#### Client Management
- `SolClient *new_sol_client(const char *url);`
  Initializes a Solana client for the given RPC URL.

- `void free_client(SolClient *client);`
  Frees the memory allocated for the client.

#### Wallet Management
- `SolKeyPair *new_keypair();`
  Creates a new wallet (keypair).

- `SolKeyPair *load_wallet_from_file(const char *file_path);`
  Loads a wallet from a file.

- `char *get_wallet_address(SolKeyPair *wallet);`
  Retrieves the wallet's public address as a string.

- `void free_payer(SolKeyPair *payer);`
  Frees the memory allocated for the wallet.

#### Token Operations
- `bool transfer_sol(SolClient *client, SolKeyPair *sender, SolPublicKey *recipient, uint64_t lamports);`
  Transfers SOL from the sender to the recipient.

- `bool transfer_spl(SolClient *client, SolKeyPair *sender, SolPublicKey *recipient, SolPublicKey *mint, uint64_t amount);`
  Transfers SPL tokens from the sender to the recipient.

- `uint64_t get_associated_token_balance(SolClient *client, SolPublicKey *owner, SolPublicKey *mint);`
  Retrieves the balance of an associated token account.

#### Smart Contract Interaction
- `char *send_generic_transaction_c(SolClient *client, const char *program_id, const char *method_name, const SolPublicKey *account_pubkeys, uintptr_t account_count, SolKeyPair *const *signers, uintptr_t signer_count, const uint8_t *data_ptr, uintptr_t data_len);`
  Sends a generic transaction to a smart contract.

- `void initialize_account_c(SolClient *client, SolKeyPair *payer, SolKeyPair *account, const char *program_id);`
  Initializes an account for a program.

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

