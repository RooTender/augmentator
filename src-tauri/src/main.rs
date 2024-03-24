// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod transformation_factory;
mod transformations;
mod file_handler;

use file_handler::DimensionFilter;
use rand::{rngs::StdRng, SeedableRng};
use std::{error::Error, fs, path::Path};
use serde::Deserialize;

use crate::{file_handler::*, transformation_factory::*};

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
    let preprocessed_dir = output_dir.join("pre-processed");
    let augmented_dir = output_dir.join("augmented");

    let factory = TransformationFactory::new();

    let filters: Vec<DimensionFilter> = vec![
        //Box::new(move |_, files| files.len() >= 5),
        //Box::new(move |dim, _| dim.0 == dim.1)
        // Add more filters as needed
    ];

    preprocess_data(input_dir, &preprocessed_dir, &filters).map_err(|e| e.to_string())?;

    let rng = &mut StdRng::seed_from_u64(69);
    augment_data(
        &preprocessed_dir,
        &augmented_dir,
        &factory,
        &always_transformations,
        &one_time_transformations,
        rng,
    )
    .expect("Failed to augment input data!");

    Ok("Dataset augmentation process started successfully.".into())
}

fn augment_data(
    input_dir: &Path, 
    output_dir: &Path, 
    factory: &TransformationFactory, 
    always_transformations: &[String], 
    one_time_transformations: &[String], 
    rng: &mut StdRng
) -> Result<(), Box<dyn Error>> {
    let paths = collect_file_paths(input_dir, input_dir)?;
    fs::create_dir_all(output_dir)?;

    for path in paths.iter() {
        let full_path = input_dir.join(&path);
        let mut img = image::open(&full_path)?;

        for transformation_name in always_transformations {
            img = apply_transformation(&img, factory, transformation_name, rng)?;
        }

        for transformation_name in one_time_transformations {
            let transformed_img = apply_transformation(&img, factory, transformation_name, rng)?;
            save_transformed_image(&transformed_img, &path, output_dir, transformation_name)?;
        }
    }

    Ok(())
}

fn apply_transformation(
    img: &image::DynamicImage,
    factory: &TransformationFactory,
    transformation_name: &str,
    rng: &mut StdRng,
) -> Result<image::DynamicImage, Box<dyn Error>> {
    match factory.create(transformation_name) {
        Some(transformation) => transformation.apply(img, rng).map_err(Into::into),
        None => {
            println!("Warning: Transformation '{}' not implemented, skipping.", transformation_name);
            Ok(img.clone()) // Return the original image unchanged
        }
    }
}

fn save_transformed_image(
    img: &image::DynamicImage,
    path: &Path,
    output_dir: &Path,
    transformation_name: &str,
) -> Result<(), Box<dyn Error>> {
    if let Some(file_stem) = path.file_stem().and_then(|name| name.to_str()) {
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("png");
        let new_file_name = format!("{}_{}.{}", file_stem, transformation_name, extension);
        let new_path = output_dir.join(new_file_name);

        img.save(new_path)?;
        Ok(())
    } else {
        Err("Could not extract filename".into())
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
