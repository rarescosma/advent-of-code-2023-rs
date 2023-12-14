use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::HashSet;

#[derive(Copy, Clone)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn offset(&self) -> Pos {
        match self {
            Dir::North => (0, -1),
            Dir::South => (0, 1),
            Dir::East => (1, 0),
            Dir::West => (-1, 0),
        }
        .into()
    }

    fn tilt_range<T>(&self, m: &Map<T>) -> (i32, i32, i32, i32, i32) {
        match self {
            Dir::North => (0, 1, m.size.y, 0, m.size.x),
            Dir::South => (m.size.y - 1, -1, -1, 0, m.size.x),
            Dir::East => (m.size.x - 1, -1, -1, 0, m.size.y),
            Dir::West => (0, 1, m.size.x, 0, m.size.y),
        }
    }

    fn make_pos(&self, c1: i32, c2: i32) -> Pos {
        match self {
            Dir::North | Dir::South => (c2, c1),
            Dir::East | Dir::West => (c1, c2),
        }
        .into()
    }
}

fn cast_ray(p: Pos, m: &Map<char>, dir: Dir) -> Pos {
    let mut ret = p;
    let offset = dir.offset();
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

fn tilt(m: &mut Map<char>, dir: Dir) {
    let (start, increment, end, r_start, r_end) = dir.tilt_range(m);
    let mut c1 = start;

    while c1 != end {
        for c2 in r_start..r_end {
            let p = dir.make_pos(c1, c2);
            if m.get_unchecked(p) == 'O' {
                let new_pos = cast_ray(p, m, dir);
                if new_pos != p {
                    m.swap(new_pos, p);
                }
            }
        }

        c1 += increment;
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
    tilt(m, Dir::North);
    tilt(m, Dir::West);
    tilt(m, Dir::South);
    tilt(m, Dir::East);
}

fn cycles_until_repeat(m: &mut Map<char>) -> i32 {
    let mut cache: HashSet<Map<char>> = HashSet::with_capacity(512);
    (1..)
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

    tilt(&mut p1_map, Dir::North);
    let p1 = load(&p1_map);

    let phase = cycles_until_repeat(&mut p2_map);
    let wavelength = cycles_until_repeat(&mut p2_map) - 1;

    let rem = (1000000000 - phase) % wavelength;

    for _ in 1..rem {
        cycle(&mut p2_map);
    }

    (p1, load(&p2_map))
}

aoc_2023::main! {
    solve()
}
