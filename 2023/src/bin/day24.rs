use aoc_prelude::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, Sub};

#[derive(Debug, Copy, Clone)]
struct Coords {
    x: f64,
    y: f64,
    z: f64,
}

impl Coords {
    fn as_ints(&self) -> (i128, i128, i128) {
        (self.x as i128, self.y as i128, self.z as i128)
    }
}

impl Sub<Coords> for Coords {
    type Output = Coords;

    fn sub(self, rhs: Coords) -> Self::Output {
        Coords {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Hail {
    orig: Coords,
    v: Coords,
}

impl Display for Hail {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x, y, z) = self.orig.as_ints();
        let (vx, vy, vz) = self.v.as_ints();
        f.write_str(&format!("{x}, {y}, {z} @ {vx}, {vy}, {vz}"))
    }
}

impl<T: Iterator<Item = f64>> From<T> for Hail {
    fn from(nums: T) -> Self {
        let (x, y, z, vx, vy, vz) = nums.collect_tuple::<(_, _, _, _, _, _)>().unwrap();
        Self {
            orig: Coords { x, y, z },
            v: Coords {
                x: vx,
                y: vy,
                z: vz,
            },
        }
    }
}

impl Hail {
    fn slope(&self) -> f64 {
        self.v.y / self.v.x
    }

    fn y_intercept(&self) -> f64 {
        self.orig.y - self.slope() * self.orig.x
    }
}

impl BitAnd<Hail> for Hail {
    type Output = Option<(f64, f64)>;

    fn bitand(self, rhs: Hail) -> Self::Output {
        if self.v.x == rhs.v.x && self.v.y == rhs.v.y {
            // assumption is they didn't give us the same trajectory twice
            return None;
        }
        let x = (rhs.y_intercept() - self.y_intercept()) / (self.slope() - rhs.slope());
        let y = self.slope() * x + self.y_intercept();
        if (x - self.orig.x).signum() == self.v.x.signum()
            && (y - self.orig.y).signum() == self.v.y.signum()
            && (x - rhs.orig.x).signum() == rhs.v.x.signum()
            && (y - rhs.orig.y).signum() == rhs.v.y.signum()
        {
            Some((x, self.slope() * x + self.y_intercept()))
        } else {
            None
        }
    }
}

impl Sub<Coords> for Hail {
    type Output = Hail;

    fn sub(self, rhs: Coords) -> Self::Output {
        Self {
            orig: self.orig - rhs,
            v: self.v,
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let hails = input
        .lines()
        .map(|l| {
            l.replace('@', ",")
                .split(',')
                .filter_map(|x| x.trim().parse::<i128>().ok().map(|x| x as f64))
                .into()
        })
        .collect::<Vec<Hail>>();

    let (t_lo, t_hi) = (200000000000000f64, 400000000000000f64);

    let p1 = hails
        .iter()
        .tuple_combinations()
        .filter_map(|(h1, h2)| h1.bitand(*h2))
        .filter(|&(x, y)| t_lo <= x && x <= t_hi && t_lo <= y && y <= t_hi)
        .count();

    (p1, 0)
}
aoc_2023::main! {
    solve(include_str!("../../inputs/24.in"))
}
