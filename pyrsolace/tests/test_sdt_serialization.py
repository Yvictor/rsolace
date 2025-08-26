"""
Tests for pyrsolace SDT (Solace Data Transfer) serialization functionality

This module tests the dumps() and loads() functions that handle serialization
and deserialization of Python objects to/from Solace SDT format.

Key test: pyrsolace.loads(pyrsolace.dumps({"test": 1})) == {"test": 1}
"""

import pytest
import pyrsolace


def test_sdt_dict_roundtrip_basic():
    """Test basic dict SDT roundtrip: pyrsolace.loads(pyrsolace.dumps({"test": 1})) == {"test": 1}"""
    original = {"test": 1}

    # This is the exact test case from the user
    result = pyrsolace.loads(pyrsolace.dumps(original))

    assert result == original, f"Expected {original}, got {result}"


@pytest.mark.parametrize(
    "test_case,data",
    [
        ("int", {"number": 42}),
        ("str", {"text": "hello"}),
        ("bool_true", {"flag": True}),
        ("bool_false", {"enabled": False}),
        ("float", {"pi": 3.14}),
        ("null", {"empty": None}),
        ("mixed", {"num": 123, "text": "hello", "flag": True, "empty": None}),
    ],
)
def test_sdt_dict_various_types(test_case, data):
    """Test dicts containing different basic types"""
    result = pyrsolace.loads(pyrsolace.dumps(data))

    # Use appropriate comparison for floats
    if "pi" in data and isinstance(data["pi"], float):
        assert abs(result["pi"] - data["pi"]) < 0.0001, (
            f"Float mismatch: {data} != {result}"
        )
    else:
        assert result == data, f"Mismatch for {test_case}: {data} != {result}"


@pytest.mark.parametrize(
    "test_case,data",
    [
        ("numbers", [1, 2, 3, 42]),
        ("strings", ["hello", "world"]),
        ("booleans", [True, False, True]),
        ("mixed", [1, "hello", True, None, 3.14]),
        ("empty", []),
    ],
)
def test_sdt_list_various_types(test_case, data):
    """Test lists containing different basic types"""
    result = pyrsolace.loads(pyrsolace.dumps(data))

    assert result == data, f"Mismatch for {test_case}: {data} != {result}"


@pytest.mark.parametrize(
    "test_case,data",
    [
        ("simple_list", [1, 2, 3, [4, 5, 6]]),
        (
            "mixed_dict",
            {
                "numbers": 123,
                "text": "hello",
                "flag": True,
                "simple_list": [1, 2, 3],
                # "nested_dict": {"numbers": 123, "text": "hello", "flag": True},
            },
        ),
    ],
)
def test_sdt_nested_structures(test_case, data):
    """Test simple nested data structures (no nested containers for now)"""
    result = pyrsolace.loads(pyrsolace.dumps(data))

    assert result == data, f"Mismatch for {test_case}: {data} != {result}"
