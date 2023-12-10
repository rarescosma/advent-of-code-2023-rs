use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::*;

static DEBUG: bool = false;

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
            ('7', [SOUTH, WEST]),
            ('L', [NORTH, EAST]),
            ('J', [NORTH, WEST]),
            ('|', [NORTH, SOUTH]),
            ('-', [WEST, EAST]),
        ] {
            res.insert(c, dir.map(|c| Pos::from(OFFSET[c])).into());
        }
        res
    };
}

fn direction(p: Pos) -> Dir {
    match (p.x, p.y) {
        (0, -1) => NORTH,
        (1, 0) => EAST,
        (0, 1) => SOUTH,
        (-1, 0) => WEST,
        _ => unimplemented!(),
    }
}

trait Rotate {
    fn rotate(x: Dir) -> Dir;
}

struct Clockwise;
struct Anticlockwise;

impl Rotate for Clockwise {
    fn rotate(x: Dir) -> Dir {
        (x + 1) % 4
    }
}

impl Rotate for Anticlockwise {
    fn rotate(x: Dir) -> Dir {
        (x + 3) % 4
    }
}

fn is_edge<T>(p: Pos, m: &Map<T>) -> bool {
    p.x == 0 || p.y == 0 || p.x == m.size.x - 1 || p.y == m.size.y - 1
}

fn area_points<T: Rotate>(&(cur, next): &(Pos, Pos)) -> [Pos; 2] {
    let dir = OFFSET[<T as Rotate>::rotate(direction(next - cur))].into();
    [cur + dir, next + dir]
}

struct World {
    map: Map<char>,
    loop_nodes: HashSet<Pos>,
    loop_edges: Vec<(Pos, Pos)>,
}

impl World {
    fn closed_area<T: Rotate>(&self) -> Option<HashSet<Pos>> {
        let mut q = self
            .loop_edges
            .iter()
            .flat_map(area_points::<T>)
            .filter(|x| !self.loop_nodes.contains(x))
            .collect::<VecDeque<_>>();

        let mut seen = HashSet::with_capacity(500);

        while !q.is_empty() {
            let pt = q.pop_front().unwrap();
            if seen.contains(&pt) || self.loop_nodes.contains(&pt) {
                continue;
            }
            seen.insert(pt);
            for neigh in pt.neighbors_simple() {
                if is_edge(neigh, &self.map) {
                    return None;
                }
                q.push_back(neigh);
            }
        }
        Some(seen)
    }
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/day10.txt")
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
                .map(|av| av[0] + pos == start || av[1] + pos == start)
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    assert_eq!(can_go.len(), 2, "start pos: {:?} not on the loop", start);

    let mut cur = can_go[0];
    let mut loop_nodes = HashSet::from([start, cur]);
    let mut loop_edges = Vec::new();

    while cur != can_go[1] {
        let next = NEIGHS[map.get_unchecked_ref(cur)]
            .iter()
            .map(|n| cur + *n)
            .find(|p| !loop_nodes.contains(p))
            .expect("we're on the loop but can't go anywhere...");
        loop_edges.push((cur, next));
        loop_nodes.insert(next);
        cur = next;
    }
    let p1 = (loop_nodes.len() + 1) / 2;

    let world = World {
        map,
        loop_nodes,
        loop_edges,
    };

    let clockwise_closed = world.closed_area::<Clockwise>();
    let anti_closed = world.closed_area::<Anticlockwise>();

    let closed_area = if clockwise_closed.is_some() {
        clockwise_closed
    } else if anti_closed.is_some() {
        anti_closed
    } else {
        None
    };

    if DEBUG {
        let mut vis_map = Map::fill(world.map.size, ' ');

        for pos in world.loop_nodes {
            vis_map.set(
                pos,
                match world.map.get_unchecked(pos) {
                    'L' => '╚',
                    'J' => '╝',
                    '7' => '╗',
                    'F' => '╔',
                    '|' => '║',
                    '-' => '═',
                    x => x,
                },
            );
        }

        if let Some(closed_area) = &closed_area {
            for pos in closed_area {
                vis_map.set(pos, 'X');
            }
        }

        println!("{}", vis_map);
    }

    let p2 = closed_area.map(|a| a.len()).unwrap_or(0);

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
