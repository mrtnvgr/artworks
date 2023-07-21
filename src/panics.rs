use std::{
    path::{Path, PathBuf},
    process::exit,
};

#[allow(clippy::indexing_slicing)]
pub fn get_folder() -> PathBuf {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        exit(1);
    }

    PathBuf::from(&args[1])
}

pub fn check_folder(folder: &Path) {
    if !folder.exists() {
        eprintln!("{} doesn't exist", folder.display());
        exit(1);
    }

    if !folder.is_dir() {
        eprintln!("{} isn't a directory", folder.display());
        exit(1);
    }
}
