// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Vladimir Rusinov

// Hamming Numbers
// https://www.codewars.com/kata/526d84b98f428f14a60008da/train/rust

fn hamming(n: usize) -> u64 {
    let mut hamming_numbers = vec![1u64];
    let (mut i2, mut i3, mut i5) = (0, 0, 0);

    while hamming_numbers.len() < n {
        let next_2 = hamming_numbers[i2] * 2;
        let next_3 = hamming_numbers[i3] * 3;
        let next_5 = hamming_numbers[i5] * 5;

        let next_hamming = next_2.min(next_3.min(next_5));
        hamming_numbers.push(next_hamming);

        if next_hamming == next_2 {
            i2 += 1;
        }
        if next_hamming == next_3 {
            i3 += 1;
        }
        if next_hamming == next_5 {
            i5 += 1;
        }
    }

    hamming_numbers[n - 1]
}

#[cfg(test)]
mod tests {
    use super::hamming;

    #[test]
    fn sample_tests() {
        assert_eq!(hamming(1), 1);
        assert_eq!(hamming(2), 2);
        assert_eq!(hamming(3), 3);
        assert_eq!(hamming(4), 4);
        assert_eq!(hamming(5), 5);
        assert_eq!(hamming(6), 6);
        assert_eq!(hamming(7), 8);
        assert_eq!(hamming(8), 9);
        assert_eq!(hamming(9), 10);
        assert_eq!(hamming(10), 12);
        assert_eq!(hamming(11), 15);
        assert_eq!(hamming(12), 16);
        assert_eq!(hamming(13), 18);
        assert_eq!(hamming(14), 20);
        assert_eq!(hamming(15), 24);
        assert_eq!(hamming(16), 25);
        assert_eq!(hamming(17), 27);
        assert_eq!(hamming(18), 30);
        assert_eq!(hamming(19), 32);
    }
}
