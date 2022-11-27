#!/usr/bin/python3

# SPDX-FileCopyrightText: 2022 Vladimir Rusinov
#
# SPDX-License-Identifier: Apache-2.0

"""
Splits the string into pairs of two characters. If the string contains an odd number of characters then it should replace the missing second character of the final pair with an underscore ('_').
"""

def solution(s):
    result = []
    for i in range(0, len(s), 2):
        hunk = s[i:i+2]
        if len(hunk) == 1:
            hunk += '_'
        result.append(hunk)
    return result