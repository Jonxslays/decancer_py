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

    fn __richcmp__(&self, other: &str, op: CompareOp) -> PyResult<bool> {
        Ok(match op {
            CompareOp::Eq => self.0 == other,
            CompareOp::Ne => self.0 != other,
            _ => false,
        })
    }

    fn __contains__(&self, other: &str) -> PyResult<bool> {
        Ok(self.contains(other))
    }

    fn __bool__(&self) -> PyResult<bool> {
        Ok(!self.0.is_empty())
    }

    fn __str__(&self) -> PyResult<&str> {
        Ok(&self.0)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }
}

fn is_dict_key(dict: &PyDict, key: &'static str) -> bool {
    if let Ok(Some(value)) = dict.get_item(key) {
        return matches!(value.is_true(), Ok(true));
    }

    false
}

macro_rules! options_override {
    ($dict:ident,$output:ident,$($option:ident),*) => {
        $(
            if is_dict_key($dict, stringify!($output)) {
                $output = decancer::Options::$option();
            }
        )*
    };
}

macro_rules! options {
    ($dict:ident,$output:ident,$($option:ident),*) => {
        $(
            if is_dict_key($dict, stringify!($output)) {
                $output = $output.$option();
            }
        )*
    };
}

fn kwargs_to_options(options: Option<&PyDict>) -> Options {
    match options {
        Some(dict) => {
            let mut output = Options::default();

            options_override!(dict, output, formatter, pure_homoglyph);

            options!(
                dict,
                output,
                retain_capitalization,
                disable_bidi,
                retain_diacritics,
                retain_japanese,
                retain_emojis,
                retain_greek,
                retain_cyrillic,
                retain_hebrew,
                retain_arabic,
                retain_devanagari,
                retain_bengali,
                retain_armenian,
                retain_gujarati,
                retain_tamil,
                retain_thai,
                retain_lao,
                retain_burmese,
                retain_khmer,
                retain_mongolian,
                retain_chinese,
                retain_korean,
                retain_braille
            );

            output
        }

        None => Options::default(),
    }
}

/// Parses a jank string into a less toxic lowercase string wrapped in CuredString object.
#[pyfunction]
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
    m.add("__version__", std::env!("CARGO_PKG_VERSION"))?;

    m.add_class::<CuredString>()?;
    m.add_function(wrap_pyfunction!(parse, m)?)
}
