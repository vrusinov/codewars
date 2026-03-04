# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Vladimir Rusinov
"""
Explosive Sum

https://www.codewars.com/kata/52ec24228a515e620b0005ef/train/python
"""

cache = {}


def exp_sum(n):
    """Calculate the number of partitions of an integer."""
    if n == 0:
        return 1
    if n < 0:
        return 0
    if n == 1:
        return 1
    if n == 2:
        return 2
    k = 1
    g = 1
    p = 0
    sign = 1
    sign_change = 0
    while g <= n:
        if n - g in cache:
            p += cache[n - g] * sign
        else:
            p += exp_sum(n - g) * sign
        sign_change += 1
        if sign_change == 2:
            sign_change = 0
            sign *= -1
        if k > 0:
            k = -k
        else:
            k = -k + 1
        g = k * (3 * k - 1) // 2
    cache[n] = p
    return p
