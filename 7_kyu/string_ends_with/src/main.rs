// SPDX-FileCopyrightText: 2022 Vladimir Rusinov
//
// SPDX-License-Identifier: Apache-2.0

fn solution(word: &str, ending: &str) -> bool {
    return word.ends_with(ending);
}

#[test]
fn returns_expected() {
  assert_eq!(true, solution("abc", "c"));
  assert_eq!(false, solution("strawberry", "banana"));
}
