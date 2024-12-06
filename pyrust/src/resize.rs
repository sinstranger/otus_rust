use crate::utils::parse_arg_format;
use pyo3::prelude::*;
use std::io::Cursor;

#[pyfunction]
pub fn resize_image(image_bytes: Vec<u8>, width: u32, height: u32, format: &str) -> PyResult<Vec<u8>> {
    let img = image::load_from_memory(&image_bytes)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to load image: {}", e)))?;
    let resized = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let format = parse_arg_format(format)?;
    let mut buffer = Vec::new();
    resized
        .write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write image: {}", e)))?;

    Ok(buffer)
}
