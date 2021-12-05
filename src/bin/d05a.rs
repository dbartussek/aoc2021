use std::collections::HashSet;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Slope {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let x = parts
            .next()
            .ok_or("x is missing")?
            .parse()
            .map_err(|_| "Cannot parse x")?;
        let y = parts
            .next()
            .ok_or("y is missing")?
            .parse()
            .map_err(|_| "Cannot parse y")?;

        if parts.next().is_some() {
            return Err("Too many numbers");
        }

        Ok(Self { x, y })
    }
}

impl Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Slope {
    pub fn simplify(self) -> Self {
        match (self.x, self.y) {
            (0, 0) => self,

            (0, n) if n > 0 => Self { x: 0, y: 1 },
            (n, 0) if n > 0 => Self { x: 1, y: 0 },

            (0, _) => Self { x: 0, y: -1 },
            (_, 0) => Self { x: -1, y: 0 },

            _ => self,
        }
    }

    pub fn point_at(self, i: i32) -> Point {
        let step_size = self.x.abs().max(self.y.abs());

        let x = i * self.x / step_size;
        let y = i * self.y / step_size;

        Point { x, y }
    }
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("->").map(|s| s.trim());

        let a = parts.next().ok_or("Missing starting point")?.parse()?;
        let b = parts.next().ok_or("Missing end point")?.parse()?;

        Ok(Self { a, b })
    }
}

impl Line {
    pub fn is_aligned(&self) -> bool {
        self.a.x == self.b.x || self.a.y == self.b.y
    }

    pub fn slope(&self) -> Slope {
        let x = self.b.x - self.a.x;
        let y = self.b.y - self.a.y;

        Slope { x, y }
    }

    pub fn points(self) -> impl Iterator<Item = Point> {
        let mut last_was_b = false;

        (0..)
            .map(move |i| {
                let slope = self.slope().simplify();
                let point = self.a + slope.point_at(i);
                (point, point == self.b)
            })
            .take_while(move |(_, is_b)| {
                let end = last_was_b;
                last_was_b = *is_b;
                !end
            })
            .map(|(point, _)| point)
    }
}

fn main() {
    let input = std::fs::read_to_string("input/05.txt").unwrap();

    let lines: Vec<Line> = input.lines().map(|line| line.parse().unwrap()).collect();
    let lines: Vec<Line> = lines.iter().copied().filter(Line::is_aligned).collect();
    let line_points: Vec<HashSet<Point>> =
        lines.iter().map(|line| line.points().collect()).collect();

    let mut seen = HashSet::new();
    let mut overlaps = HashSet::new();

    for line in line_points {
        for point in line {
            if !seen.insert(point) {
                overlaps.insert(point);
            }
        }
    }

    println!("{}", overlaps.len());
}
