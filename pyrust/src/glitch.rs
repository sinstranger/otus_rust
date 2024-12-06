use image::{GenericImageView, Rgba, RgbaImage};
use pyo3::prelude::*;
use std::io::Cursor;

use crate::utils::parse_arg_format;

#[pyfunction]
pub fn add_glitch_effect(image_bytes: Vec<u8>, offset: u32, format: &str) -> PyResult<Vec<u8>> {
    let img = image::load_from_memory(&image_bytes)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to load image: {}", e)))?;
    let (width, height) = img.dimensions();
    let mut new_img: RgbaImage = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let red_x = (x + offset) % width;
            let green_x = (x + offset / 2) % width;
            let blue_x = (x + offset / 3) % width;

            let r = img.get_pixel(red_x, y)[0];
            let g = img.get_pixel(green_x, y)[1];
            let b = img.get_pixel(blue_x, y)[2];

            new_img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }

    let format = parse_arg_format(format)?;
    let mut buffer = Vec::new();
    new_img
        .write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write image: {}", e)))?;

    Ok(buffer)
}