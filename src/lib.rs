use std::fs;
use std::fs::create_dir_all;
// use std::os::linux::fs::MetadataExt; // Linux specific file metadata can only be used on
use std::path::{Path, PathBuf};
// use std::thread::sleep;
// use std::time::Duration;

pub fn copy(source: &str, target: &str) {
    let source_path: PathBuf = Path::new(source).to_path_buf();
    let target_path: PathBuf = Path::new(target).to_path_buf();

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

pub fn delete(target: &str) {
    let target_path: PathBuf = Path::new(target).to_path_buf();
    if target_path.is_dir() {
        fs::remove_dir_all(&target_path).expect("could not delete directory");
    }
    fs::remove_file(&target_path).expect("could not delete file");
}

pub fn sync(source: &str, target: &str) {
    // First wipe the target
    delete(target);
    // Then copy everything fromt the source to the target
    copy(source, target);

    // let source_path: PathBuf = Path::new(source).to_path_buf();
    // let target_path: PathBuf = Path::new(target).to_path_buf();

    // let old_state: Vec<(PathBuf, i64)> = get_state(&source_path);

    // println!("old state is {old_state:?}");
    // sleep(Duration::from_secs(10));

    // let new_state: Vec<(PathBuf, i64)> = get_state(&source_path);

    // println!("new state is {new_state:?}");
    // let deleted: Vec<(PathBuf, i64)> = old_state
    //     .into_iter()
    //     .filter(|item| !new_state.contains(item))
    //     .collect();

    // for path in deleted {
    //     // let mut new_target_path = target_path.clone();
    //     let object_to_delete = path.0;
    //     let source_root = object_to_delete.components().next().unwrap();
    //     object_to_delete.strip_prefix(source_root).unwrap();
    //     let new_target_path = target_path.clone().join(object_to_delete);
    //     println!("{new_target_path:?}");
    //     delete(&new_target_path.to_string_lossy());
    // }

    // // scenarios to compare:
    // // new file appears and needs to be copied
    // // file is deleted
    // // file is changed

    // fn get_state(source_path: &PathBuf) -> Vec<(PathBuf, i64)> {
    //     let mut state: Vec<(PathBuf, i64)> = Vec::new();
    //     for entry in fs::read_dir(source_path).unwrap() {
    //         let entry: PathBuf = entry.unwrap().path();
    //         let modified_time = fs::metadata(&entry).unwrap().st_mtime();
    //         state.push((entry, modified_time));
    //     }
    //     return state;
    // }
}
