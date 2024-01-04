use std::io;
use std::path::{Path, PathBuf};
use std::error::Error;
use image::{GenericImageView, DynamicImage, RgbaImage, Pixel};

enum ShiftAxis {
    Horizontal,
    Vertical,
}

fn shift_image(img: &DynamicImage, shift_pixels: u32, axis: ShiftAxis) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut temp_img: RgbaImage = img.to_rgba8();

    match axis {
        ShiftAxis::Horizontal => {
            for y in 0..height {
                let mut row: Vec<_> = (0..width).map(|x| img.get_pixel(x, y).to_rgba()).collect();
                row.rotate_right(shift_pixels as usize);
                for (x, pixel) in row.into_iter().enumerate() {
                    temp_img.put_pixel(x as u32, y, pixel);
                }
            }
        },
        ShiftAxis::Vertical => {
            for x in 0..width {
                let mut col: Vec<_> = (0..height).map(|y| img.get_pixel(x, y).to_rgba()).collect();
                col.rotate_right(shift_pixels as usize);
                for (y, pixel) in col.into_iter().enumerate() {
                    temp_img.put_pixel(x, y as u32, pixel);
                }
            }
        }
    }

    DynamicImage::ImageRgba8(temp_img)
}

fn get_image_paths(path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>>
{
    let mut full_paths: Vec<PathBuf> = vec![];

    path.read_dir()?.filter_map(|e| e.ok()).for_each(|file|
    {
        let filename = file.file_name().to_string_lossy().into_owned();
        if let Some(extension) = filename.split('.').last()
        {
            if extension == "jpg" || extension == "png"
            {
                full_paths.push(file.path().canonicalize().unwrap());
            }
        }
    });

    Ok(full_paths)
}

fn main()
{
    println!("Please enter the directory containing the images: ");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);

    let paths = get_image_paths(Path::new(buffer.trim()))
        .expect("Failed to gather image paths");

    for path in paths
    {
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
