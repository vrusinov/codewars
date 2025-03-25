// SPDX-FileCopyrightText: 2025 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

// package kata implements "Stop gninnipS My sdroW!"
// https://www.codewars.com/kata/5264d2b162488dc400000001/train/go
package kata

import "strings"

func reverse(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func SpinWords(str string) string {

	words := strings.Split(str, " ")
	for i, word := range words {
		if len(word) >= 5 {
			words[i] = reverse(word)
		}
	}

	return strings.Join(words, " ")
}
