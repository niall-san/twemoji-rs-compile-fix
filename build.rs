use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::fs::File;

fn sanitize_identifier(identifier: &str) -> String {
    let mut sanitized = "ASSET_".to_owned(); // Added prefix to ensure it starts with a letter
    for ch in identifier.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            sanitized.push(ch);
        } else {
            sanitized.push('_');
        }
    }
    sanitized
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("assets.rs");
    let mut f = File::create(&dest_path).unwrap();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let assets_path = Path::new(&manifest_dir).join("assets").join("72x72");

    let mut filenames: Vec<String> = Vec::new();

    for entry in fs::read_dir(assets_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let const_name = sanitize_identifier(file_name);
            writeln!(f, "static {}: &'static [u8] = include_bytes!(\"{}\");", const_name, path.to_str().unwrap()).unwrap();
            filenames.push(file_name.to_owned());
        }
    }

    // Generate code for the static array of filenames
    writeln!(f, "pub static GENERATED_FILE_NAMES: [&'static str; {}] = {:?};", filenames.len(), filenames).unwrap();
}
