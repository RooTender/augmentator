use image::{DynamicImage, ImageBuffer, Pixel};
use rand::prelude::*;
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};

enum ShiftAxis {
    Horizontal,
    Vertical,
}

fn shift_image(img: &DynamicImage, shift_pixels: u32, axis: ShiftAxis) -> DynamicImage {
    fn shift<T: Pixel + 'static + Clone>(
        img: &ImageBuffer<T, Vec<T::Subpixel>>,
        shift_pixels: u32,
        axis: ShiftAxis,
    ) -> ImageBuffer<T, Vec<T::Subpixel>> {
        let (width, height) = img.dimensions();
        let mut temp_img = ImageBuffer::new(width, height);

        match axis {
            ShiftAxis::Horizontal => {
                for y in 0..height {
                    let row: Vec<T> = (0..width).map(|x| img.get_pixel(x, y).clone()).collect();
                    for (x, pixel) in row.into_iter().cycle().skip((width - shift_pixels) as usize).take(width as usize).enumerate() {
                        temp_img.put_pixel(x as u32, y, pixel);
                    }
                }
            },
            ShiftAxis::Vertical => {
                for x in 0..width {
                    let col: Vec<T> = (0..height).map(|y| img.get_pixel(x, y).clone()).collect();
                    for (y, pixel) in col.into_iter().cycle().skip((height - shift_pixels) as usize).take(height as usize).enumerate() {
                        temp_img.put_pixel(x, y as u32, pixel);
                    }
                }
            }
        }

        temp_img
    }

    match img {
        DynamicImage::ImageLuma8(buf) => DynamicImage::ImageLuma8(shift(buf, shift_pixels, axis)),
        DynamicImage::ImageLuma16(buf) => DynamicImage::ImageLuma16(shift(buf, shift_pixels, axis)),

        DynamicImage::ImageLumaA8(buf) => DynamicImage::ImageLumaA8(shift(buf, shift_pixels, axis)),
        DynamicImage::ImageLumaA16(buf) => DynamicImage::ImageLumaA16(shift(buf, shift_pixels, axis)),

        DynamicImage::ImageRgb8(buf) => DynamicImage::ImageRgb8(shift(buf, shift_pixels, axis)),
        DynamicImage::ImageRgb16(buf) => DynamicImage::ImageRgb16(shift(buf, shift_pixels, axis)),
        DynamicImage::ImageRgb32F(buf) => DynamicImage::ImageRgb32F(shift(buf, shift_pixels, axis)),
        
        DynamicImage::ImageRgba8(buf) => DynamicImage::ImageRgba8(shift(buf, shift_pixels, axis)),
        DynamicImage::ImageRgba16(buf) => DynamicImage::ImageRgba16(shift(buf, shift_pixels, axis)),
        DynamicImage::ImageRgba32F(buf) => DynamicImage::ImageRgba32F(shift(buf, shift_pixels, axis)),
        _ => unimplemented!(),
    }
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

fn main() {
    println!("Please enter the directory containing the images: ");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);

    let paths = get_image_paths(Path::new(buffer.trim())).expect("Failed to gather image paths");

    for (i, path) in paths.iter().enumerate() {
        let img = image::open(path)
          .expect("Failed to open image");

        let shifted_img = shift_image(&img, img.width() / 4, ShiftAxis::Vertical);
        let save_path = format!("test_1.png");

        shifted_img
          .save(&save_path)
          .expect("Failed to save image");

        println!("Saved: {}", save_path);
    }
}
