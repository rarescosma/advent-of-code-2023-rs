use aoc_prelude::{lazy_static, ArrayVec};
use std::collections::VecDeque;

lazy_static! {
    static ref TX_KEY: usize = stackmap_key("tx");
    static ref RX_KEY: usize = stackmap_key("rx");
}

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

struct Gate {
    kind: GateKind,
    out: ArrayVec<usize, 32>,
}

impl Default for Gate {
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

    fn get(&self, k: usize) -> &T {
        &self.inner[k]
    }

    fn get_mut(&mut self, k: usize) -> &mut T {
        &mut self.inner[k]
    }

    fn contains_key(&self, k: usize) -> bool {
        self.is_set[k]
    }

    fn set(&mut self, k: usize, val: T) {
        self.is_set[k] = true;
        self.inner[k] = val;
    }
}

fn stackmap_key(k: &str) -> usize {
    debug_assert!(k.len() == 2);
    let mut c = k.chars();
    let t1 = ((c.next().unwrap() as u8 - b'a') as usize) << 5;
    let t2 = c.next().unwrap() as u8 - b'a';
    t1 + t2 as usize
}

type Circuit = StackMap<Gate, 1024>;
type Rev = StackMap<ArrayVec<usize, 32>, 1024>;
type State = StackMap<bool, 1024>;

struct World {
    circuit: Circuit,
    rev: Rev,
    state: State,
    rx_cycles: ArrayVec<usize, 16>,
    rx_inputs: StackMap<(), 1024>,
}

impl World {
    fn tick(&mut self, q_buf: &mut VecDeque<(usize, usize, bool)>, t: usize) -> (usize, usize) {
        let (mut lo, mut hi) = (1, 0);
        q_buf.clear();
        for b_node in self.circuit.get(*TX_KEY).out.iter() {
            q_buf.push_back((*TX_KEY, *b_node, false));
        }

        while let Some((inp, out, pulse)) = q_buf.pop_front() {
            if pulse {
                hi += 1;
            } else {
                lo += 1;
            }
            if !self.circuit.contains_key(out) {
                continue;
            }
            self.state.set(inp, pulse);
            let out_gate = self.circuit.get(out);
            match out_gate.kind {
                GateKind::FlipFLop if !pulse => {
                    let state = self.state.get(out);
                    for downstream in &out_gate.out {
                        q_buf.push_back((out, *downstream, !state));
                    }
                    self.state.set(out, !state);
                }
                GateKind::Conj => {
                    let send = !self.rev.get(out).iter().all(|&x| *self.state.get(x));
                    if send && self.rx_inputs.contains_key(out) {
                        self.rx_cycles.push(t);
                    }
                    for downstream in &out_gate.out {
                        q_buf.push_back((out, *downstream, send));
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
        let out = rest
            .split(", ")
            .map(stackmap_key)
            .collect::<ArrayVec<_, 32>>();
        let kind = GateKind::from(name);
        let real_name = if matches!(kind, GateKind::Broadcast) {
            *TX_KEY
        } else {
            stackmap_key(name.split_at(1).1)
        };
        state.set(real_name, false);
        for output in out.iter() {
            rev.get_mut(*output).push(real_name);
        }
        circuit.set(real_name, Gate { kind, out });
    });

    let (mut rx_inputs, rx_cycles) = (StackMap::new(), ArrayVec::new());

    let mut expected_cycles = 0;
    rev.get(*rev.get(*RX_KEY).first().unwrap())
        .iter()
        .for_each(|&x| {
            expected_cycles += 1;
            rx_inputs.set(x, ());
        });

    let mut world = World {
        circuit,
        rev,
        state,
        rx_inputs,
        rx_cycles,
    };

    let mut q_buf = VecDeque::new();
    let (lo, hi) = (1..=1000)
        .map(|t| world.tick(&mut q_buf, t))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    let p1 = lo * hi;

    for t in 1001.. {
        if world.rx_cycles.len() == expected_cycles {
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
