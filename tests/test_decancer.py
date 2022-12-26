from decancer_py import parse

YEET = parse("𝔂ＥＥ𝓣")

def test_contains() -> None:
    assert YEET.contains("ee")
    assert not YEET.contains("no")

def test_starts_with() -> None:
    assert YEET.starts_with("ye")
    assert not YEET.starts_with("et")

def test_ends_with() -> None:
    assert YEET.ends_with("et")
    assert not YEET.ends_with("ye")

def test_equals() -> None:
    assert YEET == "YEET"
    assert not YEET == "NO"