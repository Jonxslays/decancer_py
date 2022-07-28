use decancer::Decancer;
use pyo3::prelude::*;

const BRIDGE: Decancer = Decancer::new();

/// Parses a jank string into a less toxic lowercase string.
#[pyfunction]
#[pyo3(text_signature = "(text: str) -> str")]
pub fn parse(text: String) -> String {
    BRIDGE.cure(&text)
}

/// Check whether some string contains some text.
#[pyfunction]
#[pyo3(text_signature = "(string: str, text: str, parse: bool = True) -> bool")]
pub fn contains(mut string: String, text: String, parse: Option<bool>) -> bool {
    if parse.unwrap_or(true) {
        string = BRIDGE.cure(&string)
    }

    BRIDGE.contains(&string, &text)
}

/// The module we export to python
#[pymodule]
fn decancer_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", std::env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(contains, m)?)?;
    Ok(())
}
