use aoc_prelude::{ArrayVec, HashSet};
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fmt::Debug;

const FLOOR: i32 = 0;
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
    }

    fn descend_to(&mut self, z: i32) {
        self.l.z += z - self.o.z;
        self.o.z = z;
    }
}

fn extract_nums(line: &str) -> Vec<i32> {
    line.replace(['~', ','], " ")
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect()
}

fn get_adj(
    bricks: &mut [Brick],
) -> (
    ArrayVec<HashSet<usize>, BRICK_NUM>,
    ArrayVec<HashSet<usize>, BRICK_NUM>,
) {
    bricks.sort_by(|b0, b1| b0.o.z.cmp(&b1.o.z));

    let mut stack = ArrayVec::<&mut Brick, BRICK_NUM>::new();
    let mut intersects = ArrayVec::<_, BRICK_NUM>::new();

    // (being_rested_on => resting_on, resting_on => being_rested_on)
    let mut supports =
        ArrayVec::<_, BRICK_NUM>::from_iter((0..bricks.len()).map(|_| HashSet::new()));
    let mut is_supported_by =
        ArrayVec::<_, BRICK_NUM>::from_iter((0..bricks.len()).map(|_| HashSet::new()));

    for (idx, brick) in bricks.iter_mut().enumerate() {
        // stack is not empty => "extend" the current brick all the way to z=0
        // check what the extended version intersects (from the stack)
        // pop the highest value(s) z (maybe this is where you build the graph?)
        // and add the z + 1 descended brick to the stack

        let mut highest_z = FLOOR;
        intersects.clear();

        for (s_idx, s_brick) in stack.iter().enumerate() {
            if brick.intersects(s_brick) {
                intersects.push((s_idx, s_brick.l.z));
                highest_z = max(highest_z, s_brick.l.z);
            }
        }

        intersects
            .iter()
            .filter(|(_, i_z)| *i_z == highest_z)
            .for_each(|&(i_idx, _)| {
                // idx is resting on i_idx
                supports[i_idx].insert(idx);
                is_supported_by[idx].insert(i_idx);
            });

        brick.descend_to(highest_z + 1);
        stack.push(brick);
    }
    (supports, is_supported_by)
}

fn disintegration_is_the_best_album_ever(
    idx: usize,
    supports: &[HashSet<usize>],
    is_supported_by: &[HashSet<usize>],
    buf: &mut (HashSet<usize>, VecDeque<usize>),
) -> usize {
    let (would_fall, deq) = buf;
    deq.clear();
    would_fall.clear();

    deq.push_back(idx);
    while let Some(n) = deq.pop_front() {
        would_fall.insert(n);
        for &supported_by in &supports[n] {
            // "n" is supported by us but all of its supports would fall
            if is_supported_by[supported_by].difference(would_fall).count() == 0 {
                deq.push_back(supported_by);
            }
        }
    }
    would_fall.len() - 1
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

    let (supports, is_supported_by) = get_adj(&mut bricks);

    let (mut p1, mut p2) = (0, 0);
    let mut buf = (HashSet::new(), VecDeque::new());

    for idx in 0..supports.len() {
        let would_fall =
            disintegration_is_the_best_album_ever(idx, &supports, &is_supported_by, &mut buf);
        if would_fall == 0 {
            p1 += 1;
        }
        p2 += would_fall;
    }

    (p1, p2)
}

aoc_2023::main! {
    solve(include_str!("../../inputs/22.in"))
}
