use itertools::Itertools;
use std::iter::*;

fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x.wrapping_sub(1), y),
        (x.wrapping_add(1), y),
        (x, y.wrapping_sub(1)),
        (x, y.wrapping_add(1)),
    ]
}

fn main() {
    let input = std::fs::read_to_string("input/09.txt")
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let mut basin_map = vec![vec![0usize; input.len()]; input[0].len()];

    let mut current_basin = 1;

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if basin_map[y][x] == 0 && input[y][x] < 9 {
                basin_map[y][x] = current_basin;
                let mut stack = neighbors(x, y);

                while let Some((x, y)) = stack.pop() {
                    if let Some(basin) = basin_map
                        .get_mut(y)
                        .and_then(|row| row.get_mut(x))
                        .filter(|basin| **basin == 0 && input[y][x] < 9)
                    {
                        *basin = current_basin;
                        stack.extend(neighbors(x, y));
                    }
                }

                current_basin += 1;
            }
        }
    }

    let mut counters = vec![0usize; current_basin];

    for basin in basin_map.iter().map(|row| row.iter()).flatten().copied() {
        counters[basin] += 1;
    }

    println!("{:?}", counters);
    counters[1..].sort_unstable();
    println!("{:?}", counters);

    println!(
        "{}",
        counters
            .iter()
            .rev()
            .take(3)
            .copied()
            .reduce(|a, b| a * b)
            .unwrap()
    );
}
