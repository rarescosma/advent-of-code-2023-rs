use aoc_2dmap::prelude::{Map, Pos};
use aoc_dijsktra::{Dijsktra, GameState, Transform};

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
    cost: usize,
}

impl<'a> GameState<LavaCtx<'a>> for State {
    type Steps = Vec<Move>;

    fn accept(&self, _cost: usize, ctx: &mut LavaCtx) -> bool {
        self.cur == ctx.goal
    }

    fn steps(&self, ctx: &mut LavaCtx) -> Self::Steps {
        let mut steps = Vec::new();
        for o in OFFSETS.iter() {
            let o = Pos::from(*o);
            if is_opposite(o, self.direction) || o == self.direction {
                continue;
            }
            let mut cost = 0;
            for dist in 1..=ctx.max_straight {
                let to = self.cur + Pos::new(o.x * dist as i32, o.y * dist as i32);

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

impl Transform<State> for Move {
    fn cost(&self) -> usize {
        self.cost
    }

    fn transform(&self, state: &State) -> State {
        State {
            cur: self.to,
            direction: (self.to - state.cur).signum(),
        }
    }
}

struct LavaCtx<'a> {
    map: &'a Map<usize>,
    goal: Pos,
    min_straight: Option<usize>,
    max_straight: usize,
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
