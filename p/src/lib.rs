use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
pub fn sum_2(a: usize, b: usize) -> PyResult<usize> {
    Ok(a + b)
}

/// A Python module implemented in Rust.
#[pymodule]
fn p(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_2, m)?)?;
    Ok(())
}
