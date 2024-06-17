use day10::{p1, p2};


fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    p1(divan::black_box(include_str!(
        "../input.txt",
    )));
}

#[divan::bench]
fn part2() {
    p2(divan::black_box(include_str!(
        "../input.txt",
    )));
}

#[divan::bench]
fn part2_github() {
    day10::github::p2(divan::black_box(include_str!(
        "../input.txt",
    )));
}
