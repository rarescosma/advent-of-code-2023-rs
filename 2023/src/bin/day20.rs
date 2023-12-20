use aoc_prelude::{ArrayVec, HashSet};
use std::collections::VecDeque;

const BCAST: &str = "bc";

#[derive(Debug, Clone)]
enum GateKind {
    Broadcast,
    Conj,
    FlipFLop,
}
impl From<&str> for GateKind {
    fn from(s: &str) -> Self {
        if s.starts_with('%') {
            Self::FlipFLop
        } else if s.starts_with('&') {
            Self::Conj
        } else {
            Self::Broadcast
        }
    }
}

#[derive(Debug)]
struct Gate<'a> {
    kind: GateKind,
    out: ArrayVec<&'a str, 32>,
}

impl<'a> Default for Gate<'a> {
    fn default() -> Self {
        Self {
            kind: GateKind::FlipFLop,
            out: ArrayVec::new(),
        }
    }
}

struct StackMap<T, const M: usize> {
    inner: ArrayVec<T, M>,
    is_set: ArrayVec<bool, M>,
}

impl<const M: usize, T: Default> StackMap<T, M> {
    fn new() -> Self {
        Self {
            inner: ArrayVec::from_iter((0..M).map(|_| T::default())),
            is_set: ArrayVec::from([false; M]),
        }
    }

    fn get(&self, k: &str) -> &T {
        &self.inner[StackMap::<T, M>::_key(k)]
    }

    fn get_mut(&mut self, k: &str) -> &mut T {
        &mut self.inner[StackMap::<T, M>::_key(k)]
    }

    fn contains_key(&self, k: &str) -> bool {
        self.is_set[StackMap::<T, M>::_key(k)]
    }

    fn set(&mut self, k: &str, val: T) {
        let key = StackMap::<T, M>::_key(k);
        self.is_set[key] = true;
        self.inner[key] = val;
    }

    fn _key(k: &str) -> usize {
        let mut c = k.chars();
        let t1 = c.next().unwrap() as u8 - b'a';
        let t2 = c.next().unwrap() as u8 - b'a';
        t1 as usize * 32 + t2 as usize
    }
}

type Circuit<'a> = StackMap<Gate<'a>, 1024>;
type Rev<'a> = StackMap<ArrayVec<&'a str, 32>, 1024>;
type State<'a> = StackMap<bool, 1024>;

struct World<'a> {
    circuit: Circuit<'a>,
    rev: Rev<'a>,
    state: State<'a>,
    rx_cycles: ArrayVec<usize, 16>,
    rx_inputs: HashSet<&'a str>,
}

impl<'a> World<'a> {
    fn tick(&mut self, q_buf: &mut VecDeque<(&'a str, &'a str, bool)>, t: usize) -> (usize, usize) {
        let (mut lo, mut hi) = (1, 0);
        q_buf.clear();
        for b_node in self.circuit.get(BCAST).out.iter() {
            q_buf.push_back((BCAST, *b_node, false));
        }

        while let Some((inp, out, pulse)) = q_buf.pop_front() {
            // println!("{inp} sends {pulse} to {out}");
            if pulse {
                hi += 1;
            } else {
                lo += 1;
            }
            if !self.circuit.contains_key(out) {
                continue;
            }
            self.state.set(inp, pulse);
            // *self.state.get_mut(inp).unwrap() = pulse;
            let out_gate = self.circuit.get(out);
            match out_gate.kind {
                GateKind::FlipFLop if !pulse => {
                    let state = self.state.get(out);
                    for downstream in &out_gate.out {
                        q_buf.push_back((out, downstream, !state));
                    }
                    self.state.set(out, !state);
                    // *state = !*state;
                }
                GateKind::Conj => {
                    let send = !self.rev.get(out).iter().all(|x| *self.state.get(x));
                    if send && self.rx_inputs.contains(out) {
                        self.rx_cycles.push(t);
                    }
                    for downstream in &out_gate.out {
                        q_buf.push_back((out, downstream, send));
                    }
                }
                _ => {}
            }
        }

        (lo, hi)
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut circuit = Circuit::new();
    let mut rev = Rev::new();
    let mut state = State::new();

    input.lines().for_each(|l| {
        let (name, rest) = l.split_once(" -> ").unwrap();
        let out = rest.split(", ").collect::<ArrayVec<_, 32>>();
        let kind = GateKind::from(name);
        let real_name = if !matches!(kind, GateKind::Broadcast) {
            name.split_at(1).1
        } else {
            BCAST
        };
        state.set(real_name, false);
        for output in out.iter() {
            rev.get_mut(output).push(real_name);
        }
        circuit.set(real_name, Gate { kind, out });
    });

    let rx_inputs = rev
        .get(rev.get("rx").first().unwrap())
        .iter()
        .copied()
        .collect();

    let mut world = World {
        circuit,
        rev,
        state,
        rx_inputs,
        rx_cycles: ArrayVec::new(),
    };

    let mut q_buf = VecDeque::new();
    let (lo, hi) = (1..=1000)
        .map(|t| world.tick(&mut q_buf, t))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    let p1 = lo * hi;

    for t in 1001.. {
        if world.rx_cycles.len() == world.rx_inputs.len() {
            break;
        }
        world.tick(&mut q_buf, t);
    }

    let p2 = world.rx_cycles.iter().product();

    (p1, p2)
}

aoc_2023::main! {
    solve(include_str!("../../inputs/20.in"))
}
