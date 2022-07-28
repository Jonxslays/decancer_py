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

Two functions are exported from decancer_py:
- `parse` - Parse a jank string to a normal string.
- `contains` - Check if some string contains some other string.

The `contains` API is altered a bit from the decancer crate.

The result from `parse` will always be in ALL LOWERCASE.

By default `contains` will attempt to parse the string and then see if
the string is contained. Pass `parse=False` to skip the parsing step.

To be clear, using `"s" in "string"` is going to be more performant than
calling `contains` due to the overhead of the FFI. The main use case is
if you want to check if some regular string is in some jank string.

```py
from decancer_py import parse


parsed = parse("𝔂ＥＥ𝓣")
assert parsed == "yeet"
```

```py
from decancer_py import contains


assert contains("yeet", "ye")
assert contains("𝔂ＥＥ𝓣", "ye")
assert not contains("𝔂ＥＥ𝓣", "ye", parse=False)
```

## License

decancer_py is licensed under the
[MIT License](https://github.com/Jonxslays/decancer_py/blob/master/LICENSE).
