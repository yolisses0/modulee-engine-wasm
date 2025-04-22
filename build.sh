#!/bin/bash
export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
wasm-pack build $1 &&
    (
        # Define the file path
        file_path="pkg/modulee_engine_wasm_bg.js"

        # Define the import statement
        import_statement='import {
    TextDecoder,
    TextEncoder,
} from "./text_encoder_and_decoder_polyfill.js";
import "./getRandomValues_polyfill.js";

'

        # Create a temporary file with the import statement and the original content
        {
            echo "$import_statement"
            cat "$file_path"
        } >"${file_path}.tmp"

        # Replace the original file with the temporary file
        mv "${file_path}.tmp" "$file_path"

        # Update package.json to include the polyfill files
        package_json="pkg/package.json"
        jq '.files += ["getRandomValues_polyfill.js", "text_encoder_and_decoder_polyfill.js"]' "$package_json" >"${package_json}.tmp" &&
            mv "${package_json}.tmp" "$package_json"
    )
