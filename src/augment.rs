use image::{GenericImageView, Primitive, DynamicImage, ImageBuffer, Pixel, imageops::colorops};
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub fn augment_dataset(samples_path: &Path, output_path: &Path, seed: u64) -> Result<(), Box<dyn Error>> {
    let samples = get_image_paths(samples_path, output_path)?;
    
    samples.par_iter().for_each(|(input_path, output_path)| {
        // Copy original image to the output directory
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create directories");
        }
        fs::copy(input_path, output_path).expect("Failed to copy files");

        // Perform augmentation
        let seed = seed;
        augment_image(output_path, seed)
            .expect("An error occurred during image augmentation");
    });

    Ok(())
}

fn get_image_paths(input_dir: &Path, output_dir: &Path) -> Result<Vec<(PathBuf, PathBuf)>, Box<dyn Error>> {
    let mut paths = Vec::new();

    fn traverse(input_path: &Path, output_path: &Path, paths: &mut Vec<(PathBuf, PathBuf)>) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(input_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let new_output = output_path.join(path.file_name().unwrap());
                fs::create_dir_all(&new_output)?;
                traverse(&path, &new_output, paths)?;
            } else {
                if image::open(&path).is_ok() {
                    let relative_path = path.strip_prefix(input_path)?;
                    let output_file_path = output_path.join(relative_path);
                    paths.push((path, output_file_path));
                }
            }
        }
        Ok(())
    }

    traverse(input_dir, output_dir, &mut paths)?;
    Ok(paths)
}

enum ShiftAxis {
    Horizontal,
    Vertical,
}

fn shift_image<I, P, S>(img: &I, shift_pixels: u32, axis: ShiftAxis) -> ImageBuffer<P, Vec<S>>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel<Subpixel = S> + 'static,
    S: Primitive + 'static,
{
    let (width, height) = img.dimensions();
    let mut temp_img = ImageBuffer::new(width, height);

    match axis {
        ShiftAxis::Horizontal => {
            for y in 0..height {
                let row: Vec<P> = (0..width).map(|x| img.get_pixel(x, y).clone()).collect();
                for (x, pixel) in row
                    .into_iter()
                    .cycle()
                    .skip((width - shift_pixels) as usize)
                    .take(width as usize)
                    .enumerate() {
                        temp_img.put_pixel(x as u32, y, pixel);
                }
            }
        },
        ShiftAxis::Vertical => {
            for x in 0..width {
                let col: Vec<P> = (0..height).map(|y| img.get_pixel(x, y).clone()).collect();
                for (y, pixel) in col
                    .into_iter()
                    .cycle()
                    .skip((height - shift_pixels) as usize)
                    .take(height as usize)
                    .enumerate() {
                        temp_img.put_pixel(x, y as u32, pixel);
                }
            }
        }
    }

    temp_img
}

fn augment_image(save_location: &Path, seed: u64) -> Result<(), Box<dyn Error>> {
    let img = image::open(save_location)
        .expect("Cannot open image from given location");
    let mut rng = StdRng::seed_from_u64(seed);

    let transformations = vec![
        |img: &DynamicImage| img.clone(),
        |img: &DynamicImage| img.rotate90(),
        |img: &DynamicImage| img.rotate180(),
        |img: &DynamicImage| img.rotate270(),
        |img: &DynamicImage| img.flipv(),
        |img: &DynamicImage| img.fliph(),
    ];

    let ops: Vec<Box<dyn Fn(&mut DynamicImage, &mut StdRng)>> = vec![
        Box::new(|_: &mut DynamicImage, _: &mut StdRng| {}),
        Box::new(|img: &mut DynamicImage, _: &mut StdRng| img.invert()),
        Box::new(|img: &mut DynamicImage, rng: &mut StdRng| { 
            img.brighten(rng.gen_range(-255..255)); 
        }),
        Box::new(|img: &mut DynamicImage, rng: &mut StdRng| { 
            img.adjust_contrast(rng.gen_range(-1.0..1.0)); 
        }),
    ];

    fn apply_shift_and_hue_rotate(img: &DynamicImage, rng: &mut StdRng) -> DynamicImage {
        let shift_pixels_v = rng.gen_range(1..img.height());
        let shift_pixels_h = rng.gen_range(1..img.width());
        let hue_angle = rng.gen_range(1..360);

        let shifted_img = shift_image(img, shift_pixels_v, ShiftAxis::Vertical);
        let shifted_img = shift_image(&shifted_img, shift_pixels_h, ShiftAxis::Horizontal);
        DynamicImage::from(colorops::huerotate(&shifted_img, hue_angle))
    }

    for (i, transform) in transformations.iter().enumerate() {
        for (j, op) in ops.iter().enumerate() {
            let mut transformed_img = transform(&apply_shift_and_hue_rotate(&img, &mut rng));
            op(&mut transformed_img, &mut rng);

            let filename: std::borrow::Cow<'_, str> = save_location
                .file_stem()
                .expect("Failed to get filename without extension")
                .to_string_lossy();
            let augmented_path = save_location.with_file_name(format!("{}_{}_{}.png", filename, i, j));

            transformed_img.save(&augmented_path)
                .expect("Cannot save the augmented image");
        }
    }

    Ok(())
}
