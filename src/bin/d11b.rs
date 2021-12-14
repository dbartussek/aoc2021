use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
pub struct Octopussi(pub [[u8; 10]; 10]);

fn neighbors(x: usize, y: usize) -> [(usize, usize); 8] {
    [
        (x.wrapping_sub(1), y.wrapping_sub(1)),
        (x.wrapping_sub(1), y.wrapping_add(1)),
        (x.wrapping_add(1), y.wrapping_sub(1)),
        (x.wrapping_add(1), y.wrapping_add(1)),
        //
        (x.wrapping_sub(1), y),
        (x.wrapping_add(1), y),
        //
        (x, y.wrapping_sub(1)),
        (x, y.wrapping_add(1)),
    ]
}

impl Octopussi {
    const FLASH: u8 = 10;

    pub fn read(s: &str) -> Self {
        let mut this = Self(Default::default());

        for (row, line) in this.0.iter_mut().zip(s.lines()) {
            for (octo, c) in row.iter_mut().zip(line.chars()) {
                *octo = c.to_digit(10).unwrap() as u8;
            }
        }

        this
    }

    pub fn to_string(&self) -> String {
        self.0.iter().map(|r| r.iter().join("")).join("\n")
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.0.get(x).and_then(|r| r.get(y)).copied()
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        self.0.get_mut(x).and_then(|r| r.get_mut(y))
    }

    pub fn flash(mut self) -> (Self, usize) {
        for i in self.0.iter_mut().map(|row| row.iter_mut()).flatten() {
            *i += 1;
        }

        let mut stack = Vec::<(usize, usize)>::with_capacity(100);

        for x in 0..10 {
            for y in 0..10 {
                if self.get(x, y).unwrap() == Self::FLASH {
                    stack.push((x, y));
                }
            }
        }

        while let Some((x, y)) = stack.pop() {
            for (nx, ny) in neighbors(x, y) {
                if let Some(v) = self.get_mut(nx, ny).filter(|v| **v < Self::FLASH) {
                    *v += 1;
                    if *v == Self::FLASH {
                        stack.push((nx, ny));
                    }
                }
            }
        }

        let mut counter = 0;
        for i in self
            .0
            .iter_mut()
            .map(|row| row.iter_mut())
            .flatten()
            .filter(|i| **i == Self::FLASH)
        {
            counter += 1;
            *i = 0;
        }

        (self, counter)
    }
}

fn main() {
    let mut octos = Octopussi::read(&std::fs::read_to_string("input/11.txt").unwrap());

    for i in 1.. {
        let (no, flashes) = octos.flash();

        if flashes == 100 {
            println!("{}", i);
            break;
        }

        octos = no;
    }
}
