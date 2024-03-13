extern crate decancer;

use decancer::Options;
use pyo3::{basic::CompareOp, exceptions::PyRuntimeError, prelude::*, types::PyDict};

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

    fn __richcmp__(&self, other: &str, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other,
            CompareOp::Ne => self.0 != other,
            _ => false,
        }
    }

    fn __contains__(&self, other: &str) -> bool {
        self.contains(other)
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }

    fn __str__(&self) -> &str {
        &self.0
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

fn is_dict_key(dict: &PyDict, key: &'static str) -> bool {
    if let Ok(Some(value)) = dict.get_item(key) {
        return matches!(value.is_true(), Ok(true));
    }

    false
}

fn kwargs_to_options(options: Option<&PyDict>) -> Options {
    let mut result = Options::default();

    match options {
        None => result,
        Some(dict) => {
            if is_dict_key(dict, "pure_homoglyph") {
                result = Options::pure_homoglyph();
            }

            if is_dict_key(dict, "retain_capitalization") {
                result = result.retain_capitalization();
            }

            if is_dict_key(dict, "disable_bidi") {
                result = result.disable_bidi();
            }

            if is_dict_key(dict, "retain_diacritics") {
                result = result.retain_diacritics();
            }

            if is_dict_key(dict, "retain_japanese") {
                result = result.retain_japanese();
            }

            if is_dict_key(dict, "retain_emojis") {
                result = result.retain_emojis();
            }

            if is_dict_key(dict, "retain_greek") {
                result = result.retain_greek();
            }

            if is_dict_key(dict, "retain_cyrillic") {
                result = result.retain_cyrillic();
            }

            if is_dict_key(dict, "retain_hebrew") {
                result = result.retain_hebrew();
            }

            if is_dict_key(dict, "retain_arabic") {
                result = result.retain_arabic();
            }

            if is_dict_key(dict, "retain_devanagari") {
                result = result.retain_devanagari();
            }


            if is_dict_key(dict, "retain_bengali") {
                result = result.retain_bengali();
            }


            if is_dict_key(dict, "retain_armenian") {
                result = result.retain_armenian();
            }


            if is_dict_key(dict, "retain_gujarati") {
                result = result.retain_gujarati();
            }


            if is_dict_key(dict, "retain_tamil") {
                result = result.retain_tamil();
            }


            if is_dict_key(dict, "retain_thai") {
                result = result.retain_thai();
            }

            if is_dict_key(dict, "retain_lao") {
                result = result.retain_lao();
            }

            if is_dict_key(dict, "retain_burmese") {
                result = result.retain_burmese();
            }

            if is_dict_key(dict, "retain_khmer") {
                result = result.retain_khmer();
            }

            if is_dict_key(dict, "retain_mongolian") {
                result = result.retain_mongolian();
            }

            if is_dict_key(dict, "retain_chinese") {
                result = result.retain_chinese();
            }

            if is_dict_key(dict, "retain_korean") {
                result = result.retain_korean();
            }

            if is_dict_key(dict, "retain_braille") {
                result = result.retain_braille();
            }

            result
        }
    }
}

/// Parses a jank string into a less toxic lowercase string wrapped in CuredString object.
#[pyfunction]
#[pyo3(signature = (text, **options))]
#[pyo3(text_signature = "(text: str, **options) -> CuredString")]
pub fn parse<'a>(text: String, options: Option<&PyDict>) -> PyResult<CuredString> {
    match decancer::cure(&text, kwargs_to_options(options)) {
        Ok(res) => Ok(CuredString(res)),
        Err(err) => Err(Python::with_gil(|_| {
            PyRuntimeError::new_err(err.to_string())
        })),
    }
}

/// The module we export to python
#[pymodule]
fn decancer_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CuredString>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add("__version__", std::env!("CARGO_PKG_VERSION"))
}
