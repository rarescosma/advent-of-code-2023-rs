use aoc_2dmap::prelude::{Map, Pos};
use aoc_dijsktra::{Dijsktra, GameState, Transform};
use aoc_prelude::ArrayVec;
use std::hash::Hash;

// West, North, East, South
const OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

// 00, 01, 10, 11
fn pos_to_dir(p: Pos) -> usize {
    let signum = p.x + p.y;
    let msb = if signum < 0 { 0 } else { 1 };
    let lsb = if p.y == 0 { 0 } else { 1 };
    lsb + (msb << 1)
}

fn is_opposite(dir: usize, to: usize) -> bool {
    (dir + 2) % 4 == to
}

#[derive(PartialOrd, Ord, Eq, PartialEq, Hash, Clone, Copy)]
struct State {
    cur: Pos,
    direction: Option<usize>,
    went_straight: usize,
}

struct Move {
    to: Pos,
    cost: usize,
}

impl GameState<LavaCtx> for State {
    type Steps = ArrayVec<Move, 4>;

    fn accept(&self, _cost: usize, ctx: &mut LavaCtx) -> bool {
        ctx.check_min(self.went_straight) && self.cur == ctx.goal
    }

    fn steps(&self, ctx: &mut LavaCtx) -> Self::Steps {
        OFFSETS
            .into_iter()
            .map(|o| self.cur + o.into())
            .enumerate()
            .filter_map(|(new_direction, to)| {
                ctx.map.get(to).and_then(|cost| {
                    if ctx.is_valid_move(self, new_direction) {
                        Some(Move { to, cost })
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

impl Transform<State> for Move {
    fn cost(&self) -> usize {
        self.cost
    }

    fn transform(&self, state: &State) -> State {
        let mut new_state = State {
            cur: self.to,
            direction: Some(pos_to_dir(self.to - state.cur)),
            went_straight: 1,
        };

        if new_state.direction == state.direction {
            new_state.went_straight += state.went_straight
        }
        new_state
    }
}

struct LavaCtx {
    map: Map<usize>,
    goal: Pos,
    min_straight: Option<usize>,
    max_straight: usize,
}

impl LavaCtx {
    fn check_min(&self, went_straight: usize) -> bool {
        match self.min_straight {
            None => true,
            Some(min_straight) => went_straight >= min_straight,
        }
    }
    fn is_valid_move(&self, s: &State, new_direction: usize) -> bool {
        match s.direction {
            None => true,
            Some(direction) => {
                (new_direction == direction && s.went_straight < self.max_straight)
                    || (new_direction != direction
                        && !is_opposite(new_direction, direction)
                        && self.check_min(s.went_straight))
            }
        }
    }
}

fn solve() -> (usize, usize) {
    let input = include_str!("../../inputs/day17.txt")
        .lines()
        .collect::<Vec<_>>();

    let size = (input[0].len(), input.len());

    let map = Map::new(
        size,
        input
            .join("")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize),
    );
    let goal = map.size + (-1, -1).into();

    let init_state = State {
        cur: Pos::default(),
        direction: None,
        went_straight: 0,
    };

    let mut p1_ctx = LavaCtx {
        map: map.clone(),
        max_straight: 3,
        min_straight: None,
        goal,
    };
    let p1 = init_state.dijsktra(&mut p1_ctx).expect("no shortest path!");

    //-------------------------------------------------------------------------

    let mut p2_ctx = LavaCtx {
        map,
        max_straight: 10,
        min_straight: Some(4),
        goal,
    };
    let p2 = init_state.dijsktra(&mut p2_ctx).expect("no shortest path!");

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
