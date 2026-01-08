# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Vladimir Rusinov
"""
Nesting Structure Comparison

https://www.codewars.com/kata/520446778469526ec0000001/train/python
"""


def same_structure_as(original, other):
    """Check if two structures have the same nesting structure."""
    if not isinstance(original, list) and not isinstance(other, list):
        return True
    if isinstance(original, list) != isinstance(other, list):
        return False
    if len(original) != len(other):
        return False
    for o_item, other_item in zip(original, other):
        if not same_structure_as(o_item, other_item):
            return False
    return True
