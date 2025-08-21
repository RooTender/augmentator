// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod transformation_factory;
mod transformations;

use blake3::Hasher;
use image::DynamicImage;
use tauri::{AppHandle, Emitter};
use tauri::webview::WebviewWindow;
use rand::{rngs::StdRng, SeedableRng};
use serde::Serialize;
use serde::Deserialize;
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

use crate::transformation_factory::*;


#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct AugmentProgress {
    processed: usize,
    total: usize,
    percent: u8,
}

#[derive(Debug, Deserialize)]
struct Directories {
    input: String,
    output: String,
}

#[tauri::command]
async fn augment_dataset(
    app: AppHandle,
    window: WebviewWindow,
    directories: Directories,
    transformations: Vec<String>,
    seed: u64,
) -> Result<String, String> {
    check_missing_directories(&directories)?;

    let (always_transformations, one_time_transformations) =
        transformations
            .iter()
            .fold((Vec::new(), Vec::new()), |(mut always, mut one_time), t| {
                if t == "hor_shift" || t == "ver_shift" {
                    always.push(t.clone());
                } else {
                    one_time.push(t.clone());
                }
                (always, one_time)
            });

    let input_dir = PathBuf::from(directories.input.trim());
    let output_dir = PathBuf::from(directories.output.trim());

    let mut image_paths = collect_image_paths(&input_dir).map_err(|e| e.to_string())?;
    image_paths.sort();
    let total = image_paths.len();

    let label = window.label().to_string();
    let _ = app.emit_to(&label, "augment-started", total);

    tauri::async_runtime::spawn_blocking(move || {
        let base_seed: u64 = seed;
        let factory = TransformationFactory::new();

        if let Err(e) = augment_data_with_progress(
            &image_paths,
            &input_dir,
            &output_dir,
            &factory,
            &always_transformations,
            &one_time_transformations,
            base_seed,
            &app,
            &label,
            total,
        ) {
            let _ = app.emit_to(&label, "augment-error", e.to_string());
            return Err(e.to_string());
        }

        let _ = app.emit_to(&label, "augment-finished", ());
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| e.to_string())??;

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
                }
                Err(_) => {} // Not an image, skip
            }
        } else if path.is_dir() {
            image_paths.extend(collect_image_paths(&path)?);
        }
    }

    Ok(image_paths)
}

fn augment_data_with_progress(
    image_paths: &Vec<PathBuf>,
    input_dir: &Path,
    output_dir: &Path,
    factory: &TransformationFactory,
    always_transformations: &[String],
    one_time_transformations: &[String],
    base_seed: u64,
    app: &AppHandle,
    label: &str,
    total: usize,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(output_dir)?;

    for (idx, path) in image_paths.iter().enumerate() {
        let relative_path = path
            .strip_prefix(input_dir)
            .expect("Error stripping prefix from path");

        let output_base_path = output_dir.join(relative_path).with_extension("");

        let img = image::open(&path)?;

        if let Some(parent_dir) = output_base_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }

        let stem = path.file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        img.save(output_base_path.with_extension("png"))?;

        if one_time_transformations.is_empty() {
            let mut result = img.clone();

            for shift_transformation_name in always_transformations {
                let t_seed = derive_seed_for_transform(&stem, base_seed, shift_transformation_name);
                result = apply_transformation(&result, shift_transformation_name, t_seed, factory)
                            .unwrap_or(result);
            }

            let output_path = format!("{}_shifted.png", output_base_path.display());
            result.save(Path::new(&output_path))?;

        } else {
            for one_time_transformation in one_time_transformations {
                let mut base = img.clone();

                for transformation_name in always_transformations {
                    let t_seed = derive_seed_for_transform(&stem, base_seed, transformation_name);
                    base = apply_transformation(&base, transformation_name, t_seed, factory)
                        .unwrap_or(base);
                }

                let t_seed = derive_seed_for_transform(&stem, base_seed, one_time_transformation);
                if let Some(transformed_img) =
                    apply_transformation(&base, one_time_transformation, t_seed, factory)
                {
                    let transformed_output_path =
                        format!("{}_{}.png", output_base_path.display(), one_time_transformation);
                    transformed_img.save(Path::new(&transformed_output_path))?;
                }
            }
        }

        let processed = idx + 1;
        let percent = (((processed as f64 / total as f64) * 100.0).round() as u8).min(100);
        let _ = app.emit_to(
            label, "augment-progress",
            AugmentProgress { processed, total, percent }
        );
    }

    Ok(())
}

fn apply_transformation(
    img: &DynamicImage,
    transformation_name: &str,
    seed: u64,
    factory: &TransformationFactory,
) -> Option<DynamicImage> {
    let mut rng = StdRng::seed_from_u64(seed);

    match factory.create(transformation_name) {
        Some(transformation) => match transformation.apply(img, &mut rng) {
            Ok(transformed_img) => Some(transformed_img),
            Err(_) => {
                println!(
                    "Error applying transformation '{}', skipping.",
                    transformation_name
                );
                None
            }
        },
        None => {
            println!(
                "Warning: Transformation '{}' not implemented, skipping.",
                transformation_name
            );
            None
        }
    }
}

fn derive_seed_for_transform(stem: &str, base_seed: u64, transform: &str) -> u64 {
    let mut h = Hasher::new();
    h.update(&base_seed.to_le_bytes());
    h.update(stem.as_bytes());
    h.update(transform.as_bytes());

    let out = h.finalize();
    let mut eight = [0u8; 8];

    eight.copy_from_slice(&out.as_bytes()[..8]);
    u64::from_le_bytes(eight)
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
        return Err(format!(
            "Directories {} aren't set.",
            missing_directories_str
        ));
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![augment_dataset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
