// SPDX-FileCopyrightText: 2022 Vladimir Rusinov
//
// SPDX-License-Identifier: Apache-2.0

fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_vec: Vec<i32> = good
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let evil_vec: Vec<i32> = evil
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Hobbits, Men, Elves, Dwarves, Eagles, Wizards.
    // 1        2    3      3        4       10
    let good_score =
        good_vec[0]*1 +
        good_vec[1]*2 +
        good_vec[2]*3 +
        good_vec[3]*3 +
        good_vec[4]*4 +
        good_vec[5]*10;

    // Orcs, Men, Wargs, Goblins, Uruk Hai, Trolls, Wizards.
    // 1     2    2      2        3         5       10
    let evil_score =
        evil_vec[0]*1 +
        evil_vec[1]*2 +
        evil_vec[2]*2 +
        evil_vec[3]*2 +
        evil_vec[4]*3 +
        evil_vec[5]*5 +
        evil_vec[6]*10;

    if good_score > evil_score {
        return String::from("Battle Result: Good triumphs over Evil");
    } else if good_score == evil_score {
        return String::from("Battle Result: No victor on this battle field");
    }
    String::from("Battle Result: Evil eradicates all trace of Good")
}

  #[test]
  fn returns_expected() {
      assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"), "Battle Result: Good triumphs over Evil");
      assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"), "Battle Result: Evil eradicates all trace of Good");
      assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"), "Battle Result: No victor on this battle field");
  }
