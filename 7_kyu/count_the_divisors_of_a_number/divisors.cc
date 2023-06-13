// https://www.codewars.com/kata/542c0f198e077084c0000c2e/train/cpp

// SPDX-FileCopyrightText: 2023 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

int divisors(int n) {
    int divisors = 1; // Always divisible by self
    for (int divisor = n / 2; divisor > 0; divisor--) {
        if (n % divisor == 0) {
            divisors++;
        }
    }

  return divisors;
}
