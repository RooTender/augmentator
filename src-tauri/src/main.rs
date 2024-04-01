// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod transformation_factory;
mod transformations;

use image::DynamicImage;
use rand::{rngs::StdRng, SeedableRng};
use std::{error::Error, fs, io, path::{Path, PathBuf}};
use serde::Deserialize;

use crate::transformation_factory::*;

#[derive(Debug, Deserialize)]
struct Directories {
    input: String,
    output: String,
}

#[tauri::command]
fn augment_dataset(directories: Directories, transformations: Vec<String>) -> Result<String, String> {
    check_missing_directories(&directories)?;

    let (always_transformations, one_time_transformations) = transformations.iter().fold(
        (Vec::new(), Vec::new()),
        |(mut always, mut one_time), t| {
            if t == "hor_shift" || t == "ver_shift" {
                always.push(t.clone());
            } else {
                one_time.push(t.clone());
            }
            (always, one_time)
        },
    );

    let input_dir = Path::new(directories.input.trim());
    let output_dir = Path::new(directories.output.trim());

    let factory = TransformationFactory::new();

    let image_paths = collect_image_paths(input_dir).map_err(|e| e.to_string())?;

    let rng = &mut StdRng::seed_from_u64(1337);
    augment_data(
        &image_paths,
        &input_dir,
        &output_dir,
        &factory,
        &always_transformations,
        &one_time_transformations,
        rng,
    )
    .expect("Failed to augment input data!");

    Ok("Dataset augmentation process started successfully.".into())
}

fn collect_image_paths(input_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut image_paths = Vec::new();

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            match image::open(&path) {
                Ok(_) => {
                    image_paths.push(path);
                },
                Err(_) => {} // Not an image, skip
            }
        } else if path.is_dir() {
            image_paths.extend(collect_image_paths(&path)?);
        }
    }

    Ok(image_paths)
}

fn augment_data(
    image_paths: &Vec<PathBuf>, 
    input_dir: &Path,
    output_dir: &Path, 
    factory: &TransformationFactory, 
    always_transformations: &[String], 
    one_time_transformations: &[String], 
    rng: &mut StdRng,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(output_dir)?;

    for path in image_paths {
        let relative_path = path.strip_prefix(input_dir).expect("Error stripping prefix from path");
        let output_base_path = output_dir.join(relative_path).with_extension("");

        let img = image::open(&path)?;
        let mut img = img.clone();

        if let Some(parent_dir) = output_base_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        img.save(output_base_path.with_extension("png"))?;

        if one_time_transformations.is_empty() {
            for transformation_name in always_transformations {
                img = apply_transformation(&img, transformation_name, rng, factory).unwrap_or(img);
            }
            let output_path = format!("{}_shifted.png", output_base_path.display());
            img.save(Path::new(&output_path))?;
        }
        else {
            for transformation_name in one_time_transformations {
                for transformation_name in always_transformations {
                    img = apply_transformation(&img, transformation_name, rng, factory).unwrap_or(img);
                }
                if let Some(transformed_img) = apply_transformation(&img, transformation_name, rng, factory) {
                    let transformed_output_path = format!("{}_{}.png", output_base_path.display(), transformation_name);
                    transformed_img.save(Path::new(&transformed_output_path))?;
                }
            }
        }
    }

    Ok(())
}

fn apply_transformation(
    img: &DynamicImage,
    transformation_name: &str,
    rng: &mut StdRng,
    factory: &TransformationFactory,
) -> Option<DynamicImage> {
    match factory.create(transformation_name) {
        Some(transformation) => match transformation.apply(img, rng) {
            Ok(transformed_img) => Some(transformed_img),
            Err(_) => {
                println!("Error applying transformation '{}', skipping.", transformation_name);
                None
            },
        },
        None => {
            println!("Warning: Transformation '{}' not implemented, skipping.", transformation_name);
            None
        },
    }
}

fn check_missing_directories(directories: &Directories) -> Result<(), String> {
    let mut missing_directories = Vec::new();

    if directories.input.trim().is_empty() {
        missing_directories.push("input");
    }
    if directories.output.trim().is_empty() {
        missing_directories.push("output");
    }

    if !missing_directories.is_empty() {
        let missing_directories_str = missing_directories.join(", ");
        return Err(format!("Directories {} aren't set.", missing_directories_str));
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![augment_dataset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
