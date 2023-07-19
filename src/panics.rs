use std::path::Path;

pub fn check_folder(folder: &Path) {
    if !folder.exists() {
        eprintln!("{} doesn't exist", folder.display());
        std::process::exit(1);
    }

    if !folder.is_dir() {
        eprintln!("{} isn't a directory", folder.display());
        std::process::exit(1);
    }
}
