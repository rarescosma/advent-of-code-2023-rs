use aoc_2dmap::prelude::Map;
use aoc_prelude::{HashSet, Itertools};
use std::ops::Range;

#[derive(Debug, Copy, Clone)]
enum ReflectionMode {
    Row,
    Col,
}

struct Reflection<'a> {
    inner: &'a Map<char>,
    mode: ReflectionMode,
}

impl<'a> Reflection<'a> {
    fn dim(&self) -> i32 {
        match self.mode {
            ReflectionMode::Row => self.inner.size.y,
            ReflectionMode::Col => self.inner.size.x,
        }
    }

    fn factor(&self) -> usize {
        match self.mode {
            ReflectionMode::Row => 100,
            ReflectionMode::Col => 1,
        }
    }

    fn scan_range(&self) -> Range<i32> {
        0..self.dim() - 1
    }

    fn equal_cuts(&self, c1: i32, c2: i32) -> bool {
        match self.mode {
            ReflectionMode::Row => self
                .inner
                .get_row(c1)
                .zip(self.inner.get_row(c2))
                .all(|(x, y)| x == y),
            ReflectionMode::Col => self
                .inner
                .get_col(c1)
                .zip(self.inner.get_col(c2))
                .all(|(x, y)| x == y),
        }
    }
}

fn find_reflection(map: &Map<char>, mode: ReflectionMode) -> Vec<(usize, bool)> {
    let m = Reflection { inner: map, mode };

    // find matching consecutive rows and walk out from each cand and count matching rows
    let mut res = Vec::new();
    for cand in m.scan_range().filter(|&r| m.equal_cuts(r, r + 1)) {
        let next_up = (0..=cand)
            .rev()
            .zip(cand + 1..m.dim())
            .take_while(|&(ra, rb)| m.equal_cuts(ra, rb))
            .count();

        let to_upper_edge = cand as usize + 1;
        let to_lower_edge = m.dim() as usize - to_upper_edge;

        let touches_edge = (next_up == to_upper_edge) || (next_up == to_lower_edge);

        if touches_edge {
            res.push((to_upper_edge * m.factor(), true));
        }
    }

    if res.is_empty() {
        vec![(m.dim() as usize * m.factor(), false)]
    } else {
        res.sort_by(|x, y| y.cmp(x));
        res
    }
}

fn variations(m: &Map<char>) -> impl Iterator<Item = Map<char>> + '_ {
    m.iter().map(|p| {
        let mut new_m = m.clone();
        new_m.set(
            p,
            match m.get_unchecked(p) {
                '#' => '.',
                _ => '#',
            },
        );
        new_m
    })
}

fn solve() -> (usize, usize) {
    let mut maps = Vec::new();
    for (is_empty, group) in &include_str!("../../inputs/day13.txt")
        .lines()
        .group_by(|l| l.is_empty())
    {
        if !is_empty {
            let inner = group.collect::<Vec<_>>();
            maps.push(Map::new(
                (inner[0].len(), inner.len()),
                inner.join("").chars(),
            ));
        }
    }

    let mut p2_cache = HashSet::<(usize, bool)>::with_capacity(1024);

    let (p1, p2) = maps
        .iter()
        .map(|m| {
            p2_cache.clear();

            let rr = find_reflection(m, ReflectionMode::Row);
            let cr = find_reflection(m, ReflectionMode::Col);

            let p1_ref = rr
                .into_iter()
                .chain(cr)
                .find(|x| x.1)
                .expect("no reflection!");

            for mv in variations(m) {
                let new_r = find_reflection(&mv, ReflectionMode::Row);
                let new_c = find_reflection(&mv, ReflectionMode::Col);
                p2_cache.extend(
                    new_r
                        .into_iter()
                        .chain(new_c)
                        .filter(|&x| x.1 && x != p1_ref),
                );
            }

            (
                p1_ref.0,
                p2_cache.iter().next().expect("no new reflection!").0,
            )
        })
        .fold((0, 0), |x, y| (x.0 + y.0, x.1 + y.1));

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
