use anyhow::*;
use std::str::FromStr;

pub struct School(pub [u64; 9]);

impl FromStr for School {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self(Default::default());

        for n in s.split(',') {
            let n: usize = n.trim().parse().with_context(|| format!("{}", n))?;
            if n >= this.0.len() {
                return Err(anyhow!("Index {} out of bounds", n));
            }
            this.0[n] += 1;
        }

        Ok(this)
    }
}

impl School {
    pub fn day(self) -> Self {
        let mut next = Self(Default::default());
        next.0[..(self.0.len() - 1)].copy_from_slice(&self.0[1..]);
        next.0[6] += self.0[0];
        next.0[8] += self.0[0];
        next
    }
}

fn main() {
    let input = std::fs::read_to_string("input/06.txt").unwrap();
    let mut school: School = input.parse().unwrap();

    for _ in 0..80 {
        school = school.day();
    }

    println!("{}", school.0.iter().sum::<u64>());
}
