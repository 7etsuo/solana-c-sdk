#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <jansson.h>

// Utility to read a file into a string
char *read_file(const char *file_path)
{
    FILE *file = fopen(file_path, "rb");
    if (!file)
    {
        perror("Failed to open IDL file");
        exit(1);
    }
    fseek(file, 0, SEEK_END);
    long len = ftell(file);
    fseek(file, 0, SEEK_SET);

    char *data = malloc(len + 1);
    fread(data, 1, len, file);
    fclose(file);
    data[len] = '\0';
    return data;
}

// Generate dynamic functions from IDL and write to file
void generate_function(json_t *idl, const char *program_name, FILE *output_file)
{
    json_t *instructions = json_object_get(idl, "instructions");
    size_t index;

    json_t *instr;
    json_array_foreach(instructions, index, instr)
    {
        const char *instr_name = json_string_value(json_object_get(instr, "name"));
        json_t *accounts = json_object_get(instr, "accounts");

        fprintf(output_file, "// Function to call '%s' dynamically\n", instr_name);
        fprintf(output_file,
                "char *%s_%s_c(SolClient *client, const char *program_id, SolPublicKey *accounts, size_t account_count, SolKeyPair **signers, size_t signer_count) {\n",
                program_name, instr_name);

        // Start the body of the function
        fprintf(output_file, "    return send_generic_transaction_c(\n");
        fprintf(output_file, "        client,\n");
        fprintf(output_file, "        program_id,\n");
        fprintf(output_file, "        \"%s\",\n", instr_name);
        fprintf(output_file, "        accounts,\n");
        fprintf(output_file, "        account_count,\n");
        fprintf(output_file, "        signers,\n");
        fprintf(output_file, "        signer_count,\n");
        fprintf(output_file, "        NULL,\n");
        fprintf(output_file, "        0);\n");
        fprintf(output_file, "}\n\n");
    }
}

// Main function
int main(int argc, char *argv[])
{
    if (argc != 2)
    {
        fprintf(stderr, "Usage: %s <idl.json>\n", argv[0]);
        return 1;
    }

    // Read the IDL JSON file
    char *idl_data = read_file(argv[1]);
    json_error_t error;
    json_t *idl = json_loads(idl_data, 0, &error);
    free(idl_data);

    if (!idl)
    {
        fprintf(stderr, "Error parsing JSON: %s\n", error.text);
        return 1;
    }

    // Extract metadata for program name
    json_t *metadata = json_object_get(idl, "metadata");
    const char *program_name = json_string_value(json_object_get(metadata, "name"));

    if (!program_name)
    {
        fprintf(stderr, "Failed to get program name from metadata.\n");
        json_decref(idl);
        return 1;
    }

    // Generate output file
    char output_filename[256];
    snprintf(output_filename, sizeof(output_filename), "header/%s_interface.c", program_name);
    FILE *output_file = fopen(output_filename, "w");

    if (!output_file)
    {
        perror("Failed to create output file");
        json_decref(idl);
        return 1;
    }

    // Write file header
    fprintf(output_file, "// Auto-generated C interface for Solana Program: %s\n\n", program_name);
    fprintf(output_file, "#include <stdio.h>\n#include <stdint.h>\n#include <stdlib.h>\n#include \"solana_sdk.h\"\n\n");

    // Generate dynamic function signatures
    generate_function(idl, program_name, output_file);

    fclose(output_file);
    json_decref(idl);

    printf("✅ C interface generated: %s\n", output_filename);
    return 0;
}
