use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use image::GenericImageView;

pub type DimensionFilter = Box<dyn Fn(&(u32, u32), &Vec<PathBuf>) -> bool + Send + Sync>;

pub fn preprocess_data(input_dir: &Path, output_dir: &Path, filters: &[DimensionFilter]) -> io::Result<()> {
    let mut dimension_map: HashMap<(u32, u32), Vec<PathBuf>> = HashMap::new();
    traverse_and_collect(input_dir, &mut dimension_map)?;

    for (dimensions, files) in dimension_map {
        if filters.iter().all(|f| f(&dimensions, &files)) {
            let output_subdir = output_dir.join(format!("{}x{}", dimensions.0, dimensions.1));
            fs::create_dir_all(&output_subdir)?;

            for file in files {
                let filename = file.file_name()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to extract filename"))?;
                fs::copy(&file, output_subdir.join(filename))?;
            }
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

fn traverse_and_collect(input_dir: &Path, dimension_map: &mut HashMap<(u32, u32), Vec<PathBuf>>) -> io::Result<()> {
    for entry in fs::read_dir(input_dir)? {
        let entry = entry.expect("Failed to read the entry.");
        let path = entry.path();
        if path.is_dir() {
            traverse_and_collect(&path, dimension_map)?;
            continue;
        }

        if let Ok(img) = image::open(&path) {
            let dimensions = img.dimensions();
            dimension_map.entry(dimensions).or_insert_with(Vec::new).push(path);
        }
    }

    Ok(())
}
