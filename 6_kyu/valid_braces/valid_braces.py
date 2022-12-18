# SPDX-FileCopyrightText: 2022 Vladimir Rusinov <vladimir.rusinov@gmail.com>
#
# SPDX-License-Identifier: Apache-2.0
"""Valid Braces

https://www.codewars.com/kata/5277c8a221e209d3f6000b56/train/python
"""

BRACE_PAIRS = {
    '[': ']',
    '(': ')',
    '{': '}',
}


def valid_braces(string):
    braces = []
    for brace in string:
        # If it's an opening brace add to braces stack
        if brace in BRACE_PAIRS:
            braces.append(brace)
        else:
            # Check if it corresponds to the one on top of the braces stack
            if not braces:
                return False
            if BRACE_PAIRS[braces.pop()] != brace:
                return False
    if braces:
        return False
    return True
