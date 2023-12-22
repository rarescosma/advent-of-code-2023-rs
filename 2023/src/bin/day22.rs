use aoc_prelude::{ArrayVec, HashSet};
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fmt::Debug;

const FLOOR: i32 = 1;
const BRICK_NUM: usize = 2048;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Default)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

fn cuts((s0, s1): (i32, i32), (f0, f1): (i32, i32)) -> bool {
    max(s0, f0) <= min(s1, f1)
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Default)]
struct Brick {
    o: Point,
    l: Point,
}

impl Brick {
    fn intersects(&self, other: &Brick) -> bool {
        cuts((self.o.x, self.l.x), (other.o.x, other.l.x))
            && cuts((self.o.y, self.l.y), (other.o.y, other.l.y))
            && cuts((self.o.z, self.l.z), (other.o.z, other.l.z))
    }

    fn descend_to(self, z: i32) -> Brick {
        let (mut o, mut l) = (self.o, self.l);
        let z_diff = l.z - o.z;
        o.z = z;
        l.z = z + z_diff;
        Brick { o, l }
    }

    fn extend_to(&self, z: i32) -> Brick {
        let mut o = self.o;
        o.z = z;
        Brick { o, l: self.l }
    }
}

fn extract_nums(line: &str) -> Vec<i32> {
    line.replace(['~', ','], " ")
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect()
}

fn estimate_subtree(
    idx: usize,
    adj: &[(HashSet<usize>, HashSet<usize>)],
    would_fall: &mut HashSet<usize>,
    deq: &mut VecDeque<usize>,
) {
    deq.clear();
    would_fall.clear();

    deq.push_back(idx);
    while let Some(n) = deq.pop_front() {
        would_fall.insert(n);
        for &resting_on_us in &adj[n].0 {
            // a block that's resting on us has all shaky foundations
            if adj[resting_on_us].1.iter().all(|x| would_fall.contains(x)) {
                deq.push_back(resting_on_us);
            }
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut bricks = ArrayVec::<Brick, BRICK_NUM>::new();
    bricks.extend(input.lines().map(|l| {
        let nums = extract_nums(l);
        let nums = nums.as_slice();
        Brick {
            o: Point::new(nums[0], nums[1], nums[2]),
            l: Point::new(nums[3], nums[4], nums[5]),
        }
    }));

    bricks.sort_by(|b0, b1| b0.o.z.cmp(&b1.o.z));

    let mut stack = ArrayVec::<Brick, BRICK_NUM>::new();
    let mut intersects = ArrayVec::<(usize, i32), BRICK_NUM>::new();

    // (being_rested_on => resting_on, resting_on => being_rested_on)
    let mut adj = ArrayVec::<_, BRICK_NUM>::from_iter(
        (0..bricks.len()).map(|_| (HashSet::new(), HashSet::new())),
    );

    for (idx, brick) in bricks.iter().enumerate() {
        if stack.is_empty() {
            stack.push(brick.descend_to(FLOOR));
            continue;
        }

        // stack is not empty => "extend" the current brick all the way to z=0
        // check what the extended version intersects (from the stack)
        // pop the highest value(s) z (maybe this is where you build the graph?)
        // and add the z + 1 descended brick to the stack
        let extended = brick.extend_to(0);

        let mut highest_z = FLOOR;
        intersects.clear();

        for (s_idx, s_brick) in stack.iter().enumerate() {
            if extended.intersects(s_brick) {
                intersects.push((s_idx, s_brick.l.z));
                highest_z = max(highest_z, s_brick.l.z);
            }
        }

        intersects
            .iter()
            .filter(|(_, i_z)| *i_z == highest_z)
            .for_each(|&(i_idx, _)| {
                // idx is resting on i_idx
                adj[i_idx].0.insert(idx);
                adj[idx].1.insert(i_idx);
            });

        stack.push(brick.descend_to(highest_z + 1));
    }

    let (mut p1, mut p2) = (0, 0);
    let mut would_fall = HashSet::new();
    let mut deq = VecDeque::new();

    for idx in 0..adj.len() {
        estimate_subtree(idx, &adj, &mut would_fall, &mut deq);
        if would_fall.len() == 1 {
            p1 += 1;
        }
        p2 += would_fall.len() - 1;
    }

    (p1, p2)
}

aoc_2023::main! {
    solve(include_str!("../../inputs/22.in"))
}
