use image::ImageFormat;
use pyo3::prelude::*;

pub fn parse_arg_format(format: &str) -> Result<ImageFormat, PyErr> {
    let format: ImageFormat = match format.to_lowercase().as_str() {
        "png" => ImageFormat::Png,
        "jpeg" | "jpg" => ImageFormat::Jpeg,
        "bmp" => ImageFormat::Bmp,
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Unsupported format. Use 'png', 'jpeg', or 'bmp'.",
            ))
        }
    };
    Ok(format)
}
