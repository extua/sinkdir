use std::fs;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
// Linux-specific metadata used to get the file
// last modified value
use std::os::linux::fs::MetadataExt;

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

// Simple sync process which recursively deletes everything in
// the target directory, then copies the source directory in its
// place
pub fn sync(source: &str, target: &str) {
    let source_path: PathBuf = Path::new(source).to_path_buf();
    let target_path: PathBuf = Path::new(target).to_path_buf();
    if source_path.is_dir() && target_path.is_dir() {
        // We want to delete everything _in_ the directory,
        // not the directory itself
        for entry in fs::read_dir(target_path).unwrap() {
            delete(&entry.unwrap().path().to_string_lossy());
        }
        // Then copy everything from the source to the target
        copy(source, target);
    } else {
        eprintln!("Both {source} and {target} must be directories");
    }
}

// This was a more ambitious attempt at reading through both source
// and target directories to selectively delete / copy only files
// which need to be deleted / copied.
pub fn _sync(source: &str, target: &str) {
    let source_path: PathBuf = Path::new(source).to_path_buf();
    let target_path: PathBuf = Path::new(target).to_path_buf();

    // This function reads recursively through a directory and
    // returns a list of tuples containing
    // 1. filename PathBuf
    // 2. last modified time in seconds from unix epoch
    fn get_state(source_path: &PathBuf) -> Vec<(PathBuf, i64)> {
        // Instantiate empty vec ready to pass into
        // directory reading loop
        let mut state: Vec<(PathBuf, i64)> = Vec::new();
        fn read_dir<'reading_state>(
            source_path: &'reading_state PathBuf,
            state: &'reading_state mut Vec<(PathBuf, i64)>,
        ) -> &'reading_state mut Vec<(PathBuf, i64)> {
            for entry in fs::read_dir(source_path).unwrap() {
                let entry: PathBuf = entry.unwrap().path();
                let modified_time = fs::metadata(&entry).unwrap().st_mtime();
                state.push((
                    entry.clone(), // entry is cloned here to avoid
                    // further messing around with lifetimes.
                    modified_time,
                ));
                // If we hit a directory, return it back
                // to read_dir again
                if entry.is_dir() {
                    read_dir(&entry, state);
                }
            }
            return state;
        }
        read_dir(source_path, &mut state);
        return state.clone();
    }

    // Get the current state of the source and target
    let source_state: Vec<(PathBuf, i64)> = get_state(&source_path);
    // println!("source state is {source_state:?}");
    let target_state: Vec<(PathBuf, i64)> = get_state(&target_path);
    // println!("target state is {target_state:?}");

    // Find the items in the source list which are not
    // present in the target list, these are things which
    // have been deleted. Unfortunately at this point the
    // comparison breaks because the paths for  source and
    // target directories are _necessarily_ different
    let absent_from_target: Vec<(PathBuf, i64)> = target_state
        .into_iter()
        .filter(|item| !source_state.contains(item))
        .collect();

    // Loop through the deleted source objects and
    // try to normalise the file paths
    for path in absent_from_target {
        let object_to_delete = path.0;
        // Remove the highest directory in the path,
        // this is the relative point from which we
        // can compare to the source directory
        let ultimate_parent = object_to_delete.components().next().unwrap();
        // Remove the ultimate parent from the path
        object_to_delete.strip_prefix(ultimate_parent).unwrap();
        delete(&object_to_delete.to_string_lossy());
    }
}
