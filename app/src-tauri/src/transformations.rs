use image::*;
use palette::{FromColor, Hsl, Srgb};
use rand::rngs::StdRng;
use rand::Rng;

// Move
#[derive(Default)]
pub struct ShiftV;
impl ImageTransformation for ShiftV {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let shift = sample_shift(img.height(), rng, 0.10, 0.30);
        let shift = if rng.gen_bool(0.5) { shift } else { (img.height() - shift) % img.height() };

        Ok(shift_image(img, shift, ShiftAxis::Vertical))
    }
}

#[derive(Default)]
pub struct ShiftH;
impl ImageTransformation for ShiftH {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let shift = sample_shift(img.width(), rng, 0.10, 0.30);
        let shift = if rng.gen_bool(0.5) { shift } else { (img.width() - shift) % img.width() };

        Ok(shift_image(img, shift, ShiftAxis::Horizontal))
    }
}

fn sample_shift(dim: u32, rng: &mut StdRng, frac_min: f32, frac_max: f32) -> u32 {
    if dim <= 1 { return 0; }
    let f = rng.gen_range(frac_min..frac_max);
    let mut px = (dim as f32 * f).round() as u32;
    px = px.clamp(1, dim - 1);
    px
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
        }
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
#[derive(Default)]
pub struct Rotate90;
impl ImageTransformation for Rotate90 {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.rotate90())
    }
}

#[derive(Default)]
pub struct Rotate180;
impl ImageTransformation for Rotate180 {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.rotate180())
    }
}

#[derive(Default)]
pub struct Rotate270;
impl ImageTransformation for Rotate270 {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.rotate270())
    }
}

// Flip
#[derive(Default)]
pub struct FlipH;
impl ImageTransformation for FlipH {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.fliph())
    }
}

#[derive(Default)]
pub struct FlipV;
impl ImageTransformation for FlipV {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.flipv())
    }
}

// Colors
#[derive(Default)]
pub struct HueRotate;
impl ImageTransformation for HueRotate {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let min_deg = 10.0;
        let max_deg = 60.0;

        let mut deg = sample_triangular(rng, -max_deg, 0.0, max_deg);
        if deg.abs() < min_deg {
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            deg = sign * min_deg;
        }

        Ok(img.huerotate(deg.round() as i32))
    }
}

fn sample_triangular(rng: &mut StdRng, min: f32, mode: f32, max: f32) -> f32 {
    let u: f32 = rng.gen();
    let c = (mode - min) / (max - min + f32::EPSILON);
    if u < c {
        min + ((u * (max - min) * (mode - min)).sqrt())
    } else {
        max - (((1.0 - u) * (max - min) * (max - mode)).sqrt())
    }
}

#[derive(Default)]
pub struct Saturate;
impl ImageTransformation for Saturate {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let stats = compute_stats(img);

        let room_up = (1.0 - stats.mean_sat).max(0.0);
        let room_dn = stats.mean_sat.max(0.0);

        let k = 0.8;
        let min_factor = (1.0 - k * room_dn).max(0.0);
        let max_factor =  1.0 + k * room_up;

        let mut f = sample_triangular(rng, min_factor, 1.0, max_factor);

        let min_delta = 0.10f32.min(((max_factor - 1.0).abs()).max((1.0 - min_factor).abs()));
        if (f - 1.0).abs() < min_delta {
            let go_up = rng.gen_bool(0.5);
            if go_up {
                f = (1.0 + min_delta).min(max_factor);
            } else {
                f = (1.0 - min_delta).max(min_factor);
            }
        }

        Ok(adjust_saturation_factor(img, f))
    }
}

fn adjust_saturation_factor(img: &DynamicImage, factor: f32) -> DynamicImage {
    let (w, h) = img.dimensions();
    let mut out: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(w, h);

    for (x, y, p) in img.pixels() {
        let rgb = Srgb::new(p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0);
        let hsl = Hsl::from_color(rgb);
        let new_hsl = Hsl::new(hsl.hue, (hsl.saturation * factor).clamp(0.0, 1.0), hsl.lightness);
        let adj: Srgb<u8> = Srgb::from_color(new_hsl).into_format();
        let (r, g, b) = adj.into_components();
        out.put_pixel(x, y, Rgba([r, g, b, p[3]]));
    }
    DynamicImage::ImageRgba8(out)
}

#[derive(Default)]
pub struct Brighten;
impl ImageTransformation for Brighten {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let stats = compute_stats(img);

        let max_up = (255.0 * (1.0 - stats.mean_luma)).clamp(10.0, 255.0);
        let max_dn = (255.0 * stats.mean_luma).clamp(10.0, 255.0);

        let min = -max_dn as f32;
        let max =  max_up as f32;

        let mut delta = sample_triangular(rng, min, 0.0, max);

        let near_room = if delta >= 0.0 { max_up } else { max_dn };
        let min_abs = (0.15 * near_room).max(10.0);
        if delta.abs() < min_abs {
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            delta = sign * min_abs;
        }

        delta = delta.clamp(min, max);
        Ok(img.brighten(delta.round() as i32))
    }
}

#[derive(Default)]
pub struct Contrast;
impl ImageTransformation for Contrast {
    fn apply(&self, img: &DynamicImage, rng: &mut StdRng) -> ImageResult<DynamicImage> {
        let stats = compute_stats(img);
        let center_dist = (stats.mean_luma - 0.5).abs();

        let max_inc = (1.0 - 2.0 * center_dist).clamp(0.0, 1.0) * 50.0;
        let max_dec = 50.0;

        let mut c = sample_triangular(rng, -max_dec, 0.0, max_inc);

        let near_room = if c >= 0.0 { max_inc } else { max_dec };
        let min_abs = (0.20 * near_room).max(5.0);
        if c.abs() < min_abs {
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            c = sign * min_abs;
        }

        c = c.clamp(-max_dec, max_inc);
        Ok(img.adjust_contrast(c))
    }
}

// Filters
#[derive(Default)]
pub struct Grayscale;
impl ImageTransformation for Grayscale {
    fn apply(&self, img: &DynamicImage, _: &mut StdRng) -> ImageResult<DynamicImage> {
        Ok(img.grayscale())
    }
}

#[derive(Default)]
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

#[derive(Clone, Copy, Debug, Default)]
struct ImageStats {
    mean_luma: f32,  // 0..1
    mean_sat: f32,   // 0..1 (średnia saturacja w HSL)
}

fn compute_stats(img: &DynamicImage) -> ImageStats {
    let (mut sum_luma, mut sum_sat, mut count) = (0.0f32, 0.0f32, 0u64);
    // Operujemy na 8-bitach dla szybkości
    let rgba = img.to_rgba8();
    for p in rgba.pixels() {
        let r = p[0] as f32 / 255.0;
        let g = p[1] as f32 / 255.0;
        let b = p[2] as f32 / 255.0;
        // Luma (sRGB, aproksymacja Rec. 709)
        let luma = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        let hsl = Hsl::from_color(Srgb::new(r, g, b));
        sum_luma += luma;
        sum_sat += hsl.saturation.max(0.0).min(1.0);
        count += 1;
    }
    if count == 0 {
        return ImageStats::default();
    }
    ImageStats {
        mean_luma: (sum_luma / count as f32).clamp(0.0, 1.0),
        mean_sat:  (sum_sat  / count as f32).clamp(0.0, 1.0),
    }
}