// SPDX-FileCopyrightText: 2025 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

// Detect Pangram
// https://www.codewars.com/kata/545cedaa9943f7fe7b000048/train/rust

use std::collections::HashSet;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn is_pangram(s: &str) -> bool {
    let mut unseen = ALPHABET.chars().collect::<HashSet<char>>();;
    for c in s.to_lowercase().chars() {
        unseen.remove(&c);
    }
    return unseen.is_empty();
}

#[cfg(test)]
mod tests {
    use super::is_pangram;

    fn dotest(s: &str, expected: bool) {
        let actual = is_pangram(s);
        assert!(actual == expected, "Test failed with s = \"{s}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        dotest("The quick, brown fox jumps over the lazy dog!", true);
        dotest("Cwm fjord bank glyphs vext quiz", true);
        dotest("Pack my box with five dozen liquor jugs.", true);
        dotest("How quickly daft jumping zebras vex.", true);
        dotest("ABCD45EFGH,IJK,LMNOPQR56STUVW3XYZ", true);
        dotest("This isn't a pangram!", false);
        dotest("abcdefghijklmopqrstuvwxyz", false);
        dotest("Aacdefghijklmnopqrstuvwxyz", false);
    }
}
