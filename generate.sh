#!/bin/bash

# Default JSON file (fallback)
DEFAULT_JSON="idl.json"

# Check if JSON file is passed as an argument
if [ $# -eq 0 ]; then
    echo "⚠️ No JSON file provided. Using default: $DEFAULT_JSON"
    JSON_FILE=$DEFAULT_JSON
else
    JSON_FILE=$1  # Use the provided argument
fi

# Set variables
SOURCE_FILE="generate_c_interface.c"
OUTPUT_FILE="generate_c_interface"

# Compile the C program
gcc $SOURCE_FILE -o $OUTPUT_FILE -ljansson

# Check if compilation was successful
if [ $? -eq 0 ]; then
    echo "✅ Compilation successful. Running program with $JSON_FILE ..."
    ./$OUTPUT_FILE $JSON_FILE
else
    echo "❌ Compilation failed."
    exit 1
fi
