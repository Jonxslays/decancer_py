import pytest

from decancer_py import parse

YEET = parse("𝔂ＥＥ𝓣")


def test_contains() -> None:
    assert YEET.contains("ee")
    assert not YEET.contains("no")


def test_starts_with() -> None:
    assert YEET.starts_with("ye")
    assert not YEET.starts_with("et")


def test_starts_with_invalid_type() -> None:
    with pytest.raises(TypeError):
        YEET.starts_with(69)  # type: ignore


def test_ends_with() -> None:
    assert YEET.ends_with("et")
    assert not YEET.ends_with("ye")


def test_ends_with_invalid_type() -> None:
    with pytest.raises(TypeError):
        YEET.starts_with(69)  # type: ignore


def test_equals() -> None:
    assert YEET == "yeet"
    assert YEET != "no"


def test_contains_invalid_type() -> None:
    with pytest.raises(TypeError):
        YEET.contains(69)  # type: ignore


def test_dunder_contains() -> None:
    assert "ee" in YEET
    assert "no" not in YEET


def test_dunder_contains_invalid_type() -> None:
    with pytest.raises(TypeError):
        69 in YEET  # type: ignore


def test_chinese() -> None:
    result = parse("旧")
    assert result == "18"


def test_retain_chinese() -> None:
    result = parse("旧", retain_chinese=True)
    assert result == "旧"
