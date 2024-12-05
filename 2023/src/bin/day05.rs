use aoc_prelude::*;
use std::mem;

// dest src range_len
#[derive(Parser)]
#[grammar = "parsers/day05.pest"]
pub struct LookupParser;

#[derive(Debug)]
struct FnMap<'a> {
    from: &'a str,
    to: &'a str,
    fns: Vec<Fn>,
}

#[derive(Debug)]
struct Fn {
    dest: u64,
    src: u64,
    sz: u64,
}

type Ranges = Vec<(u64, u64)>;
struct Ctx {
    intervals: Ranges,
    buf: Ranges,
    ans: Ranges,
}

impl Ctx {
    fn new(intervals: Ranges) -> Self {
        Self {
            intervals,
            buf: Ranges::default(),
            ans: Ranges::default(),
        }
    }

    fn clear(&mut self) {
        self.buf.clear();
        self.ans.clear();
    }
}

impl Fn {
    fn apply(&self, x: u64) -> u64 {
        self.dest + x - self.src
    }
}

impl<'a> FnMap<'a> {
    fn lookup(&self, x: u64) -> u64 {
        self.fns
            .iter()
            .find_map(|f| {
                if f.src <= x && x < f.src + f.sz {
                    Some(f.apply(x))
                } else {
                    None
                }
            })
            .unwrap_or(x)
    }

    fn lookup_interval(&self, ctx: &mut Ctx) {
        ctx.clear();

        while let Some((st, ed)) = ctx.intervals.pop() {
            for f in &self.fns {
                let f_end = f.src + f.sz;
                // [st                                   ed)
                //             [f.src    f_end)
                // [BEFORE    )[CUT           )[AFTER      )
                // -----------------------------------------
                // [st    c_st)[c_st      c_ed)[c_ed     ed)
                let (c_st, c_ed) = (max(st, f.src), min(ed, f_end));
                if c_ed > c_st {
                    ctx.ans.push((f.apply(c_st), f.apply(c_ed)));
                    if st < c_st {
                        ctx.buf.push((st, c_st));
                    }
                    if c_ed < ed {
                        ctx.buf.push((c_ed, ed));
                    }
                    break;
                }
                ctx.ans.push((st, ed));
            }
            mem::swap(&mut ctx.intervals, &mut ctx.buf);
        }
        mem::swap(&mut ctx.intervals, &mut ctx.ans);
    }
}

fn extract_number(pair: Option<Pair<Rule>>) -> u64 {
    pair.unwrap().as_str().parse::<u64>().expect("no number")
}

fn extract_lookup(pair: Pair<Rule>) -> FnMap {
    let inner = pair.into_inner();
    let mut def = inner.peek().expect("no def").into_inner();
    let from = def.next().unwrap().as_str();
    let to = def.next().unwrap().as_str();

    let lookup = inner
        .filter(|x| x.as_rule() == Rule::Lookup)
        .map(|x| {
            let mut inner = x.into_inner();
            Fn {
                dest: extract_number(inner.next()),
                src: extract_number(inner.next()),
                sz: extract_number(inner.next()),
            }
        })
        .collect::<Vec<_>>();

    FnMap {
        from,
        to,
        fns: lookup,
    }
}

fn seed_to_location(seed: u64, chain: &HashMap<&str, &FnMap>) -> u64 {
    let mut ptr = "seed";
    let mut look_for = seed;
    while ptr != "location" {
        let lmap = chain.get(&ptr).unwrap();
        look_for = lmap.lookup(look_for);
        ptr = lmap.to;
    }
    look_for
}

fn seed_range_to_loc_range(range: (u64, u64), chain: &HashMap<&str, &FnMap>) -> Vec<(u64, u64)> {
    let mut ptr = "seed";
    let mut ctx = Ctx::new(vec![range]);
    while ptr != "location" {
        let lmap = chain.get(&ptr).unwrap();
        lmap.lookup_interval(&mut ctx);
        ptr = lmap.to;
    }
    ctx.intervals
}

fn solve() -> (u64, u64) {
    let parsed = LookupParser::parse(Rule::root, include_str!("../../inputs/05.in"))
        .expect("failed parse")
        .next()
        .unwrap()
        .into_inner();

    let seeds = parsed
        .peek()
        .expect("no seeds")
        .into_inner()
        .map(|x| x.as_str().parse::<u64>().expect("not a number"))
        .collect::<Vec<_>>();

    let lookups = parsed
        .filter(|x| x.as_rule() == Rule::lookup_table)
        .map(extract_lookup)
        .collect::<Vec<_>>();

    let mut chain = HashMap::new();

    for lookup in &lookups {
        chain.insert(lookup.from, lookup);
    }

    let p1 = seeds
        .clone()
        .into_iter()
        .map(|s| seed_to_location(s, &chain))
        .min()
        .expect("at least one seed");

    let p2 = seeds
        .chunks_exact(2)
        .flat_map(|x| seed_range_to_loc_range((x[0], x[0] + x[1]), &chain))
        .map(|(a, _)| a)
        .min()
        .expect("no answer");

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
