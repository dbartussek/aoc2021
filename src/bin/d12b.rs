use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input/12.txt").unwrap();
    let edges = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut s = s.split('-');
            let start = s.next().unwrap();
            let end = s.next().unwrap();
            (start, end)
        });

    let mut graph: HashMap<&str, HashSet<&str>> = Default::default();

    for (start, end) in edges {
        let node = graph.entry(start).or_insert_with(|| Default::default());
        node.insert(end);

        let node = graph.entry(end).or_insert_with(|| Default::default());
        node.insert(start);
    }

    let mut stack: Vec<(Vec<&str>, bool)> = vec![(vec!["start"], false)];
    let mut options = 0usize;

    while let Some((current, has_visited_twice)) = stack.pop() {
        for next in graph.get(current.last().unwrap()).unwrap() {
            if *next == "end" {
                options += 1;
            } else if *next == "start" {
            } else if next.chars().next().unwrap().is_uppercase()
                || !has_visited_twice
                || !current.contains(next)
            {
                stack.push((
                    current.iter().copied().chain(Some(*next)).collect(),
                    (has_visited_twice
                        || (!next.chars().next().unwrap().is_uppercase()
                            && current.contains(next))),
                ));
            }
        }
    }

    println!("{}", options);
}
