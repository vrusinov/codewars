/*
 * https://www.codewars.com/kata/5266876b8f4bf2da9b000362/train/java
 *
 *SPDX-FileCopyrightText: 2023 Vladimir Rusinov
 * SPDX-License-Identifier: Apache-2.0
 */

package com.github.vrusinov.codewars.who_likes_it;

class Solution {
    public static String whoLikesIt(String... names) {
        if (names.length == 0) {
            return "no one likes this";
        }

        if (names.length == 1) {
            return String.format("%s likes this", names[0]);
        }

        if (names.length == 2) {
            return String.format("%s and %s like this", names[0], names[1]);
        }

        if (names.length == 3) {
            return String.format("%s, %s and %s like this", names[0], names[1], names[2]);
        }

        return String.format("%s, %s and %d others like this", names[0], names[1], names.length-2);
    }
}
