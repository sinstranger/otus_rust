use image::{DynamicImage, GenericImageView, ImageFormat};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::{Cursor, Read};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn resize_image_from_bytes(
    image_bytes: Vec<u8>,
    width: u32,
    height: u32,
    format: &str,
) -> PyResult<Vec<u8>> {
    // Читаем изображение из байтов
    let img = image::load_from_memory(&image_bytes).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("Failed to load image: {}", e))
    })?;

    // Изменяем размер
    let resized = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

    // Сохраняем в буфер
    let mut buffer = Vec::new();
    let format = match format.to_lowercase().as_str() {
        "png" => ImageFormat::Png,
        "jpeg" | "jpg" => ImageFormat::Jpeg,
        "bmp" => ImageFormat::Bmp,
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Unsupported format. Use 'png', 'jpeg', or 'bmp'.",
            ))
        }
    };
    resized
        .write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Failed to write image: {}", e))
        })?;

    Ok(buffer)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyrust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(resize_image_from_bytes, m)?)?;
    Ok(())
}
