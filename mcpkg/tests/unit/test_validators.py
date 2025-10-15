import pytest
from mcpkg.validators import validate_name


def test_validate_name_accepts_valid_names() -> None:
    validate_name("valid-name", "Test")
    validate_name("valid_name", "Test")
    validate_name("valid.name", "Test")
    validate_name("ValidName123", "Test")
    validate_name("a", "Test")
    validate_name("123", "Test")


def test_validate_name_rejects_empty_string() -> None:
    with pytest.raises(ValueError, match="Test name cannot be empty"):
        validate_name("", "Test")


def test_validate_name_rejects_spaces() -> None:
    with pytest.raises(
        ValueError, match="Test name 'invalid name' contains invalid characters"
    ):
        validate_name("invalid name", "Test")


def test_validate_name_rejects_special_characters() -> None:
    with pytest.raises(
        ValueError, match="Test name 'invalid@name' contains invalid characters"
    ):
        validate_name("invalid@name", "Test")

    with pytest.raises(
        ValueError, match="Test name 'invalid/name' contains invalid characters"
    ):
        validate_name("invalid/name", "Test")

    with pytest.raises(
        ValueError, match="Test name 'invalid\\*name' contains invalid characters"
    ):
        validate_name("invalid*name", "Test")


def test_validate_name_case_sensitive() -> None:
    validate_name("UpperCase", "Test")
    validate_name("lowerCase", "Test")
    validate_name("MixedCase", "Test")


def test_validate_name_max_length() -> None:
    validate_name("a" * 255, "Test")

    with pytest.raises(ValueError, match="Test name exceeds maximum length"):
        validate_name("a" * 256, "Test")
