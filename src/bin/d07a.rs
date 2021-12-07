#![feature(int_abs_diff)]

use itertools::*;

fn main() {
    let input: Vec<u32> = std::fs::read_to_string("input/07.txt")
        .unwrap()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let (min, max) = match input.iter().copied().minmax() {
        MinMaxResult::MinMax(a, b) => (a, b),
        _ => panic!(),
    };

    let min_distance = (min..=max)
        .map(|destination| {
            input
                .iter()
                .copied()
                .map(|pos| pos.abs_diff(destination))
                .sum::<u32>()
        })
        .min()
        .unwrap();
    println!("{}", min_distance);
}
