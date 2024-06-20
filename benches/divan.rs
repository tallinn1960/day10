use day10::{p1, p2};


fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1_rust() {
    p1(divan::black_box(include_str!(
        "../input.txt",
    )));
}

#[divan::bench]
fn part1_cpp() {
    day10::ffi::p1_cpp(divan::black_box(include_str!(
        "../input.txt",
    )));
}


#[divan::bench]
fn part2_rust() {
    p2(divan::black_box(include_str!(
        "../input.txt",
    )));
}

#[divan::bench]
fn part2_rust_github() {
    day10::github::p2(divan::black_box(include_str!(
        "../input.txt",
    )));
}

#[divan::bench]
fn part2_cpp_yt() {
    day10::github::p2(divan::black_box(include_str!(
        "../input.txt",
    )));
}