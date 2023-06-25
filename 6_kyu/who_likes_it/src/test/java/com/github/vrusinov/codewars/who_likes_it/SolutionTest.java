/*
 * https://www.codewars.com/kata/5266876b8f4bf2da9b000362/train/java
 *
 *SPDX-FileCopyrightText: 2023 Vladimir Rusinov
 * SPDX-License-Identifier: Apache-2.0
 */

package com.github.vrusinov.codewars.who_likes_it;

import org.junit.Test;
import static org.junit.Assert.assertEquals;
import org.junit.runners.JUnit4;

public class SolutionTest {
    @Test
    public void staticTests() {
        assertEquals("no one likes this", Solution.whoLikesIt());
        assertEquals("Peter likes this", Solution.whoLikesIt("Peter"));
        assertEquals("Jacob and Alex like this", Solution.whoLikesIt("Jacob", "Alex"));
        assertEquals("Max, John and Mark like this", Solution.whoLikesIt("Max", "John", "Mark"));
        assertEquals("Alex, Jacob and 2 others like this", Solution.whoLikesIt("Alex", "Jacob", "Mark", "Max"));
    }
}
