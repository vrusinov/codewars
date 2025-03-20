"""
SPDX-FileCopyrightText: 2024-2025 Vladimir Rusinov
SPDX-License-Identifier: Apache-2.0

Give me a Diamond

https://www.codewars.com/kata/5503013e34137eeeaa001648/train/python
"""

from typing import Optional


def diamond(n: int) -> Optional[str]:
    """Prints a diamond of size n."""
    if n <= 0:
        return None
    if n % 2 == 0:
        return None

    lines = [""] * n
    diamonds = 1
    while diamonds <= n // 2:
        line = " " * (n // 2 - diamonds + 1) + "*" * (diamonds * 2 - 1)
        lines[diamonds - 1] = line
        lines[n - diamonds] = line
        diamonds += 1
    lines[n // 2] = "*" * n
    return "\n".join(lines) + "\n"


if __name__ == "__main__":
    print(diamond(7))
    print(diamond(5))
    print(diamond(3))
    print(diamond(1))
