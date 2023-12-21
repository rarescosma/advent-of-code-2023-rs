use aoc_2023::ConstMap;
use aoc_2dmap::prelude::Pos;
use aoc_prelude::{lazy_static, HashSet};
use rayon::prelude::*;
use std::collections::VecDeque;

lazy_static! {
    static ref OFF: [Pos; 4] = [(0, -1).into(), (-1, 0).into(), (0, 1).into(), (1, 0).into()];
}
const M_SIZE: usize = 131;

fn bfs<const M: usize>(start: Pos, num_steps: usize, map: &ConstMap<M, char>) -> usize {
    let mut ans = 0;
    let mut seen = HashSet::with_capacity(2 * num_steps.pow(2));
    let mut q = VecDeque::with_capacity(5 * num_steps);

    let start_tile = Pos::new(0, 0);

    seen.insert((start, start_tile));
    q.push_back((start, start_tile, 0));

    let side = M as i32;

    while let Some((pos, tile_coords, steps)) = q.pop_front() {
        if steps % 2 == num_steps % 2 {
            ans += 1;
        }
        if steps >= num_steps {
            continue;
        }

        for off in OFF.iter() {
            let mut new_pos = pos + *off;
            let mut new_tile_coords = tile_coords;
            if new_pos.x < 0 {
                new_pos.x = side - 1;
                new_tile_coords.x -= 1;
            }
            if new_pos.x > side - 1 {
                new_pos.x = 0;
                new_tile_coords.x += 1;
            }
            if new_pos.y < 0 {
                new_pos.y = side - 1;
                new_tile_coords.y -= 1;
            }
            if new_pos.y > side - 1 {
                new_pos.y = 0;
                new_tile_coords.y += 1;
            }
            if map.get(new_pos) == Some('#') || seen.contains(&(new_pos, new_tile_coords)) {
                continue;
            }
            seen.insert((new_pos, new_tile_coords));
            q.push_back((new_pos, new_tile_coords, steps + 1))
        }
    }
    ans
}

fn solve(input: &str) -> (usize, f64) {
    let map = input
        .replace('\n', "")
        .parse::<ConstMap<M_SIZE, char>>()
        .expect("nope");

    let start = (0..map.size())
        .zip(0..map.size())
        .find(|&(x, y)| map.get(Pos::new(x, y)) == Some('S'))
        .expect("no start")
        .into();

    let (n_steps, half) = (26501365, M_SIZE / 2);

    let res = [64, half, half + M_SIZE, half + 2 * M_SIZE]
        .par_iter()
        .map(|steps| bfs(start, *steps, &map))
        .collect::<Vec<_>>();
    let p1 = res[0];

    //--------------------------------------------------------------------------
    assert_eq!(n_steps % M_SIZE, half);
    let (f0x0, f0x1, f0x2) = (res[1], res[2], res[3]);

    let f_side = M_SIZE as f64;
    let f1x0x1 = (f0x1 - f0x0) as f64 / f_side;
    let f1x1x2 = (f0x2 - f0x1) as f64 / f_side;

    let f2x0x1x1 = (f1x1x2 - f1x0x1) / (f_side * 2f64);

    let poly = |x: f64| {
        f0x0 as f64
            + f1x0x1 * (x - half as f64)
            + f2x0x1x1 * (x - half as f64) * (x - (half + M_SIZE) as f64)
    };

    let p2 = poly(n_steps as f64);

    (p1, p2)
}
aoc_2023::main! {
    solve(include_str!("../../inputs/21.in"))
}
