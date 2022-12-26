from decancer_py import parse, contains, starts_with, ends_with

YEET = "𝔂ＥＥ𝓣"


def test_contains() -> None:
    assert contains(YEET, "ye")
    assert not contains(YEET, "no")


def test_starts_with() -> None:
    assert starts_with(YEET, "ye")
    assert not starts_with(YEET, "et")


def test_ends_with() -> None:
    assert not ends_with(YEET, "ye")
    assert ends_with(YEET, "et")


def test_parse() -> None:
    assert "yeet" == parse(YEET)
