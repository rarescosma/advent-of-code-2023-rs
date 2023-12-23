use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{lazy_static, ArrayVec, HashMap, HashSet};
use std::cmp::max;
use std::collections::VecDeque;
use std::iter::once;
use std::sync::Mutex;

const MAX_NODES: usize = 512;
type Edges = ArrayVec<(Node, usize), 64>;
type Graph = ArrayVec<Edges, MAX_NODES>;

lazy_static! {
    static ref COUNTER: Mutex<(HashMap<Pos, usize>, usize)> = Mutex::new((HashMap::new(), 0));
}

fn unique_id<P: AsRef<Pos>>(pos: P) -> usize {
    let pos = pos.as_ref();
    let mut guard = COUNTER.lock().unwrap();
    if guard.0.contains_key(pos) {
        guard.0[pos]
    } else {
        guard.1 += 1;
        let id = guard.1;
        guard.0.insert(*pos, id);
        id
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone, Debug)]
struct Node {
    id: usize,
}

impl<P: AsRef<Pos>> From<P> for Node {
    fn from(pos: P) -> Self {
        Self { id: unique_id(pos) }
    }
}

struct World {
    graph: Graph,
    goal: Node,
}

#[derive(Default)]
struct BfsBuf {
    q: VecDeque<(Pos, usize)>,
    seen: HashSet<Pos>,
    res: ArrayVec<(Node, usize), 64>,
}

impl BfsBuf {
    fn clear(&mut self) {
        self.q.clear();
        self.seen.clear();
        self.res.clear();
    }
}

fn make_array<const M: usize, T: Default>() -> ArrayVec<T, M> {
    ArrayVec::<T, M>::from_iter((0..MAX_NODES).map(|_| T::default()))
}

fn make_world<V: Fn(Pos, Pos, Pos) -> bool, R: Fn(Pos) -> bool>(
    map: &Map<char>,
    (start_pos, goal_pos): (Pos, Pos),
    is_valid_pos: V,
    is_result: R,
) -> World {
    let goal = Node::from(goal_pos);

    let mut graph = make_array::<MAX_NODES, Edges>();
    let mut buf = BfsBuf::default();

    for tile in map.iter().filter(|&p| is_result(p)).chain(once(start_pos)) {
        buf.clear();
        buf.q.push_back(if is_diode(tile, map) {
            (tile + diode_offset(map.get_unchecked(tile)), 1)
        } else {
            (tile, 0)
        });
        bfs(
            &mut buf,
            |cur, next| is_valid_pos(cur, next, tile),
            &is_result,
        );
        graph[Node::from(tile).id] = buf.res.clone();
    }

    World { graph, goal }
}

fn solve(input: &str) -> (usize, usize) {
    let lines = input.lines().collect::<Vec<_>>();

    let size = (lines[0].len(), lines.len());
    let map = Map::new(size, lines.join("").chars());

    let (start_pos, goal_pos) = (Pos::new(1, 0), Pos::new(size.0 - 2, size.1 - 1));
    let start = Node::from(start_pos);

    //--------------------------------------------------------------------------
    let p1_world = make_world(
        &map,
        (start_pos, goal_pos),
        |cur, next, _| is_valid(next, &map) && !is_against(cur, next, &map),
        |next| is_diode(next, &map) || next == goal_pos,
    );
    let p1 = compute_paths(&p1_world, start, 0, &mut make_array::<MAX_NODES, bool>());

    //--------------------------------------------------------------------------
    let p2_world = make_world(
        &map,
        (start_pos, goal_pos),
        |_cur, next, tile| is_valid(next, &map) && tile != next,
        |next| is_intersection(next, &map) || next == goal_pos,
    );
    let p2 = compute_paths(&p2_world, start, 0, &mut make_array::<MAX_NODES, bool>());

    (p1, p2)
}

fn compute_paths(
    world: &World,
    start: Node,
    cur_cost: usize,
    visited: &mut ArrayVec<bool, MAX_NODES>,
) -> usize {
    // dead-end
    if visited[start.id] {
        return 0;
    }

    if let Some(reached_goal) = world.graph[start.id]
        .iter()
        .find(|(n, _d)| *n == world.goal)
    {
        cur_cost + reached_goal.1
    } else {
        // peel off neighboring nodes and recurse
        let mut max_d = 0;
        visited[start.id] = true;
        for &(next, cost) in &world.graph[start.id] {
            max_d = max(max_d, compute_paths(world, next, cur_cost + cost, visited));
        }
        visited[start.id] = false;
        max_d
    }
}

fn bfs<V: Fn(Pos, Pos) -> bool, R: Fn(Pos) -> bool>(
    buf: &mut BfsBuf,
    is_valid_pos: V,
    is_result: R,
) {
    let BfsBuf { q, seen, res } = buf;

    while let Some((cur, t)) = q.pop_front() {
        if seen.contains(&cur) {
            continue;
        }
        seen.insert(cur);

        for next in cur.neighbors_simple() {
            if is_valid_pos(cur, next) {
                if is_result(next) {
                    res.push((Node::from(next), t + 1));
                } else {
                    q.push_back((next, t + 1))
                }
            }
        }
    }
}

fn diode_offset(c: char) -> Pos {
    Pos::from(match c {
        '<' => (-1, 0),
        '>' => (1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => unreachable!(),
    })
}

fn is_diode(p: Pos, map: &Map<char>) -> bool {
    matches!(map.get(p), Some('<' | '>' | '^' | 'v'))
}

fn is_against(cur: Pos, next: Pos, map: &Map<char>) -> bool {
    let offset = next - cur;
    let next_char = map.get(next);
    if next_char.is_none() {
        return false;
    }

    matches!(
        (offset, next_char.unwrap()),
        (Pos { x: -1, y: 0 }, '>')
            | (Pos { x: 1, y: 0 }, '<')
            | (Pos { x: 0, y: -1 }, 'v')
            | (Pos { x: 0, y: 1 }, '^')
    )
}

fn is_valid(p: Pos, map: &Map<char>) -> bool {
    let c = map.get(p);
    c.is_some() && c.unwrap() != '#'
}

fn is_intersection(p: Pos, map: &Map<char>) -> bool {
    (map.get(p) == Some('.') || is_diode(p, map))
        && p.neighbors_simple().filter(|&p| is_valid(p, map)).count() > 2
}

aoc_2023::main! {
    solve(include_str!("../../inputs/23.in"))
}
