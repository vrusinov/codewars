// SPDX-FileCopyrightText: 2025 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0
//
// https://www.codewars.com/kata/51e0007c1f9378fa810002a9/train/rust
// Make the Deadfish Swim

fn parse(code: &str) -> Vec<i32> {
    let mut value: i32 = 0;
    let mut result: Vec<i32> = vec![];

    for c in code.chars() {
        match c {
            'i' => value += 1,
            'd' => value -= 1,
            's' => value *= value,
            'o' => result.push(value),
            _ => (),
        }
    }

    return result;
}

#[cfg(test)]
mod example_programs {
    use super::parse;

    #[test]
    fn example_iiisdoso() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
    }

    #[test]
    fn example_iiisxxxdoso() {
        assert_eq!(parse("iiisxxxdoso"), vec![8, 64]);
    }
}
