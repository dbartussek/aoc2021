#[derive(Debug, Copy, Clone)]
pub struct Bingo(pub [[u8; 5]; 5]);
#[derive(Debug, Default, Copy, Clone)]
pub struct BingoMarkings(pub [[bool; 5]; 5]);

impl Bingo {
    pub fn read<'lt, It>(s: It) -> Self
    where
        It: Iterator<Item = &'lt str>,
    {
        Self(
            s.take(5)
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|n| n.parse::<u8>().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }

    pub fn play(&self, numbers: &[u8]) -> (BingoMarkings, Option<usize>) {
        let mut markings = BingoMarkings::default();

        for (index, number) in numbers.iter().copied().enumerate() {
            if self.mark(number, &mut markings) && markings.has_bingo() {
                return (markings, Some(index));
            }
        }

        (markings, None)
    }

    fn mark(&self, number: u8, markings: &mut BingoMarkings) -> bool {
        let mut marked = false;

        for (x, r) in self.0.iter().enumerate() {
            for (y, f) in r.iter().copied().enumerate() {
                if f == number {
                    markings.0[x][y] = true;
                    marked = true;
                }
            }
        }

        marked
    }

    pub fn sum_unmarked(&self, markings: &BingoMarkings) -> u32 {
        self.0
            .iter()
            .zip(markings.0.iter())
            .map(|(row, mark_row)| row.iter().zip(mark_row.iter()))
            .flatten()
            .map(|(number, mark)| if *mark { 0 } else { *number as u32 })
            .sum()
    }
}

impl BingoMarkings {
    pub fn has_bingo(&self) -> bool {
        for a in 0..5 {
            let mut horizontal_valid = true;
            let mut vertical_valid = true;

            for b in 0..5 {
                if !self.0[a][b] {
                    horizontal_valid = false;
                }
                if !self.0[b][a] {
                    vertical_valid = false;
                }
            }

            if horizontal_valid || vertical_valid {
                return true;
            }
        }

        false
    }
}

pub fn main() {
    let input = std::fs::read_to_string("input/04.txt").unwrap();

    let mut items = input.lines().filter(|s| !s.is_empty()).peekable();
    let numbers = items
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u8>>();

    let mut cards = Vec::new();

    while items.peek().is_some() {
        cards.push(Bingo::read(&mut items));
    }

    let (card, markings, turn) = cards
        .into_iter()
        .filter_map(|card| {
            let (markings, winning_turn) = card.play(&numbers);

            Some((card, markings, winning_turn?))
        })
        .max_by_key(|(_, _, turn)| *turn)
        .unwrap();

    let unmarked = card.sum_unmarked(&markings);

    println!("{}", unmarked * (numbers[turn] as u32));
}
