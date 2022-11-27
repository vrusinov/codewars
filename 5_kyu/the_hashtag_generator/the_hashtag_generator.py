# SPDX-FileCopyrightText: 2022 Vladimir Rusinov
#
# SPDX-License-Identifier: Apache-2.0

"""
https://www.codewars.com/kata/52449b062fb80683ec000024/train/python

The marketing team is spending way too much time typing in hashtags.
Let's help them with our own Hashtag Generator!

Here's the deal:

* It must start with a hashtag (#).
* All words must have their first letter capitalized.
* If the final result is longer than 140 chars it must return false.

If the input or the result is an empty string it must return false.
"""

def generate_hashtag(s):
    words = s.strip().split()
    if not words:
        return False
    ret = []
    total_len = 0
    for w in words:
        w = w.title()
        total_len += len(w)
        if total_len > 139:
            return False
        ret.append(w)
    ret = '#' + ''.join(ret)
    if len(ret) > 140:
        return False
    return ret
