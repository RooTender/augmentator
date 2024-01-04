use image::{GenericImageView, Primitive, DynamicImage, ImageBuffer, Pixel, imageops::colorops};
use rand::prelude::*;
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};

enum ShiftAxis {
    Horizontal,
    Vertical,
}

fn get_image_paths(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut full_paths: Vec<PathBuf> = vec![];

    path.read_dir()?.filter_map(|e| e.ok()).for_each(|file| {
        let filename = file.file_name().to_string_lossy().into_owned();
        if let Some(extension) = filename.split('.').last() {
            if extension == "jpg" || extension == "png" {
                full_paths.push(file.path().canonicalize().unwrap());
            }
        }
    });

    Ok(full_paths)
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
                for (x, pixel) in row.into_iter().cycle().skip((width - shift_pixels) as usize).take(width as usize).enumerate() {
                    temp_img.put_pixel(x as u32, y, pixel);
                }
            }
        },
        ShiftAxis::Vertical => {
            for x in 0..width {
                let col: Vec<P> = (0..height).map(|y| img.get_pixel(x, y).clone()).collect();
                for (y, pixel) in col.into_iter().cycle().skip((height - shift_pixels) as usize).take(height as usize).enumerate() {
                    temp_img.put_pixel(x, y as u32, pixel);
                }
            }
        }
    }

    temp_img
}

fn augment_image(image_path: &Path, save_location: &Path) -> Result<(), Box<dyn Error>> {
    let img = image::open(image_path)?;
    let mut rng = thread_rng();

    let transformations = vec![
        |img: &DynamicImage| img.clone(),
        |img: &DynamicImage| img.rotate90(),
        |img: &DynamicImage| img.rotate180(),
        |img: &DynamicImage| img.rotate270(),
        |img: &DynamicImage| img.flipv(),
        |img: &DynamicImage| img.fliph(),
    ];

    let ops: Vec<Box<dyn Fn(&mut DynamicImage, &mut ThreadRng)>> = vec![
        Box::new(|_: &mut DynamicImage, _: &mut ThreadRng| {}),
        Box::new(|img: &mut DynamicImage, _: &mut ThreadRng| img.invert()),
        Box::new(|img: &mut DynamicImage, rng: &mut ThreadRng| { img.brighten(rng.gen_range(-255..255)); }),
        Box::new(|img: &mut DynamicImage, rng: &mut ThreadRng| { img.adjust_contrast(rng.gen_range(-1.0..1.0)); }),
    ];

    fn apply_shift_and_hue_rotate(img: &DynamicImage, rng: &mut ThreadRng) -> DynamicImage {
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

            let save_path = save_location.join(format!("augmented_{}_{}.png", i, j));
            transformed_img.save(&save_path)?;
        }
    }

    Ok(())
}

fn main() {
    println!("Please enter the directory containing the images: ");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);

    let paths = get_image_paths(Path::new(buffer.trim())).expect("Failed to gather image paths");

    for path in paths {
        augment_image(path.as_path(), Path::new(".")).expect("Failed to augment the image");
    }
}
