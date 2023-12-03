use year_2023_day_01::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input/input1.txt",))).unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../input/input2.txt",))).unwrap();
}