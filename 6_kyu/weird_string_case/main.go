package kata

import (
  "unicode"
  "strings"
)

func wordToWeirdCase(word string) string {
  var ret_arr []rune

  for pos, char := range word {
    if (pos % 2 == 0) {
      ret_arr = append(ret_arr, unicode.ToUpper(char));
    } else {
      ret_arr = append(ret_arr, unicode.ToLower(char));
    }
  }

  return string(ret_arr);
}

func toWeirdCase(str string) string {
  var weird_words []string

  words := strings.Fields(str);

  for _, word := range words {
    weird_words = append(weird_words, wordToWeirdCase(word))
  }

  return strings.Join(weird_words, " ");
}
