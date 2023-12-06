use aoc_prelude::*;

// dest src range_len
#[derive(Parser)]
#[grammar = "parsers/day05.pest"]
pub struct LookupParser;

#[derive(Debug)]
struct FnMap {
    from: String,
    to: String,
    fns: Vec<Fn>,
}

#[derive(Debug)]
struct Fn {
    dest: u64,
    src: u64,
    sz: u64,
}

impl Fn {
    fn apply(&self, x: u64) -> u64 {
        self.dest + x - self.src
    }
}

impl FnMap {
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

    fn lookup_interval(
        &self,
        mut intervals: Vec<(u64, u64)>,
    ) -> impl Iterator<Item = (u64, u64)> + '_ {
        let mut ans = Vec::new();

        for f in &self.fns {
            let mut non_intersecting = Vec::new();
            let f_end = f.src + f.sz;
            while let Some((st, ed)) = intervals.pop() {
                // [st                                   ed)
                //             [f.src    f_end)
                // [BEFORE    )[INTERSECT     )[AFTER      )
                let before = (st, min(ed, f.src));
                if before.1 > before.0 {
                    non_intersecting.push(before);
                }

                let after = (max(f_end, st), ed);
                if after.1 > after.0 {
                    non_intersecting.push(after);
                }

                let intersect = (max(st, f.src), min(ed, f_end));
                if intersect.1 > intersect.0 {
                    ans.push((f.apply(intersect.0), f.apply(intersect.1)))
                }
            }
            intervals = non_intersecting;
        }
        ans.into_iter().chain(intervals)
    }
}

fn extract_number(pair: Option<Pair<Rule>>) -> u64 {
    pair.unwrap().as_str().parse::<u64>().expect("no number")
}

fn extract_lookup(pair: Pair<Rule>) -> FnMap {
    let inner = pair.into_inner();
    let mut def = inner.peek().expect("no def").into_inner();
    let from = def.next().unwrap().as_str().to_owned();
    let to = def.next().unwrap().as_str().to_owned();

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

fn seed_to_location(seed: u64, chain: &HashMap<String, &FnMap>) -> u64 {
    let mut ptr = "seed".to_owned();
    let mut look_for = seed;
    while ptr != "location" {
        let lmap = chain.get(&ptr).unwrap();
        look_for = lmap.lookup(look_for);
        ptr = lmap.to.clone();
    }
    look_for
}

fn seed_range_to_loc_range(range: (u64, u64), chain: &HashMap<String, &FnMap>) -> Vec<(u64, u64)> {
    let mut ptr = "seed".to_owned();
    let mut look_for = vec![range];
    while ptr != "location" {
        let lmap = chain.get(&ptr).unwrap();
        look_for = lmap.lookup_interval(look_for).collect();
        ptr = lmap.to.clone();
    }
    look_for
}

fn solve() -> (u64, u64) {
    let parsed = LookupParser::parse(Rule::root, include_str!("../../inputs/day05.txt"))
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

    for lookup in lookups.iter() {
        chain.insert(lookup.from.clone(), lookup);
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
