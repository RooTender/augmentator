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
    augment_data(&preprocessed_dir, &augmented_dir, &factory, &transformations, rng)
        .expect("Failed to augment input data!");

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
    fs::create_dir_all(&output_dir)?;

    for path in paths {
        let full_path = input_dir.join(&path);
        let img = image::open(&full_path)?;
    
        for transformation_name in transformations {
            if let Some(transformation) = factory.create(transformation_name) {
                let transformed_img = transformation.apply(&img, rng)?;
    
                if let Some(file_name) = path.file_stem().and_then(|name| name.to_str()) {
                    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
                    let new_file_name = format!("{}_{}.{}", file_name, transformation_name, extension);
                    
                    let new_path = if let Some(parent) = output_dir.join(&path).parent() {
                        fs::create_dir_all(parent)?;
                        parent.join(new_file_name)
                    } else {
                        output_dir.join(new_file_name)
                    };
    
                    transformed_img.save(new_path)?;
                } else {
                    eprintln!("Warning: Could not extract filename for path {:?}", path);
                }
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
