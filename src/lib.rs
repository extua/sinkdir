use std::fs;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub fn copy(source: &str, target: &str) {
    let source_path: PathBuf = Path::new(source).to_path_buf();
    let target_path: PathBuf = Path::new(target).to_path_buf();

    if target_path.is_dir() {
        if source_path.is_dir() {
            // Read through the source directory and recursively
            // run all paths back through this copy function
            for entry in fs::read_dir(source_path).unwrap() {
                let entry: PathBuf = entry.unwrap().path();
                let mut new_target_path: PathBuf = target_path.clone();
                // If the source directory contains a directory
                // we need to create it in the target directory
                if entry.is_dir() {
                    let lower_directory = entry.components().next_back().unwrap();
                    new_target_path.push(lower_directory);
                    create_dir_all(&new_target_path).unwrap();
                }

                copy(&entry.to_string_lossy(), &new_target_path.to_string_lossy());
            }
        } else {
            // Add filename to target directory, in order to
            let target_filename: PathBuf = target_path.join(source_path.file_name().unwrap());
            // println!("copying from {source_path:?} to {target_filename:?}");
            fs::copy(&source_path, target_filename).unwrap();
        }
    }
}

pub fn delete(target: &str) {
    let target_path: PathBuf = Path::new(target).to_path_buf();
    if target_path.is_dir() {
        fs::remove_dir_all(&target_path).expect("could not delete directory");
    } else if target_path.is_file() && target_path.exists() {
        // Just print to stderr here because
        // it's a recoverable error.
        fs::remove_file(&target_path)
            .unwrap_or_else(|err| eprintln!("Could not delete file: {err}"));
    }
}

pub fn sync(source: &str, target: &str) {
    let source_path: PathBuf = Path::new(source).to_path_buf();
    let target_path: PathBuf = Path::new(target).to_path_buf();
    if source_path.is_dir() && target_path.is_dir() {
        // First wipe the target, and we're reading through
        // the directory because we want to delete everything
        // _in_ the directory, not the directory itself
        for entry in fs::read_dir(target_path).unwrap() {
            delete(&entry.unwrap().path().to_string_lossy());
        }
        // Then copy everything from the source to the target
        copy(source, target);
    } else {
        eprintln!("Both {source} and {target} must be directories");
    }
}
