use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{lazy_static, ArrayVec, HashMap, HashSet};

type Dir = usize;

static NORTH: Dir = 0;
static EAST: Dir = 1;
static SOUTH: Dir = 2;
static WEST: Dir = 3;
static OFFSET: [(i8, i8); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

lazy_static! {
    static ref NEIGHS: HashMap<char, ArrayVec<Pos, 2>> = {
        let mut res = HashMap::new();
        for (c, dir) in [
            ('F', [SOUTH, EAST]),
            ('L', [NORTH, EAST]),
            ('7', [SOUTH, WEST]),
            ('J', [NORTH, WEST]),
            ('|', [NORTH, SOUTH]),
            ('-', [WEST, EAST]),
        ] {
            res.insert(c, dir.map(|c| Pos::from(OFFSET[c])).into());
        }
        res
    };
}

// Shoelace theorem, there is no escaping
fn shoelace(vertices: &[Pos]) -> i32 {
    assert!(vertices.len() >= 3);

    let a0 = vertices[0].y * (vertices[vertices.len() - 1].x - vertices[1].x);

    (a0 + (1..vertices.len() - 1)
        .map(|i| vertices[i].y * (vertices[i - 1].x - vertices[i + 1].x))
        .sum::<i32>())
        / 2
}

fn solve() -> (usize, i32) {
    let input = include_str!("../../inputs/10.in")
        .lines()
        .collect::<Vec<_>>();

    let map_size = (input[0].len(), input.len());

    let map = Map::<char>::new(map_size, input.into_iter().flat_map(|l| l.chars()));

    let start = map
        .iter()
        .find(|x| map.get_unchecked(*x) == 'S')
        .expect("no start");

    let can_go = start
        .neighbors_simple()
        .filter(|&pos| {
            map.get_ref(pos)
                .and_then(|c| NEIGHS.get(c))
                .is_some_and(|av| av[0] + pos == start || av[1] + pos == start)
        })
        .collect::<Vec<_>>();

    assert_eq!(can_go.len(), 2, "start pos: {start:?} not on the loop");

    let mut cur = can_go[0];
    let mut loop_nodes = HashSet::<Pos>::from([start, cur]);
    let mut loop_nodes_v = vec![start, cur];

    while cur != can_go[1] {
        let next = NEIGHS[map.get_unchecked_ref(cur)]
            .iter()
            .map(|n| cur + *n)
            .find(|p| !loop_nodes.contains(p))
            .expect("we're on the loop but can't go anywhere...");
        loop_nodes.insert(next);
        loop_nodes_v.push(next);
        cur = next;
    }
    loop_nodes_v.push(start);

    let p1 = (loop_nodes.len() + 1) / 2;

    let p2 = shoelace(&loop_nodes_v).abs() - loop_nodes_v.len() as i32 / 2 + 1;

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
