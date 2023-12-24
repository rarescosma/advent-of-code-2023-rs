use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{lazy_static, ArrayVec, Entry, HashMap, HashSet};
use std::collections::VecDeque;
use std::iter::once;
use std::sync::Mutex;

const MAX_NODES: usize = 512;
type Edges = ArrayVec<(Node, usize), 64>;
type Graph = ArrayVec<Edges, MAX_NODES>;

lazy_static! {
    static ref ID_MAKER: Mutex<(HashMap<Pos, usize>, usize)> = Mutex::new((HashMap::new(), 0));
}

fn make_array<const M: usize, T: Default>() -> ArrayVec<T, M> {
    ArrayVec::<T, M>::from_iter((0..M).map(|_| T::default()))
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone, Debug)]
struct Node {
    id: usize,
}

impl Node {
    fn make_id<P: AsRef<Pos>>(pos: P) -> usize {
        let pos = pos.as_ref();
        let mut guard = ID_MAKER.lock().unwrap();
        if guard.0.contains_key(pos) {
            guard.0[pos]
        } else {
            guard.1 += 1;
            let id = guard.1;
            guard.0.insert(*pos, id);
            id
        }
    }
}

impl<P: AsRef<Pos>> From<P> for Node {
    fn from(pos: P) -> Self {
        Self {
            id: Self::make_id(pos),
        }
    }
}

struct World {
    graph: Graph,
    start: Node,
    goal: Node,
    adj_masks: ArrayVec<u64, MAX_NODES>,
}

impl World {
    fn from_map<M: Fn(Pos) -> ArrayVec<Pos, 4>>(map: &Map<char>, make_neighbors: M) -> Self {
        // reset the ID maker so we can fit our seen & reachable sets within u64
        {
            let mut guard = ID_MAKER.lock().unwrap();
            guard.0.clear();
            guard.1 = 0;
        }

        let (start_pos, goal_pos) = (Pos::new(1, 0), map.size - Pos::new(2, 1));
        let start = Node::from(start_pos);
        let goal = Node::from(goal_pos);

        let mut graph = make_array::<MAX_NODES, Edges>();
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        for tile in map
            .iter()
            .filter(|&p| is_intersection(p, map))
            .chain(once(start_pos))
            .chain(once(goal_pos))
        {
            q.clear();
            q.push_back((tile, 0));
            seen.clear();

            let mut res = Edges::new();

            // BFS from this POI to adjacent POIs and stop there
            while let Some((cur, t)) = q.pop_front() {
                if seen.contains(&cur) {
                    continue;
                }
                seen.insert(cur);

                for next in make_neighbors(cur)
                    .into_iter()
                    .filter(|&p| is_valid(p, map) && !seen.contains(&p))
                {
                    if is_intersection(next, map) || next == goal_pos {
                        res.push((Node::from(next), t + 1));
                    } else {
                        q.push_back((next, t + 1))
                    }
                }
            }
            graph[Node::from(tile).id] = res;
        }

        // head straight to the goal if within reach
        graph[goal.id].clone().into_iter().for_each(|(n, cost)| {
            graph[n.id] = Edges::from_iter([(goal, cost)]);
        });

        let adj_masks: ArrayVec<u64, MAX_NODES> = graph
            .iter()
            .map(|edges| edges.iter().fold(0, |mask, (adj, _)| mask | (1 << adj.id)))
            .collect();

        World {
            graph,
            start,
            goal,
            adj_masks,
        }
    }

    // idea stolen from: https://github.com/mr-kaffee/aoc-2023
    fn reachable(&self, idx: usize, seen: u64) -> u64 {
        let mut queue = 1u64 << idx;
        let mut reached = seen | queue;

        while queue != 0 {
            let cur_idx = queue.trailing_zeros(); // get the idx back
            queue &= !(1 << cur_idx); // take the current idx out of the queue

            let mask = self.adj_masks[cur_idx as usize]; // 1 for all our neighbors
            queue |= mask & !reached; // extend queue with neighbors we haven't reached
            reached |= mask; // mark neighbors as reached
        }

        reached & !seen // what the BFS reached but we haven't seen yet => reachable
    }
}

fn solve(input: &str) -> (usize, usize) {
    let lines = input.lines().collect::<Vec<_>>();

    let size = (lines[0].len(), lines.len());
    let map = Map::new(size, lines.join("").chars());

    //--------------------------------------------------------------------------
    let p1_world = World::from_map(&map, |p| make_neighbors_p1(p, &map));
    let start = p1_world.start;
    let mut bests = HashMap::new();
    let p1 = compute_paths(&p1_world, start, 0, 0u64, &mut bests);

    //--------------------------------------------------------------------------
    let p2_world = World::from_map(&map, |p| ArrayVec::from_iter(p.neighbors_simple()));
    let start = p2_world.start;
    bests.clear();
    let p2 = compute_paths(&p2_world, start, 0, 0u64, &mut bests);

    (p1, p2)
}

fn compute_paths(
    world: &World,
    start: Node,
    cur_cost: usize,
    seen: u64,
    bests: &mut HashMap<(usize, u64), usize>,
) -> usize {
    if start == world.goal {
        return cur_cost;
    }

    // goal is not reachable
    let reachable = world.reachable(start.id, seen);
    if reachable & (1 << world.goal.id) == 0 {
        return 0;
    }

    // idea stolen from: https://github.com/mr-kaffee/aoc-2023
    match bests.entry((start.id, reachable)) {
        Entry::Occupied(o) if cur_cost <= *o.get() => return 0,
        Entry::Occupied(mut o) => *o.get_mut() = cur_cost,
        Entry::Vacant(v) => _ = v.insert(cur_cost),
    }

    // peel off neighboring nodes and recurse
    let new_seen = seen | 1_u64 << start.id;
    let res = world.graph[start.id]
        .iter()
        .filter(|(n, _)| seen & 1_u64 << n.id == 0)
        .map(|&(next, cost)| compute_paths(world, next, cur_cost + cost, new_seen, bests))
        .max()
        .unwrap_or(0);
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
