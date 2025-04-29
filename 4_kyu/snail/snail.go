// SPDX-FileCopyrightText: 2025 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

// kata package implements Snail
// https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1/train/go
package kata

func Snail(snaipMap [][]int) []int {
	// There is a bug in tests - they pass {{}} instead of {}
	if len(snaipMap) == 1 && len(snaipMap[0]) == 0 {
		return []int{}
	}

	ret := make([]int, 0, len(snaipMap)*len(snaipMap))

	min_i := 0
	max_i := len(snaipMap) - 1
	min_j := 0
	max_j := len(snaipMap) - 1

	for {
		size := max_i - min_i + 1
		if size <= 0 {
			return ret
		}
		if size == 1 {
			ret = append(ret, snaipMap[min_i][min_j])
			return ret
		}

		// horizontal pass
		i := min_i
		for j := min_j; j <= max_j; j++ {
			ret = append(ret, snaipMap[i][j])
		}
		min_i++
		// vertical pass
		j := max_j
		for i := min_i; i <= max_i; i++ {
			ret = append(ret, snaipMap[i][j])
		}
		max_j--
		// horizontal reverse:
		i = max_i
		for j := max_j; j >= min_j; j-- {
			ret = append(ret, snaipMap[i][j])
		}
		max_i--
		// vertical reverse
		j = min_j
		for i := max_i; i >= min_i; i-- {
			ret = append(ret, snaipMap[i][j])
		}
		min_j++
	}
}
