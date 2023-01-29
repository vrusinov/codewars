// SPDX-FileCopyrightText: 2023 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

unsigned int countBits(unsigned long long n){
  return __builtin_popcountll(n);
}
