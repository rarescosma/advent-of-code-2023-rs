use aoc_prelude::{HashSet, Itertools};
use std::collections::{HashMap, VecDeque};
use std::ops::RangeInclusive;

type Prop = usize;

// x,m,a,s
type Rating = [u32; 4];
type RatingRange = [RangeInclusive<u32>; 4];

const INIT_RANGE: RatingRange = [1..=4000, 1..=4000, 1..=4000, 1..=4000];
const START: &str = "in";

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Comp {
    None,
    Less(Prop, u32),
    Great(Prop, u32),
}

impl Comp {
    fn apply(&self, r: &mut RatingRange) {
        match *self {
            Comp::None => {}
            Comp::Less(prop, val) => r[prop] = *r[prop].start()..=val - 1,
            Comp::Great(prop, val) => r[prop] = val + 1..=*r[prop].end(),
        }
    }
    fn rev_apply(&self, r: &mut RatingRange) {
        match *self {
            Comp::None => {}
            Comp::Less(prop, val) => r[prop] = val..=*r[prop].end(),
            Comp::Great(prop, val) => r[prop] = *r[prop].start()..=val,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RulePart {
    comp: Comp,
    dest_name: String,
}

type RuleSet<'a> = HashMap<&'a str, Vec<RulePart>>;

fn extract_nums(s: &str) -> Vec<u32> {
    s.split(',')
        .filter_map(|w| w.chars().filter(|c| c.is_numeric()).join("").parse().ok())
        .collect()
}

fn is_valid_rating(rating: &Rating, valid_ranges: &[RatingRange]) -> bool {
    valid_ranges
        .iter()
        .any(|r| r.iter().enumerate().all(|(p, r)| r.contains(&rating[p])))
}

fn chain_to_range(chain: &[&str], rule_set: &RuleSet) -> RatingRange {
    // traverse the chain and check the final range
    let mut range = INIT_RANGE;

    'outer: for pair in chain.windows(2) {
        let (cur, next) = (&pair[0], &pair[1]);

        'inner: for rule in rule_set[cur].iter() {
            if &rule.dest_name == next {
                rule.comp.apply(&mut range);
                if rule.dest_name.ends_with("_A") {
                    break 'outer;
                } else {
                    break 'inner;
                }
            } else {
                rule.comp.rev_apply(&mut range);
            }
        }
    }
    range
}

fn get_ranges(rule_set: &RuleSet) -> Vec<RatingRange> {
    let mut ranges = Vec::new();
    let mut cur_chain = Vec::<&str>::new();

    let mut fully_explored = HashSet::new();

    let mut q = VecDeque::new();
    q.push_back(START);

    while let Some(cur) = q.pop_back() {
        if fully_explored.contains(cur) {
            if cur == START {
                break;
            }
            continue;
        }

        if cur == START {
            if !cur_chain.is_empty() && cur_chain.last().is_some_and(|x| x.ends_with("_A")) {
                ranges.push(chain_to_range(&cur_chain, rule_set));
            }
            cur_chain.clear();
        }

        cur_chain.push(cur);

        let mut mark = false;
        if !cur.ends_with("_A") {
            let rules = &rule_set[cur];
            if let Some(next) = rules
                .iter()
                .map(|r| &r.dest_name)
                .find(|&d| d != "R" && !fully_explored.contains(d.as_str()))
            {
                q.push_back(next);
            } else {
                mark = true;
            }
        } else {
            mark = true;
        }

        if mark {
            fully_explored.insert(cur);
            q.clear();
            q.push_back(START);
        }
    }

    ranges
}

fn solve(input: &str) -> (u32, usize) {
    let mut split = input.split("\n\n");
    let mut dest_idx = 0;

    let rules = split
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let (name, rest) = l.split_once('{').unwrap();
            let rule = rest[0..rest.len() - 1]
                .split(',')
                .filter_map(|dest_name| {
                    if !dest_name.contains(':') {
                        return Some((Comp::None, dest_name));
                    }
                    let (rest, dest_name) = dest_name.split_once(':')?;
                    let comp = if rest.contains('>') { '>' } else { '<' };
                    let (name, val) = rest.split_once(comp)?;
                    let name = name.chars().next()?;
                    let prop = "xmas".chars().position(|y| y == name)?;
                    let val = val.parse::<u32>().ok()?;

                    let comp = match comp {
                        '<' => Comp::Less(prop, val),
                        '>' => Comp::Great(prop, val),
                        _ => unimplemented!(),
                    };
                    Some((comp, dest_name))
                })
                .map(|(comp, d)| {
                    let dest_name = if d == "A" {
                        dest_idx += 1;
                        format!("{name}_{dest_idx}_{d}")
                    } else {
                        d.to_owned()
                    };
                    RulePart { comp, dest_name }
                })
                .collect::<Vec<_>>();
            (name, rule)
        })
        .collect::<RuleSet>();

    let valid_ranges = get_ranges(&rules);

    let p1 = split
        .next()
        .unwrap()
        .lines()
        .filter_map(|l| Rating::try_from(extract_nums(l)).ok())
        .filter(|r| is_valid_rating(r, &valid_ranges))
        .flatten()
        .sum::<u32>();

    let p2 = valid_ranges
        .into_iter()
        .map(|r| r.into_iter().map(|p| p.count()).product::<usize>())
        .sum::<usize>();

    (p1, p2)
}

aoc_2023::main! {
    solve(include_str!("../../inputs/19.in"))
}
