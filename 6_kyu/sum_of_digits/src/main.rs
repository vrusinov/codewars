fn sum_of_digits(n: i64) -> i64 {
    let mut result: i64 = 0;
    let mut reminder: i64 = n;
    while reminder >= 10 {
        result += reminder % 10;
        reminder = reminder / 10;
    }
    result += reminder;
    return result;
}

fn digital_root(n: i64) -> i64 {
    println!("{}", n);
    let mut result: i64 = n;
    while result >= 10 {
        result = sum_of_digits(result);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
      assert_eq!(digital_root(16), 7);
      assert_eq!(digital_root(942), 6);
      assert_eq!(digital_root(132189), 6);
      assert_eq!(digital_root(493193), 2);

      assert_eq!(digital_root(1), 1);
      assert_eq!(digital_root(9), 9);
      assert_eq!(digital_root(0), 0);
      assert_eq!(digital_root(11), 2);
      assert_eq!(digital_root(29), 2);
      assert_eq!(digital_root(10), 1);
    }
}