# SPDX-FileCopyrightText: 2023 Vladimir Rusinov
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.17)

project(BitCounting
    VERSION 1.0
    DESCRIPTION "Bit Counting"
    LANGUAGES CXX)

enable_testing()

# Igloo does not compile without it.
# TODO: fix igloo.
add_compile_options("-fpermissive")

add_library(bit_counting bit_counting.cc)
add_executable(bit_counting_test bit_counting_test.cc)
target_link_libraries(bit_counting_test PUBLIC bit_counting)

# TODO: add ctest to pre-commit.
add_test(BitCountingTest ./bit_counting_test)
