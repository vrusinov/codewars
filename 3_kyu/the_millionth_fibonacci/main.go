
// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Vladimir Rusinov

// package kata implements The Millionth Fibonacci kata.
// https://www.codewars.com/kata/53d40c1e2f13e331fc000c26/train/go
package kata

import "math/big"

func fib(n int64) (*big.Int){
	p := 1;
	if n < 0 && n%2 == 0{
		p = -1
	}
	if n < 0 {
		n = -n
	}
	if n == 0 {
		return big.NewInt(0)
	}
	if n == 1 {
		return big.NewInt(1)
	}
	if n == 2 {
		return big.NewInt(1)
	}
	k := n / 2
	a := fib(k)
	b := fib(k + 1)
	// https://www.nayuki.io/page/fast-fibonacci-algorithms
	if n%2 == 0 {
		// F(2k) = F(k)[2F(k+1) − F(k)]
		return new(big.Int).Mul(big.NewInt(int64(p)), new(big.Int).Mul(a, new(big.Int).Sub(new(big.Int).Mul(big.NewInt(2), b), a)))
	} else {
		// F(2k + 1) = F(k+1)^2 + F(k)^2
		return new(big.Int).Mul(big.NewInt(int64(p)), new(big.Int).Add(new(big.Int).Mul(a, a), new(big.Int).Mul(b, b)))
	}
}
