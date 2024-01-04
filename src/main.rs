use std::io;
use std::path::{Path, PathBuf};
use std::error::Error;
use image::{GenericImageView, DynamicImage, RgbaImage, Pixel};

fn shift_image_vertically(img: &DynamicImage, shift_pixels: u32) -> DynamicImage {
    let (width, height) = img.dimensions();

    let mut temp_img: RgbaImage = img.to_rgba8();

    for y in 0..height {
        for x in 0..width {
            let new_y = (y + shift_pixels) % height;
            let pixel = img.get_pixel(x, y).to_rgba();

            temp_img.put_pixel(x, new_y, pixel);
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

        let shifted_img = shift_image_vertically(&img, img.height() / 2);
        let save_path = format!("test_1.png");

        shifted_img
          .save(&save_path)
          .expect("Failed to save image");

        println!("Saved: {}", save_path);
    }
}
