# SPDX-FileCopyrightText: 2025 Vladimir Rusinov
# SPDX-License-Identifier: Apache-2.0
"""Mean Square Error

https://www.codewars.com/kata/51edd51599a189fe7f000015/train/python
"""

from typing import Sequence


def solution(array_a: Sequence[int], array_b: Sequence[int]) -> float:
    """Calculate the Mean Square Error between two arrays."""
    sum_of_squared_differences = 0
    for a, b in zip(array_a, array_b):
        difference = a - b
        sum_of_squared_differences += difference**2

    if len(array_a) > 0:
        return sum_of_squared_differences / len(array_a)
    return 0
