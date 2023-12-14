use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{lazy_static, HashSet};
use std::ops::RangeInclusive;

const MAP_SIZE: i32 = 100;
const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;

lazy_static! {
    static ref OFFSET: [Pos; 4] = [
        Pos::from((0, -1)),
        Pos::from((0, 1)),
        Pos::from((1, 0)),
        Pos::from((-1, 0)),
    ];
    static ref TILT: [(RangeInclusive<i32>, bool); 4] = [
        (1..=MAP_SIZE - 1, false),
        (0..=MAP_SIZE - 2, true),
        (0..=MAP_SIZE - 2, true),
        (1..=MAP_SIZE - 1, false),
    ];
}

fn make_pos(dir: usize, c1: i32, c2: i32) -> Pos {
    match dir {
        NORTH | SOUTH => (c2, c1).into(),
        _ => (c1, c2).into(),
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
            let p = make_pos(dir, c1, c2);
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
    tilt(m, NORTH);
    tilt(m, WEST);
    tilt(m, SOUTH);
    tilt(m, EAST);
}

fn cycles_until_repeat(m: &mut Map<char>) -> i32 {
    let mut cache: HashSet<Map<char>> = HashSet::with_capacity(512);
    (0..)
        .find(|_| {
            cache.contains(m) || {
                cache.insert(m.clone());
                cycle(m);
                false
            }
        })
        .expect("no cycle!")
}

fn solve() -> (i32, i32) {
    let input = include_str!("../../inputs/day14.txt")
        .lines()
        .collect::<Vec<_>>();

    let mut p1_map = Map::new(
        (input[0].len(), input.len()),
        input.into_iter().flat_map(|x| x.chars()),
    );
    let mut p2_map = p1_map.clone();

    tilt(&mut p1_map, NORTH);

    let phase = cycles_until_repeat(&mut p2_map);
    let wavelength = cycles_until_repeat(&mut p2_map);
    let rem = (1000000000 - phase) % wavelength;

    for _ in 0..rem {
        cycle(&mut p2_map);
    }

    (load(&p1_map), load(&p2_map))
}

aoc_2023::main! {
    solve()
}
