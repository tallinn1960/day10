use day10::{p1, p2};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1_rust() {
    p1(divan::black_box(include_str!("../input.txt",)));
}

#[cfg(target_os = "macos")]
#[divan::bench]
fn part1_swift() {
    let input = divan::black_box(include_str!("../input.txt",));
     day10::day10swift::p1_swift(input);
}

#[divan::bench]
fn part1_cpp() {
    let input = divan::black_box(include_str!("../input.txt",));
    day10::day10cpp::p1_cpp(input);
}

#[divan::bench]
fn part2_rust() {
    p2(divan::black_box(include_str!("../input.txt",)));
}
