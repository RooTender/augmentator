use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use image::GenericImageView;
use image::ImageError;

pub type DimensionFilter = Box<dyn Fn(&(u32, u32), &Vec<PathBuf>) -> bool + Send + Sync>;

pub enum ConversionFormat {
    RGBA
}

enum ScaleDirection {
    Up,
    Down,
}

pub fn preprocess_data(input_dir: &Path, output_dir: &Path, filters: Vec<DimensionFilter>) -> io::Result<()> {
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

pub fn delete_unpaired_files(dir1: &Path, dir2: &Path, scale_factor: u32) -> io::Result<()> {
    let file_paths_dir1 = collect_file_paths(dir1, dir1)?;
    let file_paths_dir2 = collect_file_paths(dir2, dir2)?;

    let scaled_up_file_paths = scale_dir_dimensions(file_paths_dir1.clone(), scale_factor, ScaleDirection::Up);
    let scaled_down_file_paths = scale_dir_dimensions(file_paths_dir2.clone(), scale_factor, ScaleDirection::Down);

    for relative_path in file_paths_dir1.difference(&scaled_down_file_paths) {
        fs::remove_file(dir1.join(relative_path))?;
    }
    for relative_path in file_paths_dir2.difference(&scaled_up_file_paths) {
        fs::remove_file(dir2.join(relative_path))?;
    }

    delete_empty_dirs(dir1)?;
    delete_empty_dirs(dir2)?;

    Ok(())
}

pub fn convert_images(dir: &Path, format: ConversionFormat) -> Result<(), ImageError> {
    let file_paths = collect_file_paths(dir, dir).map_err(|e| ImageError::IoError(e))?;

    for path in file_paths {
        let full_path = dir.join(&path);
        let img = image::open(&full_path)?;

        let converted = match format {
            ConversionFormat::RGBA => img.to_rgba8()
        };

        converted.save(full_path)?;
    }
    Ok(())
}

fn scale_dir_dimensions(paths: HashSet<PathBuf>, scale_factor: u32, direction: ScaleDirection) -> HashSet<PathBuf> {
    paths.into_iter().map(|path| {
        path.iter().map(|component| {
            if let Some(component_str) = component.to_str() {
                if let Some((width, height)) = component_str.split_once('x') {
                    if let (Ok(mut width), Ok(mut height)) = (width.parse::<u32>(), height.parse::<u32>()) {
                        match direction {
                            ScaleDirection::Up => {
                                width *= scale_factor;
                                height *= scale_factor;
                            }
                            ScaleDirection::Down => {
                                width /= scale_factor;
                                height /= scale_factor;
                            }
                        }
                        return format!("{}x{}", width, height).into();
                    }
                }
            }
            component.to_owned()
        }).collect::<PathBuf>()
    }).collect()
}

fn collect_file_paths(dir: &Path, base: &Path) -> io::Result<HashSet<PathBuf>> {
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

fn delete_empty_dirs(dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            delete_empty_dirs(&path)?;
            if fs::read_dir(&path)?.next().is_none() { // Check if the directory is empty
                fs::remove_dir(&path)?;
            }
        }
    }
    Ok(())
}

fn traverse_and_collect(input_dir: &Path, dimension_map: &mut HashMap<(u32, u32), Vec<PathBuf>>) -> io::Result<()> {
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
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
