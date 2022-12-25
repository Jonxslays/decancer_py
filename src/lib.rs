extern crate decancer;
use pyo3::prelude::*;

/// Parses a jank string into a less toxic lowercase string.
#[pyfunction]
#[pyo3(text_signature = "(text: str) -> str")]
pub fn parse(text: String) -> String {
    decancer::cure(&text).into_str()
}

/// Check whether some string contains some text.
#[pyfunction]
#[pyo3(text_signature = "(string: str, text: str) -> bool")]
pub fn contains(string: String, text: String) -> bool {
    decancer::cure(&string).contains(&text)
}

/// Check whether a string starts with some text.
#[pyfunction]
#[pyo3(text_signature = "(string: str, text: str) -> bool")]
pub fn starts_with(string: String, text: String) -> bool {
    decancer::cure(&string).starts_with(&text)
}

/// Check whether a string ends with some text.
#[pyfunction]
#[pyo3(text_signature = "(string: str, text: str) -> bool")]
pub fn ends_with(string: String, text: String) -> bool {
    decancer::cure(&string).ends_with(&text)
}

/// The module we export to python
#[pymodule]
fn decancer_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", std::env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(contains, m)?)?;
    m.add_function(wrap_pyfunction!(starts_with, m)?)?;
    m.add_function(wrap_pyfunction!(ends_with, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{contains, starts_with, ends_with, parse};

    #[test]
    fn test_contains() {
        let yeet = String::from("𝔂ＥＥ𝓣");
        assert!(contains(yeet.clone(), "ye".to_string()));
        assert!(!contains(yeet, "no".to_string()));
    }

    #[test]
    fn test_parse() {
        let yeet = String::from("𝔂ＥＥ𝓣");
        assert_eq!(parse(yeet), String::from("yeet"));
    }

    #[test]
    fn test_starts_with() {
        let yeet = String::from("𝔂ＥＥ𝓣");
        assert!(starts_with(yeet.clone(), "ye".to_string()));
        assert!(!starts_with(yeet.clone(), "et".to_string()));
    }

    #[test]
    fn test_ends_with() {
        let yeet = String::from("𝔂ＥＥ𝓣");
        assert!(!ends_with(yeet.clone(), "ye".to_string()));
        assert!(ends_with(yeet.clone(), "et".to_string()));
    }
}
