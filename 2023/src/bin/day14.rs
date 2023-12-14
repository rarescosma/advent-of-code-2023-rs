use ahash::RandomState;

use aoc_prelude::{lazy_static, HashSet, Itertools};
use std::fmt::{Display, Formatter};
use std::hash::{BuildHasher, Hash, Hasher};

use std::ptr;
use std::str::FromStr;

lazy_static! {
    static ref HASHER_BUILDER: RandomState = RandomState::new();
}

fn manually_hash<H: Hash>(state: &H) -> u64 {
    let mut hasher = HASHER_BUILDER.build_hasher();
    state.hash(&mut hasher);
    hasher.finish()
}

fn multicycle<T: Clone + Eq + PartialEq + Hash, F: Fn(&mut T)>(
    m: T,
    cycle_f: F,
    num_cycles: usize,
) -> T {
    assert!(num_cycles > 0);

    let mut cache: HashSet<u64> = HashSet::with_capacity(512);
    let mut queue: Vec<(u64, T)> = Vec::with_capacity(512);
    let mut look_for = None;

    let m = m.clone();
    let mc = &mut m.clone();

    let cycle_after = (0..num_cycles).find(|_| {
        let h = manually_hash(mc);
        if cache.contains(&h) {
            look_for = Some(h);
            true
        } else {
            cache.insert(h);
            queue.push((h, mc.clone()));
            cycle_f(mc);
            false
        }
    });
    if cycle_after.is_none() {
        return m;
    }

    let cycle_after = cycle_after.unwrap();

    let prefix_length = queue
        .iter()
        .position(|&(h, _)| Some(h) == look_for)
        .unwrap();

    let wavelength = cycle_after - prefix_length;

    let idx = (num_cycles - prefix_length) % wavelength + prefix_length;
    queue.remove(idx).1
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct FooMap<const M: usize> {
    inner: [[char; M]; M],
}

impl<const M: usize> Display for FooMap<M> {
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

impl<const M: usize> FromStr for FooMap<M> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = [[' '; M]; M];
        for (i, c) in s[0..M * M].chars().enumerate() {
            inner[i / M][i % M] = c;
        }
        Ok(FooMap { inner })
    }
}

impl<const M: usize> FooMap<M> {
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

fn const_cycle<const M: usize>(m: &mut FooMap<M>) {
    for _ in 0..4 {
        m.transpose();
        m.tilt_left();
        m.flip_vertical();
    }
}

fn const_load<const M: usize>(m: &FooMap<M>) -> i32 {
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
        .parse::<FooMap<100>>()
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
