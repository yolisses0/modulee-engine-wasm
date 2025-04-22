#!/bin/bash
export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
wasm-pack build $1 &&
    (
        cd pkg

        # Define the file path
        file_path="modulee_engine_wasm_bg.js"

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

        # Replace the content of package.json with package.json.template
        cp package.json.template package.json
    )
