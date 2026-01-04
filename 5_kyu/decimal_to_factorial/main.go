// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Vladimir Rusinov

// Package kata implements Decimal to Factorial and Back
// https://www.codewars.com/kata/54e320dcebe1e583250008fd/train/go
package kata

func Dec2FactString(nb int) string {
  ret := ""
  i := 1
  for nb > 0 {
	val := nb % i
	var c byte
	if val < 10 {
		c = byte('0' + val)
	} else {
		c = byte('A' + (val - 10))
	}
	ret = string(c) + ret
	nb = nb / i
	i++
  }
  return ret
}

func FactString2Dec(str string) int {
  ret := 0
  pos := 0
  factorial := 1

  for i := len(str) - 1 ; i >= 0; i-- {
	c := str[i]
	val := 0
	if c >= '0' && c <= '9' {
		val = int(c - '0')
	} else {
		val = int(c - 'A' + 10)
	}
	ret += val * factorial
	pos++
	factorial *= pos
  }
  return ret
}
