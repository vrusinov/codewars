# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Vladimir Rusinov
"""
Strip Comments

https://www.codewars.com/kata/51c8e37cee245da6b40000bd/train/python
"""


def strip_comments(string, markers):
    """Strip comments from the given string based on the provided markers."""
    lines = string.split("\n")
    result = []
    for line in lines:
        min_index = len(line)
        for marker in markers:
            index = line.find(marker)
            if index != -1 and index < min_index:
                min_index = index
        result.append(line[:min_index].rstrip())
    return "\n".join(result)
