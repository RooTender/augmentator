use image::{GenericImageView, Primitive, DynamicImage, ImageBuffer, Pixel, imageops::colorops};
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use std::error::Error;
use std::io::{self, Write};
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

fn augment_image(image_path: &Path, save_location: &Path, seed: u64) -> Result<(), Box<dyn Error>> {
    let img = image::open(image_path)
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

            let save_path = save_location.join(format!("augmented_{}_{}.png", i, j));
            transformed_img.save(&save_path)
                .expect("Cannot save the augmented image");
        }
    }

    Ok(())
}

fn augment_dataset(samples_path: &Path, output_path: &Path, seed: u64) -> Result<(), Box<dyn Error>> {
    let samples = get_image_paths(samples_path)?;

    for image_path in samples {
        augment_image(image_path.as_path(), output_path, seed)
            .expect("An error occurred during image augmentation");
    }

    Ok(())
}

fn read_input(prompt: &str) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read input");

    buffer.trim().to_string()
}

fn main() {
    let seed: u64 = read_input("Seed for deterministic RNG")
        .parse()
        .expect("Please type a number!");

    let input = read_input("Please enter the directory containing the images");
    let output: String = read_input("Please enter the directory where to save files");

    augment_dataset(Path::new(&input), Path::new(&output), seed)
        .expect("Failed to augment dataset");
}
