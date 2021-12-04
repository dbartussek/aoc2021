fn partition_bit<'lt>(numbers: &[&'lt str], index: usize) -> (Vec<&'lt str>, Vec<&'lt str>) {
    let mut z = Vec::new();
    let mut o = Vec::new();

    for s in numbers.iter().copied() {
        if s.chars().nth(index).unwrap() == '0' {
            z.push(s);
        } else {
            o.push(s);
        }
    }

    (z, o)
}

fn parse_bits(s: &str) -> u64 {
    let mut acc = 0;

    for b in s.chars() {
        acc <<= 1;
        if b == '1' {
            acc |= 1;
        }
    }

    acc
}

fn main() {
    let input = include_str!("../../input/03.txt");
    let lines = input.lines().collect::<Vec<_>>();

    let number_length = lines.first().unwrap().len();

    let mut common = lines.clone();
    let mut uncommon = lines;

    for i in 0..number_length {
        let (common_z, common_o) = partition_bit(&common, i);
        common = if common_z.len() > common_o.len() {
            common_z
        } else {
            common_o
        };

        let (uncommon_z, uncommon_o) = partition_bit(&uncommon, i);
        uncommon = if uncommon_o.is_empty() {
            uncommon_z
        } else if uncommon_z.is_empty() {
            uncommon_o
        } else if uncommon_z.len() <= uncommon_o.len() {
            uncommon_z
        } else {
            uncommon_o
        };
    }

    assert_eq!(common.len(), 1);
    assert_eq!(uncommon.len(), 1);

    let ox = parse_bits(common[0]);
    let co = parse_bits(uncommon[0]);

    println!("{} {} {}", ox, co, ox * co);
}
