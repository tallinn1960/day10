# "Zero cost abtractions" in Rust, C++ and Swift 
> "When it comes to performance, don’t trust your intuition: measure" 
Bjarne Stroustrup, "A tour of C++", 3rd edition


This repository contains solutions to the AoC 2023 day 10 problem in Rust and C++.

The solutions are

| Bench mark name | Source | Benchmark (Criterion/Mac mini M1)|
------------------|---------|-|
| **criterion/part1_rust** | Own solution for part 1 in Rust |60 µs|
| **criterion/part1_cpp** | Own solution for part 1 in C++ | 224 µs|
| criterion/part1_swift | Own solution in Swift (only on macOS) | 44 ms|
| criterion/part2 | Own solution for part 2 in Rust |74 µs|
| criterion/part2_github | Rust Solution taken from Github |89 µs|

Rust compiler version 1.79.0, clang 15/Swift 5.10 (macOS), gcc-12 (Linux), msvc Community 2022 (Windows 11 on ARM)

Readers may notice that the solutions for part one do more than what is required to solve part one of the AoC problem. For Rust, it turned out that computing the loop and return its locations as a vector is faster than just counting steps. So I designed all other implementations the same way. Having a vector of all locations of the loop helps to solve part 2 in a very efficient way.

The benchmark results for **criterion/part1_rust** and **criterion/part1_cpp** show that C++ fails to deliver the same performance as Rust. The Rust solution is about 5 times faster (*) than the C++ solution mycpp, which uses the same abstractions as the Rust solution is using, just by C++20 means. Both solutions use Optionals, Tuples, slices (spans) and vectors, try hard to avoid unnecessary data copies by passing data around as constant references and follow the same algorithm.

I am pretty sure that more efficient C++ solutions than those given here are possible, but my bet is that those will look more like C than C++.

(*) on a Mac mini M1 running MacOS Sonoma, it's better on Linux where the Rust code is only three times faster than the code delivered by gcc-12, unsurprising as gcc appears to produce better arm64 code than clang in general.

## Other lessons learned - Swift is embarrassingly slow

I added a Swift solution for macOS (it won't be compiled and benchmarked on other platforms). Rust code is about **1000** times **(!)** faster than the Swift solution - µs instead of ms. The Swift solution uses the same algorithm with the same abstractions as the Rust solution. The Criterion benchmark is confirmed by the Swift XCTest benchmark (run with `swift test -c release` in the *Day10Swift* directory). I bet one can create a
pure Python solution and run it faster on cpython (i.e. no JIT) or a faster javascript solution running on node.js. For a compiled language like Swift, this is embarrassing.

The Rust compiler on Windows 11 on ARM produces slower code for the Rust-Github-Solution and substantially slower code for my Rust solutions. On Linux gcc-12 produces faster code than gcc-11. But both C++ solutions are substantially worse than the Rust solutions on all platforms with all available compilers tried (macOS Sonoma, Ubunutu 24.04, Window 11 on ARM 23H2 - Linux and Windows running in a Parallels VM on a Mac mini M1).

## macOS and gcc

If you want to try the most recent *gcc* version available by **HomeBrew** on macOS (gcc 14 at the time of writing) on this repo, you can install it and run the benchmarks with the following commands:

```bash
brew install gcc
cargo clean
CC=gcc-14 CXX=g++-14 CXXSTDLIB=stdc++ RUSTFLAGS=-L/opt/homebrew/opt/gcc/lib/gcc/current/ cargo bench
```

## Note for Windows users

Make sure that cmake is in your %PATH%. There is a cmake coming with MS Visual Studio 2022 Community Edition. It's in `C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin`.