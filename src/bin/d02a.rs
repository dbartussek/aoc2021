fn main() {
    let input = std::fs::read_to_string("input/02.txt").unwrap();
    let mut distance = 0;
    let mut depth = 0;

    for line in input
        .lines()
        .map(|it| it.trim())
        .filter(|it| !it.is_empty())
    {
        let mut components = line.split_ascii_whitespace();
        let direction = components.next().unwrap();
        let magnitude: isize = components.next().unwrap().parse().unwrap();

        match direction {
            "forward" => {
                distance += magnitude;
            }
            "down" => {
                depth += magnitude;
            }
            "up" => {
                depth -= magnitude;
            }
            _ => panic!("Bad direction {}", direction),
        }
    }

    println!(
        "distance: {}, depth: {}, mul: {}",
        distance,
        depth,
        distance * depth
    );
}
