use aoc_2dmap::prelude::{Map, Pos};
use aoc_prelude::{lazy_static, HashSet};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::iter::once;
use std::sync::Mutex;

type Graph = BTreeMap<usize, Vec<(usize, usize)>>;

lazy_static! {
    static ref COUNTER: Mutex<usize> = Mutex::new(0);
}

fn get_id(p: Pos, cache: &mut HashMap<Pos, usize>) -> usize {
    let id = cache.entry(p).or_insert_with(|| {
        let mut counter = COUNTER.lock().unwrap();
        *counter += 1;
        *counter
    });
    *id
}

fn solve(input: &str) -> (usize, usize) {
    let lines = input.lines().collect::<Vec<_>>();

    let size = (lines[0].len(), lines.len());

    let map = Map::new(size, lines.join("").chars());

    let mut pos_to_num = HashMap::new();

    let start = Pos::new(1, 0);
    let start_id = get_id(start, &mut pos_to_num);
    let goal = Pos::new(size.0 - 2, size.1 - 1);
    let goal_id = get_id(goal, &mut pos_to_num);

    let mut p1_graph = BTreeMap::new();
    for slope in map.iter().filter(|&p| is_diode(p, &map)).chain(once(start)) {
        p1_graph.insert(
            get_id(slope, &mut pos_to_num),
            p1_bfs(slope, goal, &map)
                .iter()
                .map(|(to, cost)| (get_id(*to, &mut pos_to_num), *cost))
                .collect::<Vec<_>>(),
        );
    }

    let mut paths = Vec::new();
    compute_paths(&p1_graph, start_id, goal_id, 0, &mut paths);
    let p1 = *paths.iter().max().unwrap();

    //--------------------------------------------------------------------------

    let mut p2_graph = BTreeMap::new();
    for inter in map
        .iter()
        .filter(|&p| is_intersection(p, &map))
        .chain(once(start))
    {
        p2_graph.insert(
            get_id(inter, &mut pos_to_num),
            p2_bfs(inter, goal, &map)
                .iter()
                .map(|(to, cost)| (get_id(*to, &mut pos_to_num), *cost))
                .collect::<Vec<_>>(),
        );
    }

    let available = p2_graph.keys().copied().collect::<HashSet<_>>();

    // now get all paths to the end, without going in cycles
    let mut paths = Vec::new();
    compute_paths_2(&p2_graph, start_id, goal_id, available, 0, &mut paths);
    let p2 = *paths.iter().max().unwrap();

    (p1, p2)
}
fn compute_paths_2(
    graph: &Graph,
    start: usize,
    goal: usize,
    available: HashSet<usize>,
    cur_cost: usize,
    paths: &mut Vec<usize>,
) {
    // dead-end
    if !graph.contains_key(&start) || !available.contains(&start) {
        return;
    }
    // reached goal
    if let Some(reached_goal) = graph[&start].iter().find(|(n, _d)| *n == goal) {
        paths.push(cur_cost + reached_goal.1);
        return;
    }
    // peel off a neighboring node and recurse
    for &(n, d) in &graph[&start] {
        let mut new_avail = available.clone();
        new_avail.remove(&start);
        compute_paths_2(graph, n, goal, new_avail, cur_cost + d, paths);
    }
}

fn compute_paths(
    graph: &Graph,
    start: usize,
    goal: usize,
    cur_cost: usize,
    paths: &mut Vec<usize>,
) {
    // dead-end
    if !graph.contains_key(&start) {
        return;
    }
    // reached goal
    if let Some(reached_goal) = graph[&start].iter().find(|(n, _d)| *n == goal) {
        paths.push(cur_cost + reached_goal.1);
        return;
    }
    // peel off a neighboring node and recurse
    for &(n, d) in &graph[&start] {
        compute_paths(graph, n, goal, cur_cost + d, paths);
    }
}

fn p2_bfs(start: Pos, goal: Pos, map: &Map<char>) -> Vec<(Pos, usize)> {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    let mut res = Vec::new();
    q.push_back((start, 0_usize));

    while let Some((cur, t)) = q.pop_front() {
        if seen.contains(&cur) {
            continue;
        }
        seen.insert(cur);

        for next in cur.neighbors_simple() {
            if next == start {
                continue;
            }
            if is_valid(next, map) {
                if is_intersection(next, map) || next == goal {
                    res.push((next, t + 1));
                } else {
                    q.push_back((next, t + 1))
                }
            }
        }
    }
    res
}

fn p1_bfs(start: Pos, goal: Pos, map: &Map<char>) -> BTreeMap<Pos, usize> {
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    let mut res = BTreeMap::new();
    q.push_back(if is_diode(start, map) {
        (start + diode_offset(map.get_unchecked(start)), 1)
    } else {
        (start, 0)
    });

    while let Some((cur, t)) = q.pop_front() {
        if seen.contains(&cur) {
            continue;
        }
        seen.insert(cur);

        for next in cur.neighbors_simple() {
            if is_valid(next, map) && !is_against(cur, next, map) {
                if is_diode(next, map) || next == goal {
                    res.insert(next, t + 1);
                } else {
                    q.push_back((next, t + 1))
                }
            }
        }
    }
    res
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
