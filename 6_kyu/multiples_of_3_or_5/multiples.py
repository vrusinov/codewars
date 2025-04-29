# SPDX-FileCopyrightText: 2025 Vladimir Rusinov
# SPDX-License-Identifier: Apache-2.0
"""
Multiples of 3 or 5.

https://www.codewars.com/kata/514b92a657cdc65150000006/train/python
"""


def solution(number: int) -> int:
    """Returns the sum of all the multiples of 3 or 5 below the given number."""
    ret = 0
    number -= 1
    while number >= 3:
        if number % 3 == 0 or number % 5 == 0:
            ret += number
        number -= 1
    return ret
