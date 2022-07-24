use decancer::Decancer as Decancer_;
use pyo3::prelude::*;

const BRIDGE: Decancer_ = Decancer_::new();

/// Parses a jank string into a less toxic lowercase string.
///
/// Args:
///   text: str - The text to parse
///
/// Returns:
///     str - The parsed string, in all lowercase
#[pyfunction]
#[pyo3(text_signature = "(text: str) -> str")]
pub fn parse(text: String) -> String {
    BRIDGE.cure(&text)
}

/// Check whether some string contains some text.
///
/// Args:
///   string: str - The string to search through
///   text: str - The text to check for
///
/// Returns:
///     bool - true if the text is contained in the string
#[pyfunction]
#[pyo3(text_signature = "(string: str, text: str) -> bool")]
pub fn contains(string: String, text: String) -> bool {
    BRIDGE.contains(&string, &text)
}

/// The module we export to python
#[pymodule]
fn decancer_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", std::env!("CARGO_PKG_VERSION"))?;
    m.add("__all__", vec!["contains", "parse"])?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(contains, m)?)?;
    Ok(())
}
