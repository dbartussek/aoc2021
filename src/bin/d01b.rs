fn main() {
    let input = std::fs::read_to_string("input/01.txt").unwrap();
    let numbers = input
        .lines()
        .map(|it| it.trim())
        .filter(|it| !it.is_empty())
        .map(|it| it.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut count = 0usize;
    let mut it = numbers
        .windows(3)
        .map(|it| it.iter().sum::<isize>())
        .peekable();

    while let Some(this) = it.next() {
        if let Some(next) = it.peek().copied() {
            if this < next {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
