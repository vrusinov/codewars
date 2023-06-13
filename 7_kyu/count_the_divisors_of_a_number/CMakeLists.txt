# SPDX-FileCopyrightText: 2023 Vladimir Rusinov
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.17)

project(Divisors
    VERSION 1.0
    DESCRIPTION "Count the divisors of a number"
    LANGUAGES CXX)

enable_testing()

# Igloo does not compile without it.
# TODO: fix igloo.
add_compile_options("-fpermissive")

add_library(divisors divisors.cc)
add_executable(divisors_test divisors_test.cc)
target_link_libraries(divisors_test PUBLIC divisors)

# TODO: add ctest to pre-commit.
add_test(BitCountingTest ./divisors_test)
