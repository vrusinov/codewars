# SPDX-FileCopyrightText: 2022 Vladimir Rusinov
#
# SPDX-License-Identifier: Apache-2.0

from typing import Sequence


def create_phone_number(n: Sequence[int]) -> str:
    str_n = [str(x) for x in n]
    code = ''.join(str_n[:3])
    part_1 = ''.join(str_n[3:6])
    part_2 = ''.join(str_n[6:])
    return f'({code}) {part_1}-{part_2}'
