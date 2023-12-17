use aoc_2dmap::prelude::{Map, Pos};
use aoc_dijsktra::{Dijsktra, GameState, Transform};
use aoc_prelude::{lazy_static, ArrayVec};
use std::hash::Hash;

lazy_static! {
    // West, North, East, South
    // 00, 01, 10, 11
    static ref OFFSETS: [Pos; 4] = [(-1, 0).into(), (0, -1).into(), (1, 0).into(), (0, 1).into()];
}

fn is_opposite(dir: usize, to: usize) -> bool {
    (dir + 2) % 4 == to
}

fn pos_to_dir(p: Pos) -> usize {
    let signum = p.x + p.y;
    let msb = if signum < 0 { 0 } else { 1 };
    let lsb = if p.y == 0 { 0 } else { 1 };
    lsb + (msb << 1)
}

#[derive(PartialOrd, Ord, Eq, PartialEq, Hash, Clone, Copy)]
struct State {
    cur: Pos,
    goal: Pos,
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
        ctx.ultra_check(self.went_straight) && self.cur == self.goal
    }

    fn steps(&self, ctx: &mut LavaCtx) -> Self::Steps {
        let make_move = |(new_direction, to)| {
            ctx.map.get(to).and_then(|cost| match self.direction {
                None => Some(Move { to, cost }),
                Some(direction)
                    if new_direction == direction && self.went_straight < ctx.max_straight =>
                {
                    Some(Move { to, cost })
                }
                Some(direction)
                    if new_direction != direction
                        && !is_opposite(new_direction, direction)
                        && ctx.ultra_check(self.went_straight) =>
                {
                    Some(Move { to, cost })
                }
                _ => None,
            })
        };
        OFFSETS
            .into_iter()
            .map(|o| self.cur + o)
            .enumerate()
            .filter_map(make_move)
            .collect()
    }
}

impl Transform<State> for Move {
    fn cost(&self) -> usize {
        self.cost
    }

    fn transform(&self, state: &State) -> State {
        let new_direction = Some(pos_to_dir(self.to - state.cur));

        let went_straight = 1 + if new_direction == state.direction {
            state.went_straight
        } else {
            // None branch corresponds to initial state so 0 is fine
            0
        };

        State {
            cur: self.to,
            goal: state.goal,
            direction: new_direction,
            went_straight,
        }
    }
}

struct LavaCtx {
    map: Map<usize>,
    max_straight: usize,
    min_straight: Option<usize>,
}

impl LavaCtx {
    fn ultra_check(&self, went_straight: usize) -> bool {
        match self.min_straight {
            None => true,
            Some(min_straight) => went_straight >= min_straight,
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
    let init_state = State {
        cur: Pos::default(),
        goal: map.size + (-1, -1).into(),
        direction: None,
        went_straight: 0,
    };

    let mut p1_ctx = LavaCtx {
        map: map.clone(),
        max_straight: 3,
        min_straight: None,
    };
    let p1 = init_state.dijsktra(&mut p1_ctx).expect("no shortest path!");

    //-------------------------------------------------------------------------

    let mut p2_ctx = LavaCtx {
        map,
        max_straight: 10,
        min_straight: Some(4),
    };
    let p2 = init_state.dijsktra(&mut p2_ctx).expect("no shortest path!");

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
