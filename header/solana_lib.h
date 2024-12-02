#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct SolClient SolClient;

typedef struct SolKeyPair SolKeyPair;

typedef struct SolPublicKey {
  uint8_t data[32];
} SolPublicKey;

typedef struct SolSecretKey {
  uint8_t data[64];
} SolSecretKey;

typedef struct SolMint {
  struct SolPublicKey *mint_authority;
  uint64_t supply;
  uint8_t decimals;
  bool is_initialized;
  struct SolPublicKey *freeze_authority;
} SolMint;

struct SolPublicKey *get_public_key(struct SolKeyPair *wallet);

struct SolSecretKey *get_secret_key(struct SolKeyPair *wallet);

char *get_wallet_address(struct SolKeyPair *wallet);

struct SolKeyPair *create_and_save_wallet(const char *file_path);

struct SolKeyPair *load_wallet_from_file(const char *file_path);

struct SolClient *new_sol_client(const char *url);

uint64_t get_balance(struct SolClient *client, struct SolPublicKey *pubkey);

bool request_airdrop(struct SolClient *client, struct SolPublicKey *pubkey, uint64_t lamports);

bool transfer_spl(struct SolClient *client,
                  struct SolKeyPair *signer_wallet,
                  struct SolPublicKey *sender,
                  struct SolPublicKey *recipient,
                  struct SolKeyPair *mint,
                  uint64_t amount);

bool create_spl_token(struct SolClient *client, struct SolKeyPair *payer, struct SolKeyPair *mint);

struct SolMint *get_mint_info(struct SolClient *client, struct SolKeyPair *mint);

struct SolPublicKey *get_or_create_associated_token_account(struct SolClient *client,
                                                            struct SolKeyPair *payer,
                                                            struct SolPublicKey *owner,
                                                            struct SolKeyPair *mint);

bool mint_spl(struct SolClient *client,
              struct SolKeyPair *signer_wallet,
              struct SolKeyPair *mint_authority,
              struct SolPublicKey *recipient,
              uint64_t amount);

uint64_t get_associated_token_balance(struct SolClient *client,
                                      struct SolPublicKey *owner,
                                      struct SolKeyPair *mint);
