// package kata implements "RGB To Hex Conversion" problem
// https://www.codewars.com/kata/513e08acc600c94f01000001/train/go
//
// SPDX-FileCopyrightText: 2025 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0
package kata

import "fmt"

func clamp(n int) int {
	if n < 0 {
		return 0
	}
	if n > 255 {
		return 255
	}
	return n
}

func RGB(r, g, b int) string {
	return fmt.Sprintf("%02X%02X%02X", clamp(r), clamp(g), clamp(b))
}
