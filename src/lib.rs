use std::collections::HashSet;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::fs;
const IMG_PATH: &str = "assets/72x72/";

lazy_static! {
    static ref FILE_NAMES: HashSet<String> = HashSet::from_iter(
        fs::read_dir(IMG_PATH).unwrap()
            .into_iter()
            .map(|path| path.unwrap().file_name().into_string().unwrap())
    );
}

pub fn get_twemoji(text: &str) -> Option<String> {
    let fname = text.chars().into_iter()
        .map(|c| format!("{:x?}", c as u32))
        .join("-")+".png";
    if FILE_NAMES.contains(&fname) {
        Some(format!("{}{}", IMG_PATH, fname))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::get_twemoji;

    #[test]
    fn it_works() {
        println!("{:?}", get_twemoji("💻"))
    }
}
