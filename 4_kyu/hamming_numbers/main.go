// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Vladimir Rusinov

// package kata implements Hamming Numbers.
// https://www.codewars.com/kata/526d84b98f428f14a60008da/train/go
package kata

func Hammer(n int) uint {
	var hammingNumbers = make([]uint, n)
	hammingNumbers[0] = 1

	var i2, i3, i5 int
	var next2, next3, next5 uint = 2, 3, 5
	for i := 1; i < n; i++ {
		nextHamming := min(next2, next3, next5)
		hammingNumbers[i] = nextHamming

		if nextHamming == next2 {
			i2++
			next2 = hammingNumbers[i2] * 2
		}
		if nextHamming == next3 {
			i3++
			next3 = hammingNumbers[i3] * 3
		}
		if nextHamming == next5 {
			i5++
			next5 = hammingNumbers[i5] * 5
		}
	}

	return hammingNumbers[n-1]
}

func min(a, b, c uint) uint {
	if a <= b && a <= c {
		return a
	}
	if b <= a && b <= c {
		return b
	}
	return c
}
