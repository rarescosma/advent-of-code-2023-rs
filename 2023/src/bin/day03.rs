use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{HashSet, Itertools};
use std::collections::VecDeque;

enum Tile {
    Empty,
    Symbol(u8),
    Number(u8),
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            x => {
                if x.is_ascii_digit() {
                    Self::Number(x - 0x30)
                } else {
                    Self::Symbol(x)
                }
            }
        }
    }
}

fn extract_numbers<T>(map: &Map<Tile>, start: Pos) -> T
where
    T: FromIterator<(Pos, u64)>,
{
    start
        .neighbors_diag()
        .filter(|p| matches!(map.get_ref(*p), Some(Tile::Number(_))))
        .map(|p| expand_number(map, p))
        .collect::<T>()
}

fn expand_number(map: &Map<Tile>, start: Pos) -> (Pos, u64) {
    let mut deq = VecDeque::new();
    if let Some(Tile::Number(x)) = map.get_ref(start) {
        deq.push_front(*x);
    }

    let offset_left = Pos::from((-1, 0));
    let offset_right = Pos::from((1, 0));

    let mut left = start + offset_left;
    let mut right = start + offset_right;

    while let Some(Tile::Number(x)) = map.get_ref(left) {
        deq.push_front(*x);
        left += offset_left;
    }

    while let Some(Tile::Number(x)) = map.get_ref(right) {
        deq.push_back(*x);
        right += offset_right;
    }

    let num = deq.iter().fold(0_u64, |acc, x| acc * 10 + (*x as u64));

    // the while loop left us one column left of the actual number
    (left + offset_right, num)
}

aoc_2023::main! {
    let input = include_str!("../../inputs/day03.txt")
        .lines()
        .collect::<Vec<_>>();

    let map_size = (input[0].len(), input.len());

    let map = Map::<Tile>::new(
        map_size,
        input.into_iter().flat_map(|l| l.bytes().map(Tile::from)),
    );

    let p1 = map
        .iter()
        .filter(|p| matches!(map.get_unchecked_ref(*p), Tile::Symbol(_)))
        .flat_map(|p| extract_numbers::<Vec<_>>(&map, p))
        .unique()
        .map(|(_, num)| num)
        .sum::<u64>();

    let p2 = map
        .iter()
        .filter(|p| matches!(map.get_unchecked_ref(*p), Tile::Symbol(b'*')))
        .filter_map(|p| {
            let num_set = extract_numbers::<HashSet<_>>(&map, p);
            if num_set.len() == 2 {
                Some(num_set.into_iter().map(|(_, num)| num).product::<u64>())
            } else {
                None
            }
        })
        .sum::<u64>();

    (p1, p2)
}
