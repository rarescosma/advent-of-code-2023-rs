use aoc_2dmap::prelude::{Map, Pos};
use rayon::prelude::*;
use std::collections::VecDeque;

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

const OFFSET: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Copy, Clone)]
struct Beam {
    pos: Pos,
    facing: usize,
}

impl Beam {
    fn new<S: Into<Pos>>(pos: S, facing: usize) -> Self {
        Self {
            pos: pos.into(),
            facing,
        }
    }

    fn encounter(&self, n_pos: Pos, map: &Map<char>) -> [Option<Beam>; 2] {
        let prop = || Some(Beam::new(n_pos, self.facing));
        let rot = |facing| Some(Beam::new(n_pos, facing));

        let tile = map.get(n_pos);

        if tile.is_none() {
            return [None, None];
        }
        let tile = tile.unwrap();

        match tile {
            // we hit a | splitter from the side => make two beams facing N/S
            '|' if self.facing == EAST || self.facing == WEST => [rot(NORTH), rot(SOUTH)],
            // we hit a - splitter from the side => make two beams facing E/W
            '-' if self.facing == NORTH || self.facing == SOUTH => [rot(EAST), rot(WEST)],
            // empty space -> continue as is
            '.' | '|' | '-' => [prop(), None],
            '/' => [
                match self.facing {
                    NORTH => rot(EAST),
                    EAST => rot(NORTH),
                    SOUTH => rot(WEST),
                    WEST => rot(SOUTH),
                    _ => unimplemented!(),
                },
                None,
            ],
            '\\' => [
                match self.facing {
                    NORTH => rot(WEST),
                    WEST => rot(NORTH),
                    SOUTH => rot(EAST),
                    EAST => rot(SOUTH),
                    _ => unimplemented!(),
                },
                None,
            ],
            _ => unimplemented!(),
        }
    }
}

fn simulate_beam(start: Beam, map: &Map<char>) -> usize {
    let mut q = VecDeque::with_capacity(10);
    q.extend(start.encounter(start.pos, map).into_iter().flatten());

    let mut seen = vec![vec![[false; 4]; map.size.x as usize]; map.size.y as usize];

    while let Some(beam) = q.pop_front() {
        let (x, y) = (beam.pos.x as usize, beam.pos.y as usize);
        if seen[x][y][beam.facing] {
            continue;
        }
        seen[x][y][beam.facing] = true;

        let n_pos = beam.pos + OFFSET[beam.facing].into();
        let new_beams = beam.encounter(n_pos, map);

        q.extend(new_beams.into_iter().flatten());
    }

    seen.iter()
        .flatten()
        .filter(|x| x.iter().any(|&y| y))
        .count()
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/16.in")
        .lines()
        .collect::<Vec<_>>();

    let size = (input[0].len(), input.len());

    let map = Map::new(size, input.join("").chars());

    let p1 = simulate_beam(Beam::new((0, 0), EAST), &map);

    //-------------------------------------------------------------------------
    let mut start_beams = Vec::new();

    for x in 0..map.size.x {
        start_beams.push(Beam::new((x, 0), SOUTH));
        start_beams.push(Beam::new((x, map.size.y - 1), NORTH));
    }
    for y in 0..map.size.y {
        start_beams.push(Beam::new((0, y), EAST));
        start_beams.push(Beam::new((map.size.x - 1, y), WEST));
    }

    let p2 = start_beams
        .par_iter()
        .map(|&b| simulate_beam(b, &map))
        .max()
        .unwrap();

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
