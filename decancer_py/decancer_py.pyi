__all__ = ("parse", "CuredString")
__version__: str

class CuredString:
    """A small wrapper around a string used for comparisons."""

    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __contains__(self, other: str) -> bool: ...
    def __bool__(self) -> bool: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
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

def parse(text: str, **options: bool) -> CuredString:
    """Parses a jank string into a less toxic string wrapped in a CuredString
    object.

    Args:
        text: The text to parse.

    Keyword Args:
        **options: The options to enable or disable.

    Returns:
        The CuredString object to use for comparisons.

    Available options:
        - formatter: Use if all you want is text formatting.

        - pure_homoglyph: Prevents parsing chars from major foreign
            writing systems.

        - retain_capitalization: Prevents changing all chars to lower
            case.

        - disable_bidi: Disables the unicode bidirectional algorithm.
            Only use this when you don't expect and right-to-left chars.

        - retain_diacritics: Prevent parsing chars with diacritics or
            accents.

        - retain_japanese: Prevent parsing katakana and hiragana chars.

        - retain_emojis: Prevent parsing emojis.

        - retain_greek: Prevent parsing greek chars.

        - retain_cyrillic: Prevent parsing cyrillic chars.

        - retain_hebrew: Prevent parsing hebrew chars.

        - retain_arabic: Prevent parsing arabic chars.

        - retain_devanagari: Prevent parsing devanagari chars.

        - retain_bengali: Prevent parsing bengali chars.

        - retain_armenian: Prevent parsing armenian chars.

        - retain_gujarati: Prevent parsing gujarati chars.

        - retain_tamil: Prevent parsing tamil chars.

        - retain_thai: Prevent parsing thai chars.

        - retain_lao: Prevent parsing lao chars.

        - retain_burmese: Prevent parsing burmese chars.

        - retain_khmer: Prevent parsing khmer chars.

        - retain_mongolian: Prevent parsing mongolian chars.

        - retain_chinese: Prevent parsing chinese chars.

        - retain_korean: Prevent parsing korean chars.

        - retain_braille: Prevent parsing braille chars.
    """
