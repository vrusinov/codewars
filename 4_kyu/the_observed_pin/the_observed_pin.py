# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2025 Vladimir Rusinov
"""
The observed PIN

https://www.codewars.com/kata/5263c6999e0f40dee200059d/train/python
"""

NEIGHBOURS = {
    "1": ["1", "2", "4"],
    "2": ["1", "2", "3", "5"],
    "3": ["2", "3", "6"],
    "4": ["1", "4", "5", "7"],
    "5": ["2", "4", "5", "6", "8"],
    "6": ["3", "5", "6", "9"],
    "7": ["4", "7", "8"],
    "8": ["5", "7", "8", "9", "0"],
    "9": ["6", "8", "9"],
    "0": ["8", "0"],
}


def get_neighbors(pin: str) -> list[str]:
    """Get all possible neighbors for a given pin digit."""
    return NEIGHBOURS.get(pin, [])


def get_pins(observed: str) -> list[str]:
    """Generate all possible PINs based on the observed digits.

    Idea: do it recursively until we have single-digit left."""
    ret = []
    if len(observed) == 1:
        return get_neighbors(observed)
    for neighbour in get_neighbors(observed[0]):
        for remainder in get_pins(observed[1:]):
            ret.append(neighbour + remainder)
    return ret
