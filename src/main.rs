use std::fmt::Error;

use image::{ ImageBuffer, ImageReader, Luma };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let img = ImageReader::open("test_images/inputs/input.jpeg")?.with_guessed_format()?.decode()?;
    tracing::info!("Read image");

    let luma = img.to_luma32f();
    let mut new_luma = ImageBuffer::new(luma.width(), luma.height());
    let width = img.width() as usize;
    let height = img.height() as usize;
    tracing::info!("Init");

    tracing::info!("Starting waveform calculation...");
    for c in 0..width {
        for r in 0..height {
            let pixel = luma.get_pixel(c as u32, r as u32).0[0];
            let new_r = (pixel * ((height as f32) - 1.0)) as u32;
            let pixel_u8 = (pixel * 255.0) as u8;
            new_luma.put_pixel(c as u32, new_r, Luma([pixel_u8]));
        }
    }

    tracing::info!("Saving output image...");
    new_luma.save("output.png")?;

    Ok(())
}
