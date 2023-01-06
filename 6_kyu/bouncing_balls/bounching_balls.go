// SPDX-FileCopyrightText: 2022 Vladimir Rusinov
//
// SPDX-License-Identifier: Apache-2.0

// package kata implements Bouncing Balls
// https://www.codewars.com/kata/5544c7a5cb454edb3c000047/train/go
package kata

func BouncingBall(h, bounce, window float64) int {

	if (bounce >= 1) || (bounce < 0) {
		return -1
	}
	if h < 0 {
		return -1
	}
	if window >= h {
		return -1
	}

	bounces := 1
	current_height := h
	current_height = current_height * bounce
	for current_height > window {
		bounces += 2
		current_height = current_height * bounce
	}
	return bounces
}
