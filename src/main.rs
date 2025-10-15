use image::{ ImageBuffer, ImageReader, Luma };
use std::error::Error;

fn rgb_to_luma709(r: f32, g: f32, b: f32) -> f32 {
    (0.2126 * r + 0.7152 * g + 0.0722 * b).clamp(0.0, 1.0)
}
fn val_to_bin(v: f32, bins: u32) -> u32 {
    (v * ((bins as f32) - 1.0)).floor() as u32
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let img = ImageReader::open("test_images/inputs/input.jpeg")?.with_guessed_format()?.decode()?;
    let rgb = img.to_rgb32f();
    tracing::info!("Image size: {}x{}", rgb.width(), rgb.height());

    let width = rgb.width() as usize;
    let scope_h = 256;
    let mut hist = vec![0u32; width * scope_h as usize];

    // build histogram
    tracing::info!("Building histogram...");
    for x in 0..rgb.width() {
        for y in 0..rgb.height() {
            let p = rgb.get_pixel(x, y).0;
            let y_luma = rgb_to_luma709(p[0], p[1], p[2]);
            let y_bin = val_to_bin(y_luma, scope_h);

            let row = scope_h - 1 - y_bin;
            hist[(row as usize) * width + (x as usize)] += 1;
        }
    }

    // normalize with log scale
    tracing::info!("Normalizing...");
    let max_count = hist.iter().copied().max().unwrap_or(1);
    let mut out = ImageBuffer::<Luma<u8>, Vec<u8>>::new(width as u32, scope_h);
    for row in 0..scope_h as usize {
        for col in 0..width {
            let c = hist[row * width + col] as f32;
            let d = (1.0 + c).ln() / (1.0 + (max_count as f32)).ln();
            out.put_pixel(col as u32, row as u32, Luma([(d * 255.0) as u8]));
        }
    }

    tracing::info!("Saving output image...");
    new_luma.save("output.png")?;

    Ok(())
}
