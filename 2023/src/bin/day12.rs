use aoc_prelude::{HashMap, Itertools};
use std::iter;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct State {
    char_idx: usize,
    run_idx: usize,
    current_run: usize,
}

impl State {
    fn new(char_idx: usize, run_idx: usize, current_run: usize) -> Self {
        Self {
            char_idx,
            run_idx,
            current_run,
        }
    }

    fn advance_char(&self) -> Self {
        Self::new(self.char_idx + 1, self.run_idx, self.current_run)
    }

    fn start_next_run(&self) -> Self {
        Self::new(self.char_idx, self.run_idx + 1, 0)
    }

    fn increase_run(&self) -> Self {
        Self::new(self.char_idx, self.run_idx, self.current_run + 1)
    }
}

struct World<'a> {
    cfg: &'a str,
    runs: &'a Vec<usize>,
    cache: &'a mut HashMap<State, usize>,
}

fn find_combos(world: &mut World, state: State) -> usize {
    if let Some(&cached) = world.cache.get(&state) {
        return cached;
    }

    let (cfg, runs) = (world.cfg, world.runs);
    let State {
        char_idx,
        run_idx,
        current_run,
    } = state;

    if char_idx == cfg.len() {
        return usize::from(
            (run_idx == runs.len() && current_run == 0)
                || (run_idx == runs.len() - 1 && runs[run_idx] == current_run),
        );
    }

    let mut ans = 0;
    let cur_char = &cfg[char_idx..=char_idx];
    let is_wildcard = cur_char == "?";

    if is_wildcard || cur_char == "." {
        if current_run == 0 {
            // we placed a '.' and we're not currently in a run => advance the char idx
            ans += find_combos(world, state.advance_char());
        } else if run_idx < runs.len() && runs[run_idx] == current_run {
            // we placed a '.' during a matching run, successfully completing it
            // => advance the char idx & start a new run
            ans += find_combos(world, state.advance_char().start_next_run());
        }
    }

    if is_wildcard || cur_char == "#" {
        // we placed a '#' => increase the current run
        ans += find_combos(world, state.advance_char().increase_run());
    }

    world.cache.insert(state, ans);
    ans
}

fn solve() -> (usize, usize) {
    let mut cache = HashMap::with_capacity(4096);
    let mut runs1 = Vec::with_capacity(4096);
    let mut runs2 = Vec::with_capacity(4096);

    let (p1, p2) = include_str!("../../inputs/12.in")
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
