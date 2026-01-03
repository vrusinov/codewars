"""Roman Numerals Helper

https://www.codewars.com/kata/51b66044bce5799a7f000003/train/python
"""
# SPDX-FileCopyrightText: 2026 Vladimir Rusinov
# SPDX-License-Identifier: Apache-2.0

numerals = {
    "M": 1000,
    "CM": 900,
    "D": 500,
    "CD": 400,
    "C": 100,
    "XC": 90,
    "L": 50,
    "XL": 40,
    "X": 10,
    "IX": 9,
    "V": 5,
    "IV": 4,
    "I": 1,
}
from_roman_numerals = {
    "CM": 900,
    "M": 1000,
    "CD": 400,
    "D": 500,
    "XC": 90,
    "C": 100,
    "XL": 40,
    "L": 50,
    "IX": 9,
    "X": 10,
    "IV": 4,
    "V": 5,
    "I": 1,
}


class RomanNumerals:
    """Roman Numerals Helper"""

    @staticmethod
    def to_roman(val: int) -> str:
        """Convert integer to Roman numeral string."""
        ret = ""

        for numeral, integer in numerals.items():
            while val >= integer:
                ret += numeral
                val -= integer

        return ret

    @staticmethod
    def from_roman(roman_num: str) -> int:
        """Convert Roman numeral string to integer."""
        ret = 0
        for numeral, integer in from_roman_numerals.items():
            ret += roman_num.count(numeral) * integer
            roman_num = roman_num.replace(numeral, "")
        return ret
