use aoc_cycles::multicycle;
use aoc_prelude::Itertools;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

use std::ptr;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct TiltMap<const M: usize> {
    inner: [[char; M]; M],
}

impl<const M: usize> Display for TiltMap<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .inner
                .iter()
                .map(|x| x.iter().collect::<String>())
                .join("\n"),
        )
    }
}

impl<const M: usize> FromStr for TiltMap<M> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = [[' '; M]; M];
        for (i, c) in s[0..M * M].chars().enumerate() {
            inner[i / M][i % M] = c;
        }
        Ok(TiltMap { inner })
    }
}

impl<const M: usize> TiltMap<M> {
    fn transpose(&mut self) {
        for r in 0..M {
            for c in r..M {
                // trust me
                if c != r {
                    unsafe {
                        ptr::swap(&mut self.inner[r][c], &mut self.inner[c][r]);
                    }
                }
            }
        }
    }

    fn flip_vertical(&mut self) {
        for row in &mut self.inner {
            row.reverse();
        }
    }

    fn tilt_left(&mut self) {
        let mut bins = [0; M];

        for row in self.inner.as_mut() {
            let mut i = 0;

            for (idx, &c) in row.iter().enumerate().skip(1) {
                if c == 'O' {
                    bins[i] = idx;
                    i += 1;
                }
            }
            for &pos in &bins[0..i] {
                let mut new_idx = pos;
                for inner in (0..pos).rev() {
                    if row[inner] != '.' {
                        break;
                    } else {
                        new_idx = inner;
                    }
                }
                // trust me
                if new_idx != pos {
                    unsafe {
                        ptr::swap(&mut row[pos], &mut row[new_idx]);
                    }
                }
            }
        }
    }
}

fn const_cycle<const M: usize>(m: &mut TiltMap<M>) {
    for _ in 0..4 {
        m.transpose();
        m.tilt_left();
        m.flip_vertical();
    }
}

fn const_load<const M: usize>(m: &TiltMap<M>) -> i32 {
    let mut ans = 0;
    for (r, row) in m.inner.into_iter().enumerate() {
        for el in row.into_iter() {
            if el == 'O' {
                ans += (M - r) as i32;
            }
        }
    }
    ans
}

fn solve() -> (i32, i32) {
    let c_map = include_str!("../../inputs/day14.txt")
        .replace('\n', "")
        .trim()
        .parse::<TiltMap<100>>()
        .expect("nope");

    let mut p1_map = c_map;
    p1_map.transpose();
    p1_map.tilt_left();
    p1_map.transpose();
    let p1 = const_load(&p1_map);

    let p2 = const_load(&multicycle(c_map, const_cycle, 1000000000));
    (p1, p2)
}

aoc_2023::main! {
    solve()
}
