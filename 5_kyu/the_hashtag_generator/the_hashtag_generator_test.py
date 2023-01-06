# SPDX-FileCopyrightText: 2022 Vladimir Rusinov
#
# SPDX-License-Identifier: Apache-2.0

import codewars_test as test

from the_hashtag_generator import generate_hashtag

test.describe("Basic tests")
test.assert_equals(
    generate_hashtag(""), False, "Expected an empty string to return False"
)
test.assert_equals(
    generate_hashtag("Do We have A Hashtag")[0],
    "#",
    "Expeted a Hashtag (#) at the beginning.",
)
test.assert_equals(
    generate_hashtag("Codewars"), "#Codewars", "Should handle a single word."
)
test.assert_equals(
    generate_hashtag("Codewars      "),
    "#Codewars",
    "Should handle trailing whitespace.",
)
test.assert_equals(
    generate_hashtag("Codewars Is Nice"), "#CodewarsIsNice", "Should remove spaces."
)
test.assert_equals(
    generate_hashtag("codewars is nice"),
    "#CodewarsIsNice",
    "Should capitalize first letters of words.",
)
test.assert_equals(
    generate_hashtag("CodeWars is nice"),
    "#CodewarsIsNice",
    "Should capitalize all letters of words - all lower case but the first.",
)
test.assert_equals(
    generate_hashtag("c i n"),
    "#CIN",
    "Should capitalize first letters of words even when single letters.",
)
test.assert_equals(
    generate_hashtag("codewars  is  nice"),
    "#CodewarsIsNice",
    "Should deal with unnecessary middle spaces.",
)
test.assert_equals(
    generate_hashtag(
        "Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong Cat"  # noqa
    ),
    False,
    "Should return False if the final word is longer than 140 chars.",
)
