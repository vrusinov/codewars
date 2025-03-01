// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2025 Vladimir Rusinov
//
// https://www.codewars.com/kata/52685f7382004e774f0001f7/train/go
// Human Readable Time
// Write a function, which takes a non-negative integer (seconds) as input and returns the time in a human-readable format (HH:MM:SS)

package kata

import "fmt"

func HumanReadableTime(seconds int) string {
	hours := seconds / 3600
	remainder := seconds % 3600
	minutes := remainder / 60
	seconds = remainder % 60
	return fmt.Sprintf("%02d:%02d:%02d", hours, minutes, seconds)
}
