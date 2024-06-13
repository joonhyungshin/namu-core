use pyo3::prelude::*;

fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pymodule]
fn _namu_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", get_version())?; 
    Ok(())
}

