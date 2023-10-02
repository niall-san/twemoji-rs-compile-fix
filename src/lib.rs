use std::collections::HashSet;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::path::{Path, PathBuf};

include!(concat!(env!("OUT_DIR"), "/assets.rs"));

lazy_static! {
    static ref ASSETS_PATH: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets").join("72x72");

    // Populate the HashSet from the renamed static array generated in build.rs
    static ref FILE_NAMES: HashSet<&'static str> = HashSet::from_iter(GENERATED_FILE_NAMES.iter().cloned());
}

pub fn get_twemoji(text: &str) -> Option<PathBuf> {
    let fname = text.chars().into_iter()
        .map(|c| format!("{:x?}", c as u32))
        .join("-")+".png";
    if FILE_NAMES.contains(&fname.as_str()) {
        Some(ASSETS_PATH.join(&fname))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::{path::{PathBuf}, fs::File};
    use crate::get_twemoji;

    fn print_call(text: &str) -> Option<PathBuf> {
        let opt_path: Option<PathBuf> = get_twemoji("💻");
        println!("{} -> {:?}", text, opt_path);
        opt_path
    }

    #[test]
    fn it_works() {
        let opt_path: Option<PathBuf> = print_call("💻");
        assert!(opt_path.is_some());
        assert!(File::open(opt_path.unwrap()).is_ok());
    }
}
