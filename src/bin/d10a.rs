fn main() {
    let input = std::fs::read_to_string("input/10.txt").unwrap();

    let mut points = 0u64;

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
    }

    println!("{}", points);
}
