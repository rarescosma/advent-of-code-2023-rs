use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{lazy_static, HashSet};

const MAP_SIZE: i32 = 100;
const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;

struct TiltRanges {
    start: i32,
    step: i32,
    end: i32,
}

impl TiltRanges {
    fn new(start: i32, step: i32, end: i32) -> Self {
        TiltRanges { start, step, end }
    }
}

lazy_static! {
    static ref OFFSET: [Pos; 4] = [
        Pos::from((0, -1)),
        Pos::from((0, 1)),
        Pos::from((1, 0)),
        Pos::from((-1, 0)),
    ];
    static ref TILT: [TiltRanges; 4] = [
        TiltRanges::new(1, 1, MAP_SIZE),
        TiltRanges::new(MAP_SIZE - 2, -1, -1),
        TiltRanges::new(MAP_SIZE - 2, -1, -1),
        TiltRanges::new(1, 1, MAP_SIZE),
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
    let tilt = &TILT[dir];
    let mut c1 = tilt.start;

    while c1 != tilt.end {
        for c2 in 0..MAP_SIZE {
            let p = make_pos(dir, c1, c2);
            if m.get_unchecked(p) == 'O' {
                let new_pos = cast_ray(p, m, dir);
                if new_pos != p {
                    m.swap(new_pos, p);
                }
            }
        }
        c1 += tilt.step;
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
        .find(|_x| {
            cycle(m);
            if cache.contains(m) {
                true
            } else {
                cache.insert(m.clone());
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
    let p1 = load(&p1_map);

    let phase = cycles_until_repeat(&mut p2_map) + 1;
    let wavelength = cycles_until_repeat(&mut p2_map);

    let rem = (1000000000 - phase) % wavelength;

    for _ in 1..rem {
        cycle(&mut p2_map);
    }

    (p1, load(&p2_map))
}

aoc_2023::main! {
    solve()
}
