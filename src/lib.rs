use pyo3::prelude::*;

#[pyfunction]
pub fn add(_a: usize, _b: usize) -> usize {
    todo!()
}

#[pymodule]
fn pyo3_exception_testing(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
