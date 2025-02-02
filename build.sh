#!/bin/bash

# Define the file path
file_path="pkg/modulee_engine_wasm_bg.js"

# Define the import statement
import_statement='import {
    TextDecoder,
    TextEncoder,
} from "./text-encoder-and-decoder-polyfill.js";

'

# Create a temporary file with the import statement and the original content
{
    echo "$import_statement"
    cat "$file_path"
} >"${file_path}.tmp"

# Replace the original file with the temporary file
mv "${file_path}.tmp" "$file_path"
