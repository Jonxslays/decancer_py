# decancer_py

Python bindings for [decancer](https://github.com/null8626/decancer).

## Installation

You can get started with `decancer_py` by installing from PyPI:

```bash
pip install -U decancer-py
```

## Usage

`decancer_py` can be used to turn sketchy text strings into their more
basic counterparts.

A single function and class are exported from `decancer_py`:

- `CuredString` - A wrapper around a string object that can be used for comparisons.
- `parse` - Parse a jank string into a normal string wrapped in a `CuredString` object.

```py
from decancer_py import parse, CuredString

parsed: CuredString = parse("𝔂ＥＥ𝓣")

assert "ee" in parsed
assert parsed == "yeet"
assert parsed.contains("ee")
assert parsed.ends_with("et")
assert parsed.starts_with("ye")

# Implicit string conversion
print(parsed)
# yeet
```

## License

`decancer_py` is licensed under the
[MIT License](https://github.com/Jonxslays/decancer_py/blob/master/LICENSE).
