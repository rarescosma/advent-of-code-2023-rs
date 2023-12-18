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

#[derive(PartialOrd, Ord, Eq, PartialEq, Hash, Clone, Copy, Default)]
struct State {
    cur: Pos,
    direction: Option<usize>,
    went_straight: usize,
}

struct Move {
    to: Pos,
    cost: usize,
}

impl<'a> GameState<LavaCtx<'a>> for State {
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

struct LavaCtx<'a> {
    map: &'a Map<usize>,
    goal: Pos,
    min_straight: Option<usize>,
    max_straight: usize,
}

impl<'a> LavaCtx<'a> {
    fn check_min(&self, went_straight: usize) -> bool {
        match self.min_straight {
            None => true,
            Some(min_straight) => went_straight >= min_straight,
        }
    }
    fn is_valid_move(&self, s: &State, new_direction: usize) -> bool {
        match s.direction {
            None => true, // must be starting, anything goes :-)
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
    let input = include_str!("../../inputs/17.in")
        .lines()
        .collect::<Vec<_>>();

    let size = (input[0].len(), input.len());

    let map = Map::new(
        size,
        input
            .join("")
            .chars()
            .filter_map(|c| c.to_digit(10).map(|c| c as usize)),
    );
    let goal = map.size + (-1, -1).into();

    let init_state = State::default();

    let p1 = init_state.dijsktra(&mut LavaCtx {
        map: &map,
        max_straight: 3,
        min_straight: None,
        goal,
    });

    //-------------------------------------------------------------------------

    let p2 = init_state.dijsktra(&mut LavaCtx {
        map: &map,
        max_straight: 10,
        min_straight: Some(4),
        goal,
    });

    (p1.expect("failed p1"), p2.expect("failed p2"))
}

aoc_2023::main! {
    solve()
}
