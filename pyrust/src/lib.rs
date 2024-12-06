mod resize;
mod utils;
mod glitch;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::resize::resize_image;
use crate::glitch::add_glitch_effect;


#[pymodule]
fn pyrust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(resize_image, m)?)?;
    m.add_function(wrap_pyfunction!(add_glitch_effect, m)?)?;

    Ok(())
}
