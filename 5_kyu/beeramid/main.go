// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Vladimir Rusinov

// Package kata implements the Beeramid.
// https://www.codewars.com/kata/51e04f6b544cf3f6550000c1/train/go
package kata


func Beeramid(bonus int, price float64) int {
      bottles := int(float64(bonus) / price)
	  level := 0
	  total := 0
	  for {
		needed := (level + 1) * (level + 1)
		if total + needed > bottles {
			break
		}
		total += needed
		level++
	  }
	  return level
}
