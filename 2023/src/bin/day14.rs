use ahash::RandomState;
use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{lazy_static, HashSet};
use std::hash::BuildHasher;
use std::hash::{Hash, Hasher};
use std::ops::RangeInclusive;

const MAP_SIZE: i32 = 100;
const NORTH: usize = 0;
const WEST: usize = 1;
const SOUTH: usize = 2;
const EAST: usize = 3;

lazy_static! {
    static ref OFFSET: [Pos; 4] = [
        Pos::from((0, -1)),
        Pos::from((-1, 0)),
        Pos::from((0, 1)),
        Pos::from((1, 0)),
    ];
    // (from row/col to row/col, reverse)
    static ref TILT: [(RangeInclusive<i32>, bool); 4] = [
        (1..=MAP_SIZE - 1, false),
        (1..=MAP_SIZE - 1, false),
        (0..=MAP_SIZE - 2, true),
        (0..=MAP_SIZE - 2, true),
    ];
    static ref HASHER_BUILDER: RandomState = RandomState::new();
}

fn make_pos(c1: i32, c2: i32, dir: usize) -> Pos {
    match dir {
        NORTH | SOUTH => (c2, c1).into(),
        EAST | WEST => (c1, c2).into(),
        _ => unreachable!(),
    }
}

fn cast_ray(p: Pos, m: &Map<char>, dir: usize) -> Pos {
    let mut ret = p;
    let offset = OFFSET[dir];
    loop {
        let probe = ret + offset;
        if m.get(probe) == Some('.') {
            ret = probe;
        } else {
            break;
        }
    }
    ret
}

fn tilt(m: &mut Map<char>, dir: usize) {
    let (mut rng, rev) = TILT[dir].clone();
    let it = if rev {
        RangeInclusive::next_back
    } else {
        RangeInclusive::next
    };
    while let Some(c1) = it(&mut rng) {
        for c2 in 0..MAP_SIZE {
            let p = make_pos(c1, c2, dir);
            if m.get_unchecked(p) == 'O' {
                let new_pos = cast_ray(p, m, dir);
                if new_pos != p {
                    m.swap(new_pos, p);
                }
            }
        }
    }
}

fn load(m: &Map<char>) -> i32 {
    m.iter()
        .filter_map(|p| {
            if m.get_unchecked(p) == 'O' {
                Some(m.size.y - p.y)
            } else {
                None
            }
        })
        .sum::<i32>()
}

fn cycle(m: &mut Map<char>) {
    (0..=3).for_each(|dir| tilt(m, dir));
}

fn manually_hash<H: Hash>(state: &H) -> u64 {
    let mut hasher = HASHER_BUILDER.build_hasher();
    state.hash(&mut hasher);
    hasher.finish()
}

fn multicycle<T: Clone + Eq + PartialEq + Hash, F: Fn(&mut T), U: Copy, S: Fn(&T) -> U>(
    m: T,
    cycle_f: F,
    score_f: S,
    num_cycles: usize,
) -> U {
    assert!(num_cycles > 0);

    let mut cache: HashSet<u64> = HashSet::with_capacity(512);
    let mut queue: Vec<(u64, U)> = Vec::with_capacity(512);
    let mut look_for = None;

    let m = &mut m.clone();

    let cycle_after = (0..num_cycles).find(|_| {
        let h = manually_hash(m);
        if cache.contains(&h) {
            look_for = Some(h);
            true
        } else {
            cache.insert(h);
            queue.push((h, score_f(m)));
            cycle_f(m);
            false
        }
    });
    if cycle_after.is_none() {
        return score_f(m);
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

fn solve() -> (i32, i32) {
    let input = include_str!("../../inputs/day14.txt")
        .lines()
        .collect::<Vec<_>>();

    let mut p1_map = Map::new(
        (input[0].len(), input.len()),
        input.into_iter().flat_map(|x| x.chars()),
    );
    let p2_map = p1_map.clone();

    tilt(&mut p1_map, NORTH);

    let p2 = multicycle(p2_map, cycle, load, 1000000000);

    (load(&p1_map), p2)
}

aoc_2023::main! {
    solve()
}
