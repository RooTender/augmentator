use image::*;
use palette::{FromColor, Hsl, Srgb};
use rand::rngs::StdRng;
use rand::Rng;

// Move 
pub struct ShiftV;
impl ImageTransformation for ShiftV {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let shift_pixels = rng.gen_range(1..img.height());
        Ok(shift_image(img, shift_pixels, ShiftAxis::Vertical))
    }
}

pub struct ShiftH;
impl ImageTransformation for ShiftH {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let shift_pixels = rng.gen_range(1..img.width());
        Ok(shift_image(img, shift_pixels, ShiftAxis::Horizontal))
    }
}

enum ShiftAxis {
    Horizontal,
    Vertical,
}

fn shift_image(img: &DynamicImage, shift_pixels: u32, axis: ShiftAxis) -> DynamicImage {
    let rgba_img: RgbaImage = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    let mut temp_img = ImageBuffer::new(width, height);

    match axis {
        ShiftAxis::Horizontal => {
            for y in 0..height {
                for x in 0..width {
                    let new_x = (x + shift_pixels) % width;
                    let pixel = *rgba_img.get_pixel(x, y);
                    temp_img.put_pixel(new_x, y, pixel);
                }
            }
        },
        ShiftAxis::Vertical => {
            for x in 0..width {
                for y in 0..height {
                    let new_y = (y + shift_pixels) % height;
                    let pixel = *rgba_img.get_pixel(x, y);
                    temp_img.put_pixel(x, new_y, pixel);
                }
            }
        }
    }
    
    DynamicImage::from(temp_img)
}

// Rotate
pub struct Rotate90;
impl ImageTransformation for Rotate90 {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.rotate90())
    }
}

pub struct Rotate180;
impl ImageTransformation for Rotate180 {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.rotate180())
    }
}

pub struct Rotate270;
impl ImageTransformation for Rotate270 {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.rotate270())
    }
}

// Flip
pub struct FlipH;
impl ImageTransformation for FlipH {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.fliph())
    }
}

pub struct FlipV;
impl ImageTransformation for FlipV {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.flipv())
    }
}

// Colors
pub struct HueRotate;
impl ImageTransformation for HueRotate {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let hue_angle = rng.gen_range(0..360);
        Ok(img.huerotate(hue_angle))
    }
}

pub struct Saturate;
impl ImageTransformation for Saturate {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let saturation =  rng.gen_range(0.0..1.0);
        Ok(adjust_saturation(img, saturation))
    }
}

fn adjust_saturation(img: &DynamicImage, saturation: f32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    
    for (x, y, pixel) in img.pixels() {
        let rgb = Srgb::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0);
        let hsl = Hsl::from_color(rgb);

        let adjusted_hsl = Hsl::new(hsl.hue, hsl.saturation * saturation, hsl.lightness);
        let adjusted_rgb: Srgb<u8> = Srgb::from_color(adjusted_hsl).into_format();

        let (r, g, b) = adjusted_rgb.into_components();
        output_img.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    DynamicImage::ImageRgba8(output_img)
}

pub struct Brighten;
impl ImageTransformation for Brighten {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let brightness = rng.gen_range(-255..255);
        Ok(img.brighten(brightness))
    }
}

pub struct Contrast;
impl ImageTransformation for Contrast {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let contrast = rng.gen_range(-100.0..100.0);
        Ok(img.adjust_contrast(contrast))
    }
}

// Filters
pub struct Grayscale;
impl ImageTransformation for Grayscale {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.grayscale())
    }
}

pub struct Invert;
impl ImageTransformation for Invert {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        // Operation is in-place, that's why it's cloned
        let mut img_clone = img.clone();
        img_clone.invert();
        Ok(img_clone)
    }
}

pub trait ImageTransformation {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage>;
}
