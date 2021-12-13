fn main() {
    let input = std::fs::read_to_string("input/10.txt").unwrap();

    let mut points = 0u64;
    let mut complete_points = Vec::<u64>::new();

    let mut stack = Vec::new();

    'lines: for l in input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        stack.clear();

        for c in l.chars() {
            match (c, stack.last().copied()) {
                ('(', _) => {
                    stack.push(')');
                }
                ('[', _) => {
                    stack.push(']');
                }
                ('{', _) => {
                    stack.push('}');
                }
                ('<', _) => {
                    stack.push('>');
                }
                (c, Some(s)) if c == s => {
                    stack.pop();
                }
                (c, _) => {
                    points += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!(),
                    };
                    continue 'lines;
                }
            }
        }

        if !stack.is_empty() {
            let mut this_score = 0;

            for c in stack.iter().rev().copied() {
                this_score *= 5;
                this_score += match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                };
            }

            complete_points.push(this_score);
        }
    }

    let middle_index = complete_points.len() / 2;
    let middle = *complete_points.select_nth_unstable(middle_index).1;

    println!("{} {}", points, middle,);
}
