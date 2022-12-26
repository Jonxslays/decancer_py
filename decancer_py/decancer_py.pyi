from __future__ import annotations

__all__ = ("contains", "parse", "starts_with", "ends_with")


def contains(string: str, text: str) -> bool:
    """Check whether some string contains some text.

    Args:
        string (`str`): The string to search through
        text (`str`): The text to check for

    Returns:
        `bool` - True if the text is contained in the string
    """


def starts_with(string: str, text: str) -> bool:
    """Check whether some string starts with some text.

    Args:
        string (`str`): The string to search through
        text (`str`): The text to check for

    Returns:
        `bool` - True if the string starts with the text
    """


def ends_with(string: str, text: str) -> bool:
    """Check whether some string ends with some text.

    Args:
        string (`str`): The string to search through
        text (`str`): The text to check for

    Returns:
        `bool` - True if the string ends with the text
    """


def parse(text: str) -> str:
    """Parses a jank string into a less toxic lowercase string.

    Args:
        text (`str`): The text to parse

    Returns:
        `str` - The parsed string in all lowercase
    """
