from __future__ import annotations

__all__ = ("contains", "parse")


def contains(string: str, text: str, parse: bool = True) -> bool:
    """Check whether some string contains some text.

    Args:
        string (`str`): The string to search through
        text (`str`): The text to check for
        parse (`bool`): If True, parse the string first
            Defaults to True

    Returns:
        `bool` - True if the text is contained in the string
    """
def parse(text: str) -> str:
    """Parses a jank string into a less toxic lowercase string.

    Args:
        text (`str`): The text to parse

    Returns:
        `str` - The parsed string in all lowercase
    """
