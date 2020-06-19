extern crate walkdir;
extern crate rand;

use walkdir::WalkDir;
use walkdir::DirEntry;

use std::fs::metadata;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let walker = WalkDir::new("/home/test").into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
        let md = entry.metadata().unwrap();
        if md.is_dir() {
            continue
        } else {
            println!("Encrypting: {}", entry.path().display());
            println!("With key: {}", gen_random_string());
        }
    }
}

fn gen_random_string() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
    rand_string
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
