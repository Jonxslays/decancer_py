extern crate decancer;

use pyo3::{basic::CompareOp, prelude::*};

/// A small wrapper around the str datatype used for comparison reasons.
#[pyclass]
pub struct CuredString(decancer::CuredString);

#[pymethods]
impl CuredString {
    /// Checks if this string similarly starts with another string.
    #[pyo3(text_signature = "($self, other: str) -> bool")]
    fn starts_with(&self, other: &str) -> bool {
        self.0.starts_with(other)
    }

    /// Checks if this string similarly ends with another string.
    #[pyo3(text_signature = "($self, other: str) -> bool")]
    fn ends_with(&self, other: &str) -> bool {
        self.0.ends_with(other)
    }

    /// Checks if this string similarly contains another string.
    #[pyo3(text_signature = "($self, other: str) -> bool")]
    fn contains(&self, other: &str) -> bool {
        self.0.contains(other)
    }

    fn __richcmp__(&self, other: &str, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.0 == other),
            CompareOp::Ne => Ok(self.0 != other),
            _ => Ok(false),
        }
    }

    fn __contains__(&self, other: &str) -> PyResult<bool> {
        Ok(self.contains(other))
    }

    fn __bool__(&self) -> PyResult<bool> {
        Ok(self.0.len() > 0)
    }

    fn __str__(&self) -> PyResult<&str> {
        Ok(&self.0)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("CuredString(\"{}\")", &self.0))
    }
}

/// Parses a jank string into a less toxic lowercase string wrapped in CuredString object.
#[pyfunction]
#[pyo3(text_signature = "(text: str) -> CuredString")]
pub fn parse(text: String) -> CuredString {
    CuredString(decancer::cure(&text))
}

/// The module we export to python
#[pymodule]
fn decancer_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", std::env!("CARGO_PKG_VERSION"))?;

    m.add_class::<CuredString>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;

    Ok(())
}
