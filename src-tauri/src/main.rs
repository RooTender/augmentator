// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod transformation_factory;
mod transformations;
mod file_handler;

use file_handler::DimensionFilter;
use rand::{rngs::StdRng, SeedableRng};
use std::{error::Error, path::Path};
use serde::Deserialize;

use crate::{file_handler::*, transformation_factory::*, transformations::*};

#[derive(Debug, Deserialize)]
struct Directories {
    input: String,
    target: String,
    output: String,
}

#[tauri::command]
fn augment_dataset(directories: Directories, transformations: Vec<String>) -> Result<String, String> {
    check_missing_directories(&directories)?;

    let input_dir = Path::new(directories.input.trim());
    let target_dir = Path::new(directories.target.trim());
    let output_dir = Path::new(directories.output.trim());

    let factory = TransformationFactory::new();

    let filters: Vec<DimensionFilter> = vec![
        //Box::new(move |_, files| files.len() >= 5),
        //Box::new(move |dim, _| dim.0 == dim.1)
        // Add more filters as needed
    ];

    preprocess_data(input_dir, output_dir, &filters).map_err(|e| e.to_string())?;
    preprocess_data(target_dir, output_dir, &filters).map_err(|e| e.to_string())?;

    //delete_unpaired_files(dir_1, dir_2, 2)
        //     .expect("Failed to remove unpaired images");

    //let rng = &mut StdRng::seed_from_u64(69);
    //augment_data(input_dir, output_dir, &factory, &transformations, rng)
    //    .expect("Failed to augment data!");

    println!("Received directories: {:?}", directories);
    println!("Received transformations: {:?}", transformations);

    Ok("Dataset augmentation process started successfully.".into())
}

fn augment_data(
    input_dir: &Path, 
    output_dir: &Path, 
    factory: &TransformationFactory, 
    transformations: &[String], 
    rng: &mut StdRng
) -> Result<(), Box<dyn Error>> {
    let paths = collect_file_paths(input_dir, input_dir)?;

    for path in paths {
        let full_path = input_dir.join(&path);
        let img = image::open(&full_path)?;

        for transformation_name in transformations {
            if let Some(transformation) = factory.create(transformation_name) {
                let img = transformation.apply(&img, rng)?;
                let new_path = output_dir.join(&path);

                img.save(new_path)?;
            } else {
                eprintln!("Warning: Transformation '{}' not found.", transformation_name);
            }
        }
    }

    Ok(())
}

fn check_missing_directories(directories: &Directories) -> Result<(), String> {
    let mut missing_directories = Vec::new();

    if directories.input.trim().is_empty() {
        missing_directories.push("input");
    }
    if directories.target.trim().is_empty() {
        missing_directories.push("target");
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
