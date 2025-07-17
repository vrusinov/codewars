// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2025 Vladimir Rusinov
//
// Greed is Good
// https://www.codewars.com/kata/5270d0d18625160ada0000e4/train/go

package kata

func Score(dice [5]int) int {
	score := 0

	values := make([]int, 6)
	for _, die := range dice {
		values[die-1]++
	}

	if values[0] >= 3 { // three 1's
		score += 1000
		values[0] -= 3
	} else if values[5] >= 3 { // three 6's
		score += 600
		values[5] -= 3
	} else if values[4] >= 3 { // three 5's
		score += 500
		values[4] -= 3
	} else if values[3] >= 3 { // three 4's
		score += 400
		values[3] -= 3
	} else if values[2] >= 3 { // three 3's
		score += 300
		values[2] -= 3
	} else if values[1] >= 3 { // three 2's
		score += 200
		values[1] -= 3
	}

	return score
}
