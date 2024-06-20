# "Zero cost abtractions" in Rust and C++ - a performance comparison by benchmark

This repository contains solutions to the AoC 2023 day 10 problem in Rust and C++.

The solutions are

| Bench mark name | Source |
------------------|---------
| **criterion/part1** | Own solution for part1 in Rust |
| divan/part1     | dto. just measured with Divan |
| criterion/part2 | Own solution for part 2 in Rust |
| divan/part2     | dto. just measured with Divan |
| criterion/part2_github | Rust Solution taken from Github |
| **criterion/part1_mycpp** | Own solution in C++ |
| criterion/part2_cpp | C++ Solution taken from YT video |

The benchmark results for **criterion/part1** and **criterion/part1_mycpp** show that C++ fails to deliver the same performance as Rust. The Rust solution is about 5 times faster (on a Mac mini M1) than the C++ solution mycpp, which uses the same abstractions as the Rust solution is using, just by C++20 means. Both solutions use Optionals, Tuples, slices (spans) and vectors, try hard to avoid unnecessary data copies by passing data around as constant references and follow the same algorithm.

I am pretty sure that more efficient C++ solutions are possible, but my bet is that those will look more like C than C++.

