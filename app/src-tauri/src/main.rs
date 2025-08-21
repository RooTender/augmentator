// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod transformation_factory;
mod transformations;

use blake3::Hasher;
use image::DynamicImage;
use rand::{rngs::StdRng, SeedableRng};
use tauri::{AppHandle, Emitter};
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use tauri::webview::WebviewWindow;

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::time::Duration;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
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

type AnyErr = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tauri::command]
async fn augment_dataset(
    app: AppHandle,
    window: WebviewWindow,
    directories: Directories,
    transformations: Vec<String>,
    seed: u64,
) -> Result<String, String> {
    check_missing_directories(&directories)?;

    let input_dir = PathBuf::from(directories.input.trim());
    let output_dir = PathBuf::from(directories.output.trim());

    let image_paths = collect_image_paths(&input_dir).map_err(|e| e.to_string())?;
    let total = image_paths.len();

    let label = window.label().to_string();
    let _ = app.emit_to(&label, "augment-started", total);

    // KLONY:
    let app_for_blocking = app.clone();
    let label_for_blocking = label.clone();

    let app_for_emit = app.clone();
    let label_for_emit = label.clone();

    tauri::async_runtime::spawn(async move {
        let res = tauri::async_runtime::spawn_blocking(move || {
            let (always, one_time) = split_transformations(&transformations);
            let factory = Arc::new(TransformationFactory::new());

            augment_all(
                &image_paths,
                &input_dir,
                &output_dir,
                factory.clone(),
                &always,
                &one_time,
                seed,
                &app_for_blocking,   // <- używamy klona wewnątrz wątku
                &label_for_blocking, // <- i jego labela
                total,
            )
            .map_err(|e| e.to_string()) // zamiana błędu na String, żeby był Send
        })
        .await;

        match res {
            Ok(Ok(())) => {
                let _ = app_for_emit.emit_to(&label_for_emit, "augment-finished", ());
            }
            Ok(Err(err_str)) => {
                let _ = app_for_emit.emit_to(&label_for_emit, "augment-error", err_str);
            }
            Err(join_err) => {
                let _ = app_for_emit.emit_to(&label_for_emit, "augment-error", join_err.to_string());
            }
        }
    });

    Ok("Dataset augmentation started.".into())
}

