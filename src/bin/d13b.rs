use std::collections::HashSet;

fn fold<const N: usize>(
    points: &HashSet<[u16; N]>,
    (along, at): (usize, u16),
) -> HashSet<[u16; N]> {
    let mut result = HashSet::with_capacity(points.len());

    for mut p in points.iter().copied() {
        if p[along] == at {
            continue;
        }
        if p[along] > at {
            p[along] = at - (p[along] - at);
        }

        result.insert(p);
    }

    result
}

fn to_string(points: &HashSet<[u16; 2]>) -> String {
    let x = points.iter().map(|p| p[0]).max().unwrap() as usize;
    let y = points.iter().map(|p| p[1]).max().unwrap() as usize;

    let mut s = String::with_capacity(x * y);

    for y in 0..=y {
        for x in 0..=x {
            if points.contains(&[x as u16, y as u16]) {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }

    s
}


fn main() {
    let input = std::fs::read_to_string("input/13.txt").unwrap();

    let mut points = HashSet::new();
    let mut folds = Vec::new();

    let fold_prefix = "fold along ";
    for line in input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if line.starts_with(fold_prefix) {
            let line = &line[fold_prefix.len()..];
            let mut parts = line.split('=');

            folds.push((
                match parts.next().unwrap() {
                    "x" => 0usize,
                    "y" => 1usize,
                    _ => unreachable!(),
                },
                parts.next().unwrap().parse::<u16>().unwrap(),
            ));
        } else {
            let mut coords = line.split(',');
            points.insert([
                coords.next().unwrap().parse::<u16>().unwrap(),
                coords.next().unwrap().parse::<u16>().unwrap(),
            ]);
        }
    }

    for f in folds {
        points = fold(&points, f);
    }
    println!("{}\n{}", points.len(), to_string(&points));
}