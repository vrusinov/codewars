// SPDX-FileCopyrightText: 2025 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

// package kata implements "Two Sum"
// https://www.codewars.com/kata/52c31f8e6605bcc646000082/train/go
package kata

func TwoSum(numbers []int, target int) [2]int {
	// First create a map from the numbers to their indices.
	// This way we can look up the index of a number in O(1) time.

	var indices = make(map[int][]int, len(numbers))

	for i, num := range numbers {
		indices[num] = append(indices[num], i)
	}

	// Now we iterate over original array.
	// For every index we'll check if there is a suitable pairing.

	for i, num := range numbers {
		pairs, ok := indices[target-num]
		if !ok {
			continue
		}
		// Check if we have an index which is not i in potential pairs
		for _, j := range pairs {
			if i != j {
				return [2]int{i, j}
			}
		}
	}

	// By the problem statement we should never reach this point.
	return [2]int{}
}
