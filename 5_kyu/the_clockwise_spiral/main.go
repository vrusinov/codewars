// SPDX-FileCopyrightText: 2026 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

// package kata implements The Clockwise Spiral problem.
// https://www.codewars.com/kata/536a155256eb459b8700077e/train/go
package kata

func CreateSpiral(n int) [][]int {
	if n <= 0 {
		return [][]int{}
	}

	var ret [][]int = make([][]int, n)
	for i := range ret {
		ret[i] = make([]int, n)
	}

	i := 0
	j := 0
	dx := 1
	dy := 0

	for k := 1; k <= n*n; k++ {
		ret[j][i] = k
		if ret[(j+dy+n)%n][(i+dx+n)%n] != 0 {
			dx, dy = -dy, dx
		}
		i += dx
		j += dy
	}

	return ret
}
