use aoc_prelude::*;
use std::ops::Add;

// dest src range_len
#[derive(Parser)]
#[grammar = "parsers/day05.pest"]
pub struct LookupParser;

#[derive(Debug)]
struct LMap {
    from: String,
    to: String,
    lookup: Lookup,
}

impl Add for LMap {
    type Output = Option<LMap>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.to != rhs.from {
            None
        } else {
            let res = LMap {
                from: self.from,
                to: rhs.to,
                // how do we merge lookups???
                lookup: Lookup::new([].into_iter()),
            };
            Some(res)
        }
    }
}

#[derive(Debug)]
struct Lookup {
    ranges: HashMap<u64, (u64, u64)>,
}

// inclusive
fn is_between(x: u64, s: u64, e: u64) -> bool {
    x >= s && x <= e
}

fn split_interval(i: (u64, u64), by: (u64, u64)) -> Vec<(u64, u64)> {
    let (by_s, by_e) = by;
    let (i_s, i_e) = i;

    match (is_between(by_s, i_s, i_e), is_between(by_e, i_s, i_e)) {
        (false, false) => {
            vec![i]
        }
        (true, false) => {
            let mut ret = vec![(by_s, i_e)];
            if by_s > 1 {
                ret.push((i_s, by_s - 1));
            }
            ret
        }
        (false, true) => {
            vec![(i_s, by_e), (by_e + 1, i_e)]
        }
        (true, true) => {
            let mut ret = vec![(by_s, by_e), (by_e + 1, i_e)];
            if by_s > 1 {
                ret.push((i_s, by_s - 1));
            }
            ret
        }
    }
}

fn merge_intervals(x: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut hm = HashMap::<u64, u64>::new();
    x.into_iter().for_each(|(a, b)| match hm.entry(a) {
        Entry::Occupied(mut v) => {
            let vm = v.get_mut();
            *vm = max(*vm, b);
        }
        Entry::Vacant(v) => {
            v.insert(b);
        }
    });
    hm.into_iter().collect()
}

impl Lookup {
    fn new(pairs: impl Iterator<Item = (u64, u64, u64)>) -> Self {
        let ranges = pairs
            .map(|(dest, from, to)| (dest, (from, to)))
            .collect::<HashMap<_, _>>();
        Self { ranges }
    }

    fn lookup(&self, x: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|(dest, &(start, end))| {
                if start <= x && x <= end {
                    Some(dest + (x - start))
                } else {
                    None
                }
            })
            .unwrap_or(x)
    }

    fn lookup_interval(&self, interval: (u64, u64)) -> impl Iterator<Item = (u64, u64)> + '_ {
        self.ranges
            .values()
            .flat_map(move |&range| split_interval(interval, range))
            .filter_map(move |(s, e)| {
                let ls = self.lookup(s);
                let le = self.lookup(e);
                if ls <= le {
                    Some((ls, le))
                } else {
                    None
                }
            })
    }
}

fn extract_number(pair: Option<Pair<Rule>>) -> u64 {
    pair.unwrap().as_str().parse::<u64>().expect("no number")
}

fn extract_lookup(pair: Pair<Rule>) -> LMap {
    let inner = pair.into_inner();
    let mut def = inner.peek().expect("no def").into_inner();
    let from = def.next().unwrap().as_str().to_owned();
    let to = def.next().unwrap().as_str().to_owned();

    let lookup = Lookup::new(inner.filter(|x| x.as_rule() == Rule::Lookup).map(|x| {
        let mut inner = x.into_inner();
        let dest = extract_number(inner.next());
        let src = extract_number(inner.next());
        let rng_len = extract_number(inner.next());
        (dest, src, src + rng_len - 1)
    }));

    LMap { from, to, lookup }
}

fn seed_to_location(seed: u64, chain: &HashMap<String, &LMap>) -> u64 {
    let mut ptr = "seed".to_owned();
    let mut look_for = seed;
    while ptr != "location" {
        let lmap = chain.get(&ptr).unwrap();
        look_for = lmap.lookup.lookup(look_for);
        ptr = lmap.to.clone();
    }
    look_for
}

fn seed_range_to_loc_range(range: (u64, u64), chain: &HashMap<String, &LMap>) -> Vec<(u64, u64)> {
    let mut ptr = "seed".to_owned();
    let mut look_for = vec![range];
    while ptr != "location" {
        let lmap = chain.get(&ptr).unwrap();

        look_for = merge_intervals(
            look_for
                .into_iter()
                .flat_map(|x| lmap.lookup.lookup_interval(x))
                .unique()
                .collect(),
        );

        ptr = lmap.to.clone();
    }
    look_for
}

aoc_2023::main! {
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

    let mut p2_candidates = seeds
        .chunks_exact(2)
        .flat_map(|x| seed_range_to_loc_range((x[0], x[1]), &chain))
        .map(|(a, _)| a)
        .collect::<HashSet<_>>();

    // wtf
    p2_candidates.remove(&0);
    let p2 = p2_candidates.iter().min().expect("no answer");

    (p1, *p2)
}
