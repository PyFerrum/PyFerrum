use pyo3::prelude::*;
use csv::Reader;

/// CSV 파일을 읽어서 Python 리스트로 반환
#[pyfunction]
pub fn read_csv(file_path: &str) -> PyResult<Vec<Vec<String>>> {
    let mut rdr = Reader::from_path(file_path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result.map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        records.push(record.iter().map(String::from).collect());
    }

    Ok(records)
}

#[pyfunction]
pub fn filter_csv(file_path: &str, column_name: &str, filter_value: &str) -> PyResult<Vec<Vec<String>>> {
    let mut rdr = Reader::from_path(file_path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
    
    let headers = rdr.headers().map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?.clone();
    let column_index = headers.iter().position(|h| h == column_name)
        .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{}' not found", column_name)))?;

    let mut filtered_records = Vec::new();
    filtered_records.push(headers.iter().map(String::from).collect()); // 헤더 추가

    for result in rdr.records() {
        let record = result.map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        if record.get(column_index) == Some(filter_value) {
            filtered_records.push(record.iter().map(String::from).collect());
        }
    }

    Ok(filtered_records)
}

/// Rust 모듈을 Python에서 사용하도록 등록
#[pymodule]
fn pyferrum_csv(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_csv, m)?)?;
    m.add_function(wrap_pyfunction!(filter_csv, m)?)?;
    Ok(())
}