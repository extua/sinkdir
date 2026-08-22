use std::fs;

pub fn copy(source: &str, target: &str) {
    let source_path = fs::canonicalize(source).unwrap();
    let target_path = fs::canonicalize(target).unwrap();

    if source_path.is_file() && target_path.is_dir() {
        // Need to add filename and trailing separator to target directory
        let target_filename = format!(
            "{}/{}",
            target_path.to_string_lossy(),
            source_path.file_name().unwrap().to_str().unwrap()
        );
        fs::copy(&source_path, target_filename).unwrap();
    }

    if source_path.is_dir() {
        for entry in fs::read_dir(source).unwrap() {
            println!("{entry:?}")
        }
    }
}

// fn delete(source_path: Path, target_path: Path) {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
