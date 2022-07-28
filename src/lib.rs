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

#[cfg(test)]
mod tests {
    use crate::contains;
    use crate::parse;

    #[test]
    fn test_contains_when_parse_is_none() {
        let yeet = String::from("𝔂ＥＥ𝓣");
        assert!(contains(yeet.clone(), "ye".to_string(), None));
        assert!(!contains(yeet, "no".to_string(), None));
    }

    #[test]
    fn test_contains_when_parse_is_true() {
        let yeet = String::from("𝔂ＥＥ𝓣");
        assert!(contains(yeet.clone(), "ye".to_string(), Some(true)));
        assert!(!contains(yeet, "no".to_string(), None));
    }

    #[test]
    fn test_contains_when_parse_is_false() {
        let yeet = String::from("𝔂ＥＥ𝓣");
        assert!(contains("yeet".to_string(), "ye".to_string(), Some(false)));
        assert!(!contains(yeet.clone(), "ye".to_string(), Some(false)));
    }

    #[test]
    fn test_parse() {
        let yeet = String::from("𝔂ＥＥ𝓣");
        assert_eq!(parse(yeet), String::from("yeet"));
    }
}
