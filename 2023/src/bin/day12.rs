use aoc_prelude::{HashMap, Itertools};
use std::iter;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct State {
    char_idx: usize,
    block_idx: usize,
    current_run: usize,
}

impl State {
    fn new(char_idx: usize, block_idx: usize, current_run: usize) -> Self {
        Self {
            char_idx,
            block_idx,
            current_run,
        }
    }

    fn advance_char(&self) -> Self {
        Self::new(self.char_idx + 1, self.block_idx, self.current_run)
    }

    fn start_next_block(&self) -> Self {
        Self::new(self.char_idx, self.block_idx + 1, 0)
    }

    fn increase_run(&self) -> Self {
        Self::new(self.char_idx, self.block_idx, self.current_run + 1)
    }
}

struct World<'a> {
    cfg: &'a str,
    runs: &'a Vec<usize>,
    cache: &'a mut HashMap<State, usize>,
}

fn find_combos(world: &mut World, state: State) -> usize {
    if world.cache.contains_key(&state) {
        return world.cache[&state];
    }

    let (cfg, runs) = (world.cfg, world.runs);
    let State {
        char_idx,
        block_idx,
        current_run,
    } = state;

    if char_idx == cfg.len() {
        return if (block_idx == runs.len() && current_run == 0)
            || (block_idx == runs.len() - 1 && runs[block_idx] == current_run)
        {
            1
        } else {
            0
        };
    }

    let mut ans = 0;
    let cur_char = &cfg[char_idx..=char_idx];

    for c in [".", "#"] {
        if cur_char == c || cur_char == "?" {
            if c == "." {
                if current_run == 0 {
                    // we placed a '.' and we're not currently building a block => advance the char idx
                    ans += find_combos(world, state.advance_char());
                } else if current_run > 0
                    && block_idx < runs.len()
                    && runs[block_idx] == current_run
                {
                    // we placed a '.' after successfully completing the current block
                    // => advance both char idx & start a new block
                    ans += find_combos(world, state.advance_char().start_next_block());
                }
            } else {
                // we placed a '#' => increase the current run
                ans += find_combos(world, state.advance_char().increase_run());
            }
        }
    }

    world.cache.insert(state, ans);
    ans
}

fn solve() -> (usize, usize) {
    let mut cache = HashMap::with_capacity(4096);
    let mut runs1 = Vec::with_capacity(4096);
    let mut runs2 = Vec::with_capacity(4096);

    let (p1, p2) = include_str!("../../inputs/day12.txt")
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();

            let cfg1 = words.next().unwrap();
            let cfg2 = iter::repeat(cfg1).take(5).join("?");

            runs1.clear();
            runs1.extend(
                words
                    .next()
                    .unwrap()
                    .split(',')
                    .filter_map(|x| x.parse::<usize>().ok()),
            );

            runs2.clear();
            runs2.extend(iter::repeat(&runs1).take(5).flatten());

            cache.clear();
            let mut world1 = World {
                cfg: cfg1,
                runs: &runs1,
                cache: &mut cache,
            };
            let a1 = find_combos(&mut world1, State::new(0, 0, 0));

            cache.clear();
            let mut world2 = World {
                cfg: &cfg2,
                runs: &runs2,
                cache: &mut cache,
            };
            let a2 = find_combos(&mut world2, State::new(0, 0, 0));

            (a1, a2)
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    (p1, p2)
}

aoc_2023::main! {
    solve()
}
