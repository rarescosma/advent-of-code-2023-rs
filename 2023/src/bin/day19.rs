use aoc_prelude::{HashSet, Itertools};
use std::collections::{HashMap, VecDeque};
use std::ops::RangeInclusive;

type Prop = usize;
// x,m,a,s
type Rating = [u32; 4];
type RatingRange = [RangeInclusive<u32>; 4];

const INIT_RANGE: RatingRange = [1..=4000, 1..=4000, 1..=4000, 1..=4000];

#[derive(Debug, PartialEq, Eq, Hash)]
enum Comp {
    None,
    Less(Prop, u32),
    Great(Prop, u32),
}

impl Comp {
    fn apply(&self, r: &mut RatingRange) {
        match self {
            Comp::None => {}
            Comp::Less(prop, val) => {
                r[*prop] = *r[*prop].start()..=val - 1;
            }
            Comp::Great(prop, val) => r[*prop] = val + 1..=*r[*prop].end(),
        }
    }

    fn rev_apply(&self, r: &mut RatingRange) {
        match self {
            Comp::None => {}
            Comp::Less(prop, val) => r[*prop] = *val..=*r[*prop].end(),
            Comp::Great(prop, val) => {
                r[*prop] = *r[*prop].start()..=*val;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RulePart {
    comp: Comp,
    dest_name: String,
}

type RuleSet = HashMap<String, Vec<RulePart>>;

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

fn chain_to_range(chain: &[String], rule_set: &RuleSet) -> RatingRange {
    // traverse the chain and check the final range
    let mut range = INIT_RANGE;

    'outer: for pair in chain.windows(2) {
        let (cur, next) = (&pair[0], &pair[1]);

        'inner: for (idx, rule) in rule_set[cur].iter().enumerate() {
            if rule.dest_name == "A" && next == &format!("{cur}_{idx}_A") {
                rule.comp.apply(&mut range);
                break 'outer;
            }
            if &rule.dest_name == next {
                rule.comp.apply(&mut range);
                break 'inner;
            } else {
                rule.comp.rev_apply(&mut range);
            }
        }
    }
    range
}

fn get_ranges(rule_set: &RuleSet) -> Vec<RatingRange> {
    let mut ranges = Vec::new();
    let mut cur_chain = Vec::<String>::new();

    let mut fully_explored = HashSet::new();

    let mut q = VecDeque::new();
    q.push_back("in".to_owned());

    while let Some(ref cur) = q.pop_back() {
        if fully_explored.contains(cur) {
            if cur == "in" {
                break;
            }
            continue;
        }

        if cur == "in" {
            if !cur_chain.is_empty() && cur_chain.last().is_some_and(|x| x.ends_with("_A")) {
                ranges.push(chain_to_range(&cur_chain, rule_set));
            }
            cur_chain.clear();
        }

        cur_chain.push(cur.to_owned());

        let mut mark = false;
        if !cur.ends_with("_A") {
            let rules = &rule_set[cur];
            if let Some(next) = rules
                .iter()
                .enumerate()
                .filter_map(|(idx, rule)| {
                    if rule.dest_name == "R" {
                        None
                    } else if rule.dest_name == "A" {
                        Some(format!("{cur}_{idx}_A"))
                    } else {
                        Some(rule.dest_name.to_owned())
                    }
                })
                .find(|d| !fully_explored.contains(d))
            {
                q.push_back(next);
            } else {
                mark = true;
            }
        } else {
            mark = true;
        }

        if mark {
            fully_explored.insert(cur.to_owned());
            q.clear();
            q.push_back("in".to_owned());
        }
    }

    ranges
}

fn solve(input: &str) -> (u32, usize) {
    let mut split = input.split("\n\n");
    let rules = split
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut s = l.split('{');
            let name = s.next().unwrap().to_owned();
            let rules = s.next().unwrap();
            let rule_parts = rules[0..rules.len() - 1]
                .split(',')
                .map(|r| {
                    if !r.contains(':') {
                        return RulePart {
                            comp: Comp::None,
                            dest_name: r.to_owned(),
                        };
                    }

                    let mut s2 = r.split(':');
                    let rule_spec = s2.next().unwrap();
                    let dest_name = s2.next().unwrap().to_owned();

                    let mut s3 = rule_spec.chars();
                    let val = s3
                        .next()
                        .and_then(|x| "xmas".chars().position(|y| y == x))
                        .unwrap();

                    let comp = s3.next().unwrap();
                    let cmp_with = s3.join("").parse::<u32>().unwrap();
                    let comp = match comp {
                        '<' => Comp::Less(val, cmp_with),
                        '>' => Comp::Great(val, cmp_with),
                        _ => unimplemented!(),
                    };
                    RulePart { comp, dest_name }
                })
                .collect::<Vec<_>>();
            (name, rule_parts)
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
