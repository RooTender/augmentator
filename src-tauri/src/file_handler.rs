use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn preprocess_data(input_dir: &Path, output_dir: &Path) -> io::Result<()> {
    fs::create_dir_all(output_dir)?;

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            match image::open(&path) {
                Ok(_) => {
                    let relative_path = path.strip_prefix(input_dir).unwrap();
                    let destination = output_dir.join(relative_path);

                    if let Some(parent) = destination.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    fs::copy(&path, destination)?;
                },
                Err(_) => {} // Not an image, skip
            }
        } else if path.is_dir() {
            let sub_output_dir = output_dir.join(entry.file_name());
            preprocess_data(&path, &sub_output_dir)?;
        }
    }

    Ok(())
}

pub fn collect_file_paths(dir: &Path, base: &Path) -> io::Result<HashSet<PathBuf>> {
    let mut file_paths = HashSet::new();
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            file_paths.extend(collect_file_paths(&path, base)?);
        } else {
            if let Some(relative_path) = path.strip_prefix(base).ok() {
                file_paths.insert(relative_path.to_path_buf());
            }
        }
    }
    Ok(file_paths)
}
