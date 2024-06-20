# "Zero cost abtractions" in Rust and C++ - a performance comparison by benchmark

This repository contains solutions to the AoC 2023 day 10 problem in Rust and C++.

The solutions are

| Bench mark name | Source |
------------------|---------
| **criterion/part1_rust** | Own solution for part 1 in Rust |
| **criterion/part1_cpp** | Own solution in C++ |
| criterion/part2 | Own solution for part 2 in Rust |
| criterion/part2_github | Rust Solution taken from Github |
| criterion/part2_cpp_yt | C++ Solution taken from YT video |

Rust compiler version 1.79.0, clang 15 (macOS), gcc-12 (Linux), msvc Community 2022 (Windows 11 on ARM)

The benchmark results for **criterion/part1_rust** and **criterion/part1_cpp** show that C++ fails to deliver the same performance as Rust. The Rust solution is about 5 times faster (on a Mac mini M1) than the C++ solution mycpp, which uses the same abstractions as the Rust solution is using, just by C++20 means. Both solutions use Optionals, Tuples, slices (spans) and vectors, try hard to avoid unnecessary data copies by passing data around as constant references and follow the same algorithm.

I am pretty sure that more efficient C++ solutions than those given here are possible, but my bet is that those will look more like C than C++.

## Other lessons learned

The Rust compiler on Windows 11 on ARM produces slower code for the Rust-Github-Solution and substantially slower code for my Rust solutions. But both C++ solutions are substantially worse than the Rust solutions on all platforms tried (macOS, Ubunutu 24.04, Window 11 on ARM - Linux and Windows running in a Parallels VM on a Mac mini M1)

