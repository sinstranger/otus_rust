use image::{DynamicImage, GenericImageView, ImageFormat, RgbaImage, Rgba};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::{Cursor, Read};


#[pyfunction]
fn resize_image_from_bytes(image_bytes: Vec<u8>, width: u32, height: u32, format: &str,) -> PyResult<Vec<u8>> {
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


#[pyfunction]
fn glitch_effect(image_bytes: Vec<u8>, offset: u32, format: &str) -> PyResult<Vec<u8>> {

    let img = image::load_from_memory(&image_bytes).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("Failed to load image: {}", e))
    })?;
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

    let mut buffer = Vec::new();
    new_img
        .write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("Failed to write image: {}", e))
        })?;

    Ok(buffer)
}


/// A Python module implemented in Rust.
#[pymodule]
fn pyrust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(resize_image_from_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(glitch_effect, m)?)?;

    Ok(())
}
