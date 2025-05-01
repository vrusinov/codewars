# SPDX-FileCopyrightText: 2025 Vladimir Rusinov
# SPDX-License-Identifier: Apache-2.0

"""
int32 to IPv4

https://www.codewars.com/kata/52e88b39ffb6ac53a400022e/train/python
"""


def int32_to_ip(int32: int) -> str:
    """Convert a 32-bit integer to an IPv4 address."""
    return f"{(int32 >> 24) & 0xFF}.{(int32 >> 16) & 0xFF}.{(int32 >> 8) & 0xFF}.{int32 & 0xFF}"
