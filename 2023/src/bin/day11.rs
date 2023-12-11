use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::Itertools;
use std::cmp::{max, min};

fn find_empty_lines<S: AsRef<str>>(lines: impl Iterator<Item = S>) -> Vec<i32> {
    lines
        .filter(|x| !x.as_ref().is_empty())
        .enumerate()
        .filter_map(|(idx, content)| {
            if content.as_ref().chars().all(|c| c == '.') {
                Some(idx as i32)
            } else {
                None
            }
        })
        .sorted()
        .collect::<Vec<_>>()
}

fn manhattan(p: Pos, q: Pos) -> usize {
    let m = p - q;
    (m.x.abs() + m.y.abs()) as usize
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/day11.txt")
        .lines()
        .collect::<Vec<_>>();

    // get the coords of empty lines => store them as cols because we transpose
    let empty_cols = find_empty_lines(input.iter());

    let w = input[0].len();
    let h = input.len();

    let mut transposed_chars = vec![' '; w * h];
    for y in 0..h {
        for (x, c) in input[y].chars().enumerate() {
            transposed_chars[x * h + y] = c;
        }
    }
    let map = Map::new((h, w), transposed_chars.into_iter());

    let empty_rows = find_empty_lines(format!("{map}").lines());

    let dist = map
        .iter()
        .filter(|p| map.get_unchecked(p) == '#')
        .collect::<Vec<_>>()
        .into_iter()
        .tuple_combinations::<(_, _)>()
        .map(|(g1, g2)| {
            let min_x = min(g1.x, g2.x);
            let min_y = min(g1.y, g2.y);
            let max_x = max(g1.x, g2.x);
            let max_y = max(g1.y, g2.y);

            let empty_rows_between = empty_rows
                .iter()
                .filter(|y| **y > min_y && **y < max_y)
                .count();
            let empty_cols_between = empty_cols
                .iter()
                .filter(|x| **x > min_x && **x < max_x)
                .count();
            (manhattan(g1, g2), empty_rows_between + empty_cols_between)
        })
        .collect::<Vec<_>>();

    let p1 = dist.iter().map(|&(d, e)| d + e).sum();
    let p2 = dist.iter().map(|&(d, e)| d + e * (1000000 - 1)).sum();

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
