use itertools::Itertools;
use std::iter::*;

fn main() {
    let input = std::fs::read_to_string("input/09.txt")
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let mut total = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, v) in row.iter().copied().enumerate() {
            let up = input
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x))
                .copied()
                .unwrap_or(10);
            let down = input
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x))
                .copied()
                .unwrap_or(10);
            let left = input
                .get(y)
                .and_then(|row| row.get(x.wrapping_sub(1)))
                .copied()
                .unwrap_or(10);
            let right = input
                .get(y)
                .and_then(|row| row.get(x.wrapping_add(1)))
                .copied()
                .unwrap_or(10);

            if v < right.min(left).min(down).min(up) {
                total += v + 1;
            }
        }
    }

    println!("{}", total);
}
