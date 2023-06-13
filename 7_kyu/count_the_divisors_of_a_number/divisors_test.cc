// SPDX-FileCopyrightText: 2023 Vladimir Rusinov
// SPDX-License-Identifier: Apache-2.0

#include <igloo/igloo_alt.h>
#include "divisors.h"
using namespace igloo;

Describe(Count_divisors)
{
    It(Fixed_tests)
    {
        Assert::That(divisors(1), Equals(1));
        Assert::That(divisors(4), Equals(3));
        Assert::That(divisors(5), Equals(2));
        Assert::That(divisors(12), Equals(6));
        Assert::That(divisors(25), Equals(3));
        Assert::That(divisors(30), Equals(8));
        Assert::That(divisors(4096), Equals(13));
    }
};

int main(int argc, char *argv[])
{
  return TestRunner::RunAllTests(argc, argv);
}
