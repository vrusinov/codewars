"""
SPDX-FileCopyrightText: 2024 Vladimir Rusinov
SPDX-License-Identifier: Apache-2.0

Break camelCase

Complete the solution so that the function will break up camel casing, using a space between words.

https://www.codewars.com/kata/5208f99aee097e6552000148/train/python

"""


def solution(s: str):
    result = []
    for char in s:
        if char.isupper() and result:
            result.append(" ")
        result.append(char)
    return "".join(result)
