// SPDX-FileCopyrightText: 2022 Vladimir Rusinov
//
// SPDX-License-Identifier: Apache-2.0

package kata

import (
  "unicode"
  "strings"
)

func wordToWeirdCase(word string) string {
  var retArr []rune

  for pos, char := range word {
    if (pos % 2 == 0) {
      retArr = append(retArr, unicode.ToUpper(char));
    } else {
      retArr = append(retArr, unicode.ToLower(char));
    }
  }

  return string(retArr);
}

func toWeirdCase(str string) string {  // nolint: deadcode
  var weirdWords []string

  words := strings.Fields(str);

  for _, word := range words {
    weirdWords = append(weirdWords, wordToWeirdCase(word))
  }

  return strings.Join(weirdWords, " ");
}
