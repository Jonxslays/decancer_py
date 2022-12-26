# decancer_py

Python bindings for [decancer](https://github.com/null8626/decancer).

## Installation

You can get started with decancer_py by installing from PyPI:

```bash
pip install -U decancer-py
```

## Usage

decancer_py can be used to turn sketchy text strings into their more
basic counterparts.

Only one function is exported from decancer_py:
- `parse` - Parse a jank string to a normal string wrapped in a `CuredString` object.

```py
from decancer_py import parse

parsed = parse("𝔂ＥＥ𝓣")

assert parsed == "yeet"
assert parsed.starts_with("ye")
assert parsed.ends_with("et")
assert parsed.contains("ee")

# turn it into an ordinary python str

parsed_as_str = str(parsed)
print(parsed_as_str)
```


## License

decancer_py is licensed under the
[MIT License](https://github.com/Jonxslays/decancer_py/blob/master/LICENSE).