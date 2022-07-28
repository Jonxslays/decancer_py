from decancer_py import parse, contains

YEET = "𝔂ＥＥ𝓣"


def test_contains() -> None:
    assert contains(YEET, "ye")
    assert not contains(YEET, "no")


def test_contains_when_parse_is_false() -> None:
    assert contains("yeet", "ye", parse=False)
    assert not contains(YEET, "ye", parse=False)


def test_parse() -> None:
    assert "yeet" == parse(YEET)
