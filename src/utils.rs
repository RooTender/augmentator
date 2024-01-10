use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use image::{GenericImageView, DynamicImage};

pub fn preprocess_data(input_dir: &Path, output_dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        if let Ok(img) = image::open(&path) {
            let (mut width, mut height) = img.dimensions();
            if height > width {
                std::mem::swap(&mut width, &mut height);
            }
            let output_subdir = output_dir.join(format!("{}x{}", width, height));
            fs::create_dir_all(&output_subdir)?;
            img.save(output_subdir.join(entry.file_name()))?;
        }
    }
    Ok(())
}
