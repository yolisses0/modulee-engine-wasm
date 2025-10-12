use std::fs;
use std::path::Path;
use std::process::Command;

fn add_polyfill_imports(pkg_dir: &Path) {
    let file_path = pkg_dir.join("modulee_engine_wasm_bg.js");
    let file_content =
        fs::read_to_string(&file_path).expect("Failed to read modulee_engine_wasm_bg.js");
    let import_statement = r#"import {
    TextDecoder,
    TextEncoder,
} from "./text_encoder_and_decoder_polyfill.js";
import "./getRandomValues_polyfill.js";

"#;
    let new_file_content = format!("{}{}", import_statement, file_content);
    fs::write(&file_path, new_file_content).expect("Failed to write to modulee_engine_wasm_bg.js");
}

fn main() {
    std::env::set_var("RUSTFLAGS", "--cfg getrandom_backend=\"wasm_js\"");

    let pkg_dir = Path::new("pkg");

    Command::new("wasm-pack")
        .arg("build")
        .output()
        .expect("Error running wasm-pack");

    add_polyfill_imports(pkg_dir);

    // Copy package.template.json to package.json
    let package_template = pkg_dir.join("package.template.json");
    let package_json = pkg_dir.join("package.json");
    fs::copy(package_template, package_json)
        .expect("Failed to copy package.template.json to package.json");
}
