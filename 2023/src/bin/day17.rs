use aoc_2023::ConstMap;
use aoc_2dmap::prelude::Pos;
use aoc_dijsktra::{Dijsktra, GameState, Transform};
use aoc_prelude::ArrayVec;

type Direction = Pos;

// East, South, West, North
const OFFSETS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn is_opposite(dir: Direction, to: Direction) -> bool {
    dir.x == -to.x && dir.y == -to.y
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone, Default)]
struct State {
    cur: Pos,
    direction: Direction,
}

struct Move {
    to: Pos,
    cost: u32,
}

impl<'a, const M: usize> GameState<LavaCtx<'a, M>, u32> for State {
    type Steps = ArrayVec<Move, M>;

    fn accept(&self, _cost: u32, ctx: &mut LavaCtx<M>) -> bool {
        self.cur == ctx.goal
    }

    fn steps(&self, ctx: &mut LavaCtx<M>) -> Self::Steps {
        let mut steps = ArrayVec::new();
        for o in &OFFSETS {
            let o = Pos::from(*o);
            if is_opposite(o, self.direction) || o == self.direction {
                continue;
            }
            let mut cost = 0;
            for dist in 1..=ctx.max_straight {
                let to = self.cur + Pos::new(o.x * dist, o.y * dist);

                if let Some(step_cost) = ctx.map.get(to) {
                    cost += step_cost;
                    if ctx.min_straight.is_some_and(|m| dist < m) {
                        continue;
                    }
                    steps.push(Move { to, cost });
                }
            }
        }
        steps
    }
}

impl Transform<State, u32> for Move {
    fn cost(&self) -> u32 {
        self.cost
    }

    fn transform(&self, state: &State) -> State {
        State {
            cur: self.to,
            direction: (self.to - state.cur).signum(),
        }
    }
}

struct LavaCtx<'a, const M: usize> {
    map: &'a ConstMap<M, u32>,
    goal: Pos,
    min_straight: Option<i32>,
    max_straight: i32,
}

fn solve() -> (u32, u32) {
    let map = include_str!("../../inputs/17.in")
        .replace('\n', "")
        .trim()
        .parse::<ConstMap<141, u32>>()
        .expect("nope");

    let goal = (map.size() - 1, map.size() - 1).into();

    let init_state = State::default();

    let p1 = init_state.dijsktra(&mut LavaCtx {
        map: &map,
        max_straight: 3,
        min_straight: None,
        goal,
    });

    //--------------------------------------------------------------------------

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
