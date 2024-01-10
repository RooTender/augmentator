use std::io;
use std::fs;
use std::path::Path;
use image::{self, GenericImageView};
use std::collections::HashSet;
use std::sync::Mutex;

pub fn preprocess_data(input_dir: &Path, output_dir: &Path) -> io::Result<()> {
    let seen_dimensions = Mutex::new(HashSet::new());
    traverse_and_process(input_dir, output_dir, &seen_dimensions)
}

fn traverse_and_process(input_dir: &Path, output_dir: &Path, seen_dimensions: &Mutex<HashSet<(u32, u32)>>) -> io::Result<()> {
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            traverse_and_process(&path, output_dir, seen_dimensions)?;
            continue;
        }

        match image::open(&path) {
            Ok(img) => {
                let (mut width, mut height) = img.dimensions();
                if height > width {
                    std::mem::swap(&mut width, &mut height);
                }
                let dimensions = (width, height);

                let mut dimensions_set = seen_dimensions.lock().unwrap();
                let output_subdir = output_dir.join(format!("{}x{}", width, height));
                if dimensions_set.insert(dimensions) {
                    fs::create_dir_all(&output_subdir)?;
                }
                drop(dimensions_set);

                img.save(output_subdir.join(entry.file_name()))
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            },
            Err(e) => {
                println!("Error opening file {:?}: {}", path, e);
                continue;
            },
        }
    }

    Ok(())
}
