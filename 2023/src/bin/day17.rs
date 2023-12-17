use aoc_2dmap::prelude::{Map, Pos};
use aoc_dijsktra::{Dijsktra, GameState, Transform};
use aoc_prelude::{lazy_static, ArrayVec};
use std::hash::Hash;

lazy_static! {
    // North, South, East, West
    static ref OFFSETS: [Pos; 4] = [(0, -1).into(), (1, 0).into(), (0, 1).into(), (-1, 0).into()];
}

fn is_opposite(dir: usize, to: usize) -> bool {
    (dir + 2) % 4 == to
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
    cost: u32,
}

impl GameState<LavaCtx> for State {
    type Steps = ArrayVec<Move, 4>;

    fn accept(&self, _cost: usize, ctx: &mut LavaCtx) -> bool {
        ctx.ultra_check(self.went_straight) && self.cur == self.goal
    }

    fn steps(&self, ctx: &mut LavaCtx) -> Self::Steps {
        if self.direction.is_none() {
            return self
                .cur
                .neighbors_simple()
                .filter_map(|to| ctx.map.get(to).map(|cost| Move { to, cost }))
                .collect();
        }

        let direction = self.direction.unwrap();

        OFFSETS
            .into_iter()
            .map(|o| self.cur + o)
            .enumerate()
            .filter_map(|(new_direction, to)| {
                ctx.map.get(to).and_then(|cost| {
                    if new_direction == direction && self.went_straight < ctx.max_straight {
                        return Some(Move { to, cost });
                    }
                    if new_direction != direction
                        && !is_opposite(new_direction, direction)
                        && ctx.ultra_check(self.went_straight)
                    {
                        return Some(Move { to, cost });
                    }
                    None
                })
            })
            .collect()
    }
}

impl Transform<State> for Move {
    fn cost(&self) -> usize {
        self.cost as usize
    }

    fn transform(&self, game_state: &State) -> State {
        let offset = self.to - game_state.cur;
        let new_direction = OFFSETS.into_iter().position(|x| x == offset);

        let went_straight =
            1 + if game_state.direction.is_none() || new_direction == game_state.direction {
                // continue if direction matches (or we're at the beginning)
                game_state.went_straight
            } else {
                // reset if direction changed
                0
            };

        State {
            cur: self.to,
            goal: game_state.goal,
            direction: new_direction,
            went_straight,
        }
    }
}

struct LavaCtx {
    map: Map<u32>,
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
        input.join("").chars().map(|c| c.to_digit(10).unwrap()),
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
