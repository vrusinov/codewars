fn solution(word: &str, ending: &str) -> bool {
    return word.ends_with(ending);
}

#[test]
fn returns_expected() {
  assert_eq!(true, solution("abc", "c"));
  assert_eq!(false, solution("strawberry", "banana"));
}
