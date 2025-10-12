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

fn get_cargo_version() -> Option<String> {
    let cargo_toml = fs::read_to_string("Cargo.toml").ok()?;
    // Find the [package] section first, then look for a version = "..." line inside it
    if let Some(pkg_start) = cargo_toml.find("[package]") {
        let rest = &cargo_toml[pkg_start..];
        for line in rest.lines().skip(1) {
            let l = line.trim();
            if l.starts_with('[') {
                // reached next section
                break;
            }
            if l.starts_with("version") {
                if let Some(eq_pos) = l.find('=') {
                    let val = l[eq_pos + 1..].trim();
                    // val is expected to be a quoted string
                    if let Some(first_quote) = val.find('"') {
                        let rest_val = &val[first_quote + 1..];
                        if let Some(second_quote) = rest_val.find('"') {
                            return Some(rest_val[..second_quote].to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

fn replace_version_in_json(content: &str, version: &str) -> String {
    // Find the "\"version\"" field and replace its string value.
    if let Some(field_pos) = content.find("\"version\"") {
        if let Some(colon_rel) = content[field_pos..].find(':') {
            let after_colon = field_pos + colon_rel + 1;
            if let Some(first_quote_rel) = content[after_colon..].find('"') {
                let start = after_colon + first_quote_rel + 1;
                if let Some(end_quote_rel) = content[start..].find('"') {
                    let end = start + end_quote_rel;
                    let mut s = content.to_string();
                    s.replace_range(start..end, version);
                    return s;
                }
            }
        }
    }
    content.to_string()
}

fn main() {
    std::env::set_var("RUSTFLAGS", "--cfg getrandom_backend=\"wasm_js\"");

    let pkg_dir = Path::new("pkg");

    Command::new("wasm-pack")
        .arg("build")
        .output()
        .expect("Error running wasm-pack");

    add_polyfill_imports(pkg_dir);

    // Copy package.template.json to package.json, replacing the version with Cargo.toml's version
    let package_template = pkg_dir.join("package.template.json");
    let package_json = pkg_dir.join("package.json");

    let template_content =
        fs::read_to_string(&package_template).expect("Failed to read pkg/package.template.json");

    let final_content = if let Some(version) = get_cargo_version() {
        replace_version_in_json(&template_content, &version)
    } else {
        // If we couldn't read Cargo.toml, fall back to the template as-is
        template_content
    };

    fs::write(&package_json, final_content).expect("Failed to write pkg/package.json");
}
