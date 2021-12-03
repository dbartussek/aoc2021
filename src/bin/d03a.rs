fn main() {
    let input = include_str!("../../input/03.txt");

    let mut counters = Vec::<usize>::new();
    let mut total = 0usize;

    for line in input.lines() {
        total += 1;

        for (index, c) in line.chars().enumerate() {
            if counters.len() <= index {
                counters.push(0);
            }

            if c == '1' {
                counters[index] += 1;
            }
        }
    }

    let mut g = 0usize;
    let mut e = 0usize;

    for it in counters {
        g <<= 1;
        e <<= 1;

        if it > total / 2 {
            g |= 1;
        } else {
            e |= 1;
        }
    }

    println!("{} {} {}", g, e, g * e);
}
