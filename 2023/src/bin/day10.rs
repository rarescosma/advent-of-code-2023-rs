use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{lazy_static, ArrayVec, HashMap, HashSet, Itertools};
use std::mem;

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

struct Area {
    inner: HashSet<Pos>,
    touches_edge: bool,
}

impl Area {
    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_inner(&self) -> bool {
        !self.touches_edge
    }
}

struct World {
    map: Map<char>,
    visited: HashSet<Pos>,
    loopy: Vec<(Pos, Pos)>,
}

impl World {
    fn closed_area<T: Rotate>(&self) -> Area {
        let mut area = Area {
            inner: self
                .loopy
                .iter()
                .flat_map(area_points::<T>)
                .filter(|x| !self.visited.contains(x))
                .collect(),
            touches_edge: false,
        };
        self._expand(&mut area, &mut HashSet::new());
        area
    }

    fn _expand(&self, area: &mut Area, buf: &mut HashSet<Pos>) {
        buf.clear();
        for pos in area
            .inner
            .iter()
            .flat_map(|x| x.neighbors_simple_inclusive())
            .filter(|p| !self.visited.contains(p) && self.map.get(p).is_some())
        {
            buf.insert(pos);
            if is_edge(pos, &self.map) {
                area.touches_edge = true;
                return;
            }
        }
        if buf.len() > area.inner.len() {
            mem::swap(&mut area.inner, buf);
            // recurse
            self._expand(area, buf);
        }
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
        .find(|x| map.get_unchecked_ref(*x) == &'S')
        .expect("no start");

    let can_go = start
        .neighbors_simple()
        .filter(|x| match map.get_ref(x) {
            Some(c) if NEIGHS.contains_key(c) => NEIGHS[c].iter().map(|n| *x + *n).contains(&start),
            _ => false,
        })
        .collect::<Vec<_>>();

    let mut cur = can_go[0];
    let mut visited = HashSet::from([start, cur]);
    let mut loopy = Vec::new();

    while cur != can_go[1] {
        let n = NEIGHS[map.get_unchecked_ref(cur)]
            .iter()
            .map(|n| cur + *n)
            .find(|p| !visited.contains(p))
            .expect("we're on the loop but can't go anywhere...");
        loopy.push((cur, n));
        visited.insert(n);
        cur = n;
    }
    let p1 = (visited.len() + 1) / 2;

    let world = World {
        map,
        visited,
        loopy,
    };

    let clockwise_closed = world.closed_area::<Clockwise>();
    let anti_closed = world.closed_area::<Anticlockwise>();

    let closed_area = if clockwise_closed.is_inner() {
        Some(clockwise_closed)
    } else if anti_closed.is_inner() {
        Some(anti_closed)
    } else {
        None
    };

    if DEBUG {
        let mut vis_map = Map::fill(world.map.size, ' ');

        for pos in world.visited {
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
            for pos in &closed_area.inner {
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
