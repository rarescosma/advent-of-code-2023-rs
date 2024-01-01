use aoc_prelude::{BTreeMap, HashSet};

fn solve(input: &str) -> (usize, usize) {
    let mut adj = BTreeMap::new();
    input
        .lines()
        .filter_map(|l| l.split_once(": "))
        .for_each(|(from, rest)| {
            for r in rest.split_whitespace() {
                adj.entry(from).or_insert_with(HashSet::new).insert(r);
                adj.entry(r).or_insert_with(HashSet::new).insert(from);
            }
        });

    let idx_map = adj
        .keys()
        .enumerate()
        .map(|(x, y)| (*y, x))
        .collect::<BTreeMap<_, _>>();

    dbg!(&adj);
    dbg!(&idx_map);

    (0, 0)
}

fn main() {
    solve(include_str!("../../inputs/25.ex"));
}
