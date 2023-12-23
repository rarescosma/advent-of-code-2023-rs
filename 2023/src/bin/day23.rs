use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{lazy_static, ArrayVec, HashMap, HashSet};
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

fn make_array<const M: usize, T: Default>() -> ArrayVec<T, M> {
    ArrayVec::<T, M>::from_iter((0..M).map(|_| T::default()))
}

fn make_world<R: Fn(Pos) -> bool, M: Fn(Pos) -> ArrayVec<Pos, 4>>(
    map: &Map<char>,
    (start_pos, goal_pos): (Pos, Pos),
    make_neighbors: M,
    is_poi: R,
) -> World {
    let goal = Node::from(goal_pos);

    let mut graph = make_array::<MAX_NODES, Edges>();
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    for tile in map.iter().filter(|&p| is_poi(p)).chain(once(start_pos)) {
        q.clear();
        q.push_back((tile, 0));
        seen.clear();
        graph[Node::from(tile).id] = bfs(&mut q, &mut seen, &make_neighbors, &is_poi, map);
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
        |p| make_neighbors_p1(p, &map),
        |p| is_intersection(p, &map) || p == goal_pos,
    );
    let p1 = compute_paths(&p1_world, start, 0, &mut make_array::<MAX_NODES, bool>());

    //--------------------------------------------------------------------------
    let p2_world = make_world(
        &map,
        (start_pos, goal_pos),
        |p| ArrayVec::from_iter(p.neighbors_simple()),
        |p| is_intersection(p, &map) || p == goal_pos,
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
        visited[start.id] = true;
        let res = world.graph[start.id]
            .iter()
            .map(|&(next, cost)| compute_paths(world, next, cur_cost + cost, visited))
            .max()
            .unwrap();
        visited[start.id] = false;
        res
    }
}

fn bfs<R: Fn(Pos) -> bool, M: Fn(Pos) -> ArrayVec<Pos, 4>>(
    q: &mut VecDeque<(Pos, usize)>,
    seen: &mut HashSet<Pos>,
    make_neighbors: M,
    is_poi: R,
    map: &Map<char>,
) -> Edges {
    let mut res = Edges::new();

    while let Some((cur, t)) = q.pop_front() {
        if seen.contains(&cur) {
            continue;
        }
        seen.insert(cur);

        for next in make_neighbors(cur)
            .into_iter()
            .filter(|&p| is_valid(p, map) && !seen.contains(&p))
        {
            if is_poi(next) {
                res.push((Node::from(next), t + 1));
            } else {
                q.push_back((next, t + 1))
            }
        }
    }
    res
}

fn make_neighbors_p1(p: Pos, map: &Map<char>) -> ArrayVec<Pos, 4> {
    let mut ret = ArrayVec::new();
    match map.get(p) {
        Some('.') => ret.extend(p.neighbors_simple()),
        Some('<') => ret.push(p + (-1, 0).into()),
        Some('>') => ret.push(p + (1, 0).into()),
        Some('^') => ret.push(p + (0, -1).into()),
        Some('v') => ret.push(p + (0, 1).into()),
        _ => {}
    };
    ret
}

fn is_valid(p: Pos, map: &Map<char>) -> bool {
    matches!(map.get(p), Some(c) if c != '#')
}

fn is_intersection(p: Pos, map: &Map<char>) -> bool {
    is_valid(p, map) && p.neighbors_simple().filter(|&p| is_valid(p, map)).count() > 2
}

aoc_2023::main! {
    solve(include_str!("../../inputs/23.in"))
}
