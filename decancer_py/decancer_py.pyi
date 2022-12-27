from __future__ import annotations

__all__ = ("parse", "CuredString", "__version__")
__version__: str

class CuredString:
    """A small wrapper around a string used for comparisons."""

    def __eq__(self, other: str) -> bool: ...
    def __ne__(self, other: str) -> bool: ...
    def __contains__(self, other: str) -> bool: ...
    def __bool__(self) -> bool: ...
    def __str__(self) -> bool: ...
    def __repr__(self) -> bool: ...
    def starts_with(self, other: str) -> bool:
        """Checks if this cured string starts with the other string.

        Args:
            other: The other string to compare against.

        Returns:
            True if the cured string starts with the other string, else False.
        """
    def ends_with(self, other: str) -> bool:
        """Checks if this cured string ends with the other string.

        Args:
            other: The other string to compare against.

        Returns:
            True if the cured string ends with the other string, else False.
        """
    def contains(self, other: str) -> bool:
        """Checks if this cured string contains the other string.

        Args:
            other: The other string to compare against.

        Returns:
            True if the cured string contains the other string, else False.
        """

def parse(text: str) -> CuredString:
    """Parses a jank string into a less toxic string wrapped in a CuredString
    object.

    Args:
        text: The text to parse.

    Returns:
        The CuredString object to use for comparisons.
    """
