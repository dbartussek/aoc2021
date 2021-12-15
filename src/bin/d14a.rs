use std::collections::HashMap;

fn step(s: &str, productions: &HashMap<(char, char), char>) -> String {
    s.chars()
        .zip(s.chars().skip(1))
        .map(|(a, b)| match productions.get(&(a, b)).copied() {
            Some(c) => vec![a, c],
            None => vec![a],
        })
        .flatten()
        .chain(s.chars().last())
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input/14.txt").unwrap();
    let mut input = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty());

    let mut string: String = input.next().unwrap().to_owned();

    let productions: HashMap<(char, char), char> = input
        .map(|line| {
            let mut parts = line.split(" -> ");
            let mut source = parts.next().unwrap().chars();
            let result = parts.next().unwrap().chars().next().unwrap();

            ((source.next().unwrap(), source.next().unwrap()), result)
        })
        .collect();

    for i in 0..10 {
        string = step(&string, &productions);
        println!("{}", i);
    }

    let mut count = HashMap::new();

    for c in string.chars() {
        *count.entry(c).or_insert(0u64) += 1;
    }

    let max = count.iter().max_by_key(|e| e.1).unwrap();
    let min = count.iter().min_by_key(|e| e.1).unwrap();

    println!("{}\n{:?} {:?} {}", string.len(), max, min, max.1 - min.1);
}
