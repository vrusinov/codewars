fn find_missing(seq: &[i32]) -> i32 {
    let len = seq.len();
    let slope: i32 = (seq[len-1] - seq[0]) / (len as i32);
    let mut expected: i32 = seq[0];

    for s in seq.iter() {
        if *s != expected {
            return expected;
        }
        expected += slope;
    }
    return 0; // Should not happen under well-formed input
}

#[cfg(test)]
mod tests {
    use super::find_missing;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[i32], expected: i32) {
        assert_eq!(find_missing(a), expected, "{ERR_MSG} with seq = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 6, 7, 8, 9], 5);
        dotest(&[1, 3, 4, 5, 6, 7, 8, 9], 2);
    }
    #[test]
    fn tests2() {
        dotest(&[2, 4, 6, 10], 8);
    }
}