fn augment_all(
    image_paths: &[PathBuf],
    input_dir: &Path,
    output_dir: &Path,
    factory: Arc<TransformationFactory>,
    always_transformations: &[&str],
    one_time_transformations: &[&str],
    base_seed: u64,
    app: &AppHandle,
    label: &str,
    total: usize,
) -> Result<(), AnyErr> {
    fs::create_dir_all(output_dir)?;

    // współdzielone
    let processed = Arc::new(AtomicUsize::new(0));
    let app = Arc::new(app.clone());
    let label = label.to_string();

    // Flaga do zamknięcia reportera
    let running = Arc::new(AtomicBool::new(true));

    // --- Reporter: JEDNA nitka emituje progres co ~100ms ---
    let processed_for_reporter = processed.clone();
    let app_for_reporter = app.clone();
    let label_for_reporter = label.clone();
    let running_for_reporter = running.clone();

    let reporter = std::thread::spawn(move || {
        let mut last = 0usize;
        while running_for_reporter.load(Ordering::Relaxed) {
            let done = processed_for_reporter.load(Ordering::Relaxed);
            if done != last {
                let percent = (((done as f64 / total as f64) * 100.0).round() as u8).min(100);
                let _ = app_for_reporter.emit_to(
                    &label_for_reporter,
                    "augment-progress",
                    AugmentProgress { processed: done, total, percent },
                );
                last = done;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        // finalny „dopędzacz” po zamknięciu
        let done = processed_for_reporter.load(Ordering::Relaxed);
        let percent = (((done as f64 / total as f64) * 100.0).round() as u8).min(100);
        let _ = app_for_reporter.emit_to(
            &label_for_reporter,
            "augment-progress",
            AugmentProgress { processed: done, total, percent },
        );
    });

    // --- Równoległa praca ---
    // zostaw 1 rdzeń dla UI/systemu
    let num_threads = num_cpus::get().saturating_sub(1).max(1);
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .map_err(|e| format!("Failed to build rayon pool: {e}"))?;
    let chunk = 8;

    pool.install(|| {
        image_paths.par_chunks(chunk).for_each(|paths| {
            for path in paths {
                if let Err(err) = process_single(
                    path,
                    input_dir,
                    output_dir,
                    &factory,
                    always_transformations,
                    one_time_transformations,
                    base_seed,
                ) {
                    eprintln!("Failed to process {}: {err}", path.display());
                }
            }
            // tylko licznik, ZERO emitów stąd
            processed.fetch_add(paths.len(), Ordering::Relaxed);
        });
    });

    // zamknij reportera i poczekaj
    running.store(false, Ordering::Relaxed);
    let _ = reporter.join();

    Ok(())
}


fn process_single(
    path: &Path,
    input_dir: &Path,
    output_dir: &Path,
    factory: &TransformationFactory,
    always_transformations: &[&str],
    one_time_transformations: &[&str],
    base_seed: u64,
) -> Result<(), AnyErr> {
    let relative_path = path.strip_prefix(input_dir)?;
    let output_base = output_dir.join(relative_path).with_extension("");

    if let Some(parent) = output_base.parent() {
        fs::create_dir_all(parent)?;
    }

    let img = image::open(path)?;
    save_image(&output_base.with_extension("png"), &img)?;

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    if one_time_transformations.is_empty() {
        let mut result = img.clone();

        for t_name in always_transformations {
            let t_seed = derive_seed_for_transform(&stem, base_seed, t_name);
            result = apply_transformation(&result, t_name, t_seed, factory).unwrap_or(result);
        }

        let shifted_path = output_base.with_file_name(format!(
            "{}_shifted",
            output_base
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("output")
        ));
        save_image(&shifted_path.with_extension("png"), &result)?;
        return Ok(());
    }

    let mut base = img.clone();
    for transformation in always_transformations {
        let t_seed = derive_seed_for_transform(&stem, base_seed, transformation);
        base = apply_transformation(&base, transformation, t_seed, factory).unwrap_or(base);
    }

    for transformation in one_time_transformations {
        let t_seed = derive_seed_for_transform(&stem, base_seed, transformation);
        if let Some(transformed) = apply_transformation(&base, transformation, t_seed, factory) {
            let out = output_base.with_file_name(format!(
                "{}_{}",
                output_base
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output"),
                transformation
            ));
            save_image(&out.with_extension("png"), &transformed)?;
        }
    }

    Ok(())
}

fn split_transformations<'a>(list: &'a [String]) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut always = Vec::new();
    let mut one_time = Vec::new();

    for item in list {
        let transformation = item.as_str();
        if matches!(transformation, "hor_shift" | "ver_shift") {
            always.push(transformation);
        } else {
            one_time.push(transformation);
        }
    }
    (always, one_time)
}

fn save_image(path: &Path, img: &DynamicImage) -> image::ImageResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }
    img.save(path)
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
                eprintln!(
                    "Error applying transformation '{transformation_name}', skipping.",
                );
                None
            }
        },
        None => {
            eprintln!(
                "Warning: Transformation '{transformation_name}' not implemented, skipping.",
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

fn collect_image_paths(input_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    collect_recursive(input_dir, &mut out)?;
    out.sort();
    Ok(out)
}

fn collect_recursive(dir: &Path, acc: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_recursive(&path, acc)?;

        } else if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if &ext.to_ascii_lowercase() == "png" {
                    acc.push(path);
                }
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
        return Err(format!(
            "Directories {} aren't set.", 
            missing_directories.join(", "))
        );
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
