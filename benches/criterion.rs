// a criterion benchmark for p2, p2_reverse, and p2_maps

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use day10::{p1, p2};
use std::fs::File;
use std::io::Read;

fn bench_p1(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part1_rust", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| p1(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

fn bench_p1_cpp(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part1_cpp", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| unsafe {day10::day10cpp::run_p1(f.as_ptr(), f.len())},
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

#[cfg(target_os = "macos")]
fn bench_p1_swift(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part1_swift", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| unsafe {day10::day10swift::p1(f.as_ptr(), f.len())},
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

fn bench_p2(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part2_rust", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| p2(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish()
}


fn bench_p2_github(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part2_rust_github", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| day10::github::p2(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish()
}


#[cfg(target_os = "macos")]
criterion_group!(benches, bench_p1, 
    bench_p1_swift, 
    bench_p1_cpp, bench_p2, bench_p2_github);

#[cfg(not(target_os = "macos"))]
criterion_group!(benches, bench_p1, 
    bench_p1_cpp, bench_p2, bench_p2_github);

criterion_main!(benches);
