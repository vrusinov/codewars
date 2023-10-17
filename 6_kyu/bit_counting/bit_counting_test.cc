// SPDX-FileCopyrightText: 2023 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

#include <igloo/igloo_alt.h>
#include "bit_counting.h"
using namespace igloo;

Describe(CountBits){
  It(BasicTests){
    Assert::That(countBits(0), Equals(0));
    Assert::That(countBits(4), Equals(1));
    Assert::That(countBits(7), Equals(3));
    Assert::That(countBits(9), Equals(2));
    Assert::That(countBits(10), Equals(2));
    Assert::That(countBits(26), Equals(3));
    Assert::That(countBits(77231418), Equals(14));
    Assert::That(countBits(12525589), Equals(11));
    Assert::That(countBits(3811), Equals(8));
    Assert::That(countBits(392902058), Equals(17));
    Assert::That(countBits(7027622455159733985), Equals(32));
  }
};

int main(int argc, const char *argv[])
{
  return TestRunner::RunAllTests(argc, argv);
}
