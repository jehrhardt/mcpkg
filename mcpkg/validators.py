import re

NAME_PATTERN = re.compile(r"^[a-zA-Z0-9._-]+$")
MAX_NAME_LENGTH = 255


def validate_name(name: str, entity_type: str) -> None:
    if not name:
        raise ValueError(f"{entity_type} name cannot be empty")

    if len(name) > MAX_NAME_LENGTH:
        raise ValueError(
            f"{entity_type} name exceeds maximum length of {MAX_NAME_LENGTH} characters"
        )

    if not NAME_PATTERN.match(name):
        raise ValueError(
            f"{entity_type} name '{name}' contains invalid characters. "
            f"Allowed: a-z, A-Z, 0-9, -, _, ."
        )
