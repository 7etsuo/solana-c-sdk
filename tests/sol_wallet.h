#ifndef SOL_WALLET_H
#define SOL_WALLET_H
#include <stdint.h>
#include <stdio.h>
#include <stddef.h>
#include <inttypes.h>

#ifdef __cplusplus
extern "C"
{
#endif

    extern char *create_and_save_wallet(const char *file_path);
    extern char *load_wallet_from_file(const char *file_path);
    extern void free_string(char *s);

#ifdef __cplusplus
}
#endif

#endif // SOL_WALLET_H