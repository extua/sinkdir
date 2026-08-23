use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn copy(source: &str, target: &str) {
    let source_path: PathBuf = fs::canonicalize(source).unwrap();
    let target_path: PathBuf = fs::canonicalize(target).unwrap();

    if target_path.is_dir() {
        if source_path.is_dir() {
            // Read through the directory and recursively
            // run all paths back through this copy function
            for entry in fs::read_dir(source_path).unwrap() {
                let entry: PathBuf = entry.unwrap().path();
                let mut new_target_path = target_path.clone();
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
            // Need to add filename and trailing separator to target directory
            let target_filename = format!(
                "{}/{}",
                target_path.to_string_lossy(),
                source_path.file_name().unwrap().to_str().unwrap()
            );
            println!("copying from {source_path:?} to {target_filename:?}");
            fs::copy(&source_path, target_filename).unwrap();
        }
    }
}

pub fn delete(target_path: &str) {
    let target_path: PathBuf = fs::canonicalize(target_path).unwrap();
    if target_path.is_dir() {
        fs::remove_dir_all(&target_path).expect("could not delete directory");
    }
    fs::remove_file(&target_path).expect("could not delete file");
}

// pub fn sync(source_path, target_path) {

// }
