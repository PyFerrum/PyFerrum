mod csv_parser;

use pyo3::prelude::*;

#[pymodule]
fn pyferrum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(csv_parser::read_csv, m)?)?;
    m.add_function(wrap_pyfunction!(csv_parser::filter_csv, m)?)?;
    Ok(())
}