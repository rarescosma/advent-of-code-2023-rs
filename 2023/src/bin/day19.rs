use aoc_prelude::{ArrayVec, HashMap};
use std::ops::RangeInclusive;

type Prop = usize;

// x,m,a,s
type Rating = [u32; 4];
type RatingRange = [RangeInclusive<u32>; 4];

const INIT_RANGE: RatingRange = [1..=4000, 1..=4000, 1..=4000, 1..=4000];
const START: &str = "in";

#[derive(Debug, PartialEq, Eq, Hash)]
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
struct RulePart<'a> {
    comp: Comp,
    dest_name: &'a str,
}

type RuleSet<'a> = HashMap<&'a str, ArrayVec<RulePart<'a>, 16>>;

fn extract_nums<const M: usize>(s: &str) -> [u32; M] {
    let mut res = [0; M];
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|w| !w.is_empty())
        .enumerate()
        .for_each(|(i, w)| res[i] = w.parse::<u32>().unwrap());
    res
}

fn is_valid_rating(rating: &Rating, valid_ranges: &[RatingRange]) -> bool {
    valid_ranges
        .iter()
        .any(|rs| rs.iter().enumerate().all(|(p, r)| r.contains(&rating[p])))
}

#[inline(always)]
fn is_destination<S: AsRef<str>>(s: S) -> bool {
    s.as_ref().ends_with('A')
}

fn get_ranges_2(rule_set: &RuleSet) -> ArrayVec<RatingRange, 1024> {
    let mut r_ranges = ArrayVec::new();
    let mut q = ArrayVec::<_, 64>::new();
    q.push((START, INIT_RANGE));

    while let Some((workflow, ranges)) = q.pop() {
        if workflow == "R" {
            continue;
        }
        if is_destination(workflow) {
            r_ranges.push(ranges);
            continue;
        }

        let mut outer_range = ranges.clone();
        for rule in &rule_set[workflow] {
            let mut true_range = outer_range.clone();
            rule.comp.apply(&mut true_range);

            if !true_range.is_empty() {
                q.push((&rule.dest_name, true_range));
            }

            rule.comp.rev_apply(&mut outer_range);
            if outer_range.is_empty() {
                break;
            }
        }
    }
    r_ranges
}

fn solve(input: &str) -> (u32, usize) {
    let mut split = input.split("\n\n");

    let mut rules = RuleSet::with_capacity(1024);

    rules.extend(split.next().unwrap().lines().map(|l| {
        let (name, rest) = l.split_once('{').unwrap();
        let rule = rest[0..rest.len() - 1]
            .split(',')
            .filter_map(|dest_name| {
                if !dest_name.contains(':') {
                    let comp = Comp::None;
                    return Some(RulePart { comp, dest_name });
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
                Some(RulePart { comp, dest_name })
            })
            .collect();
        (name, rule)
    }));

    let valid_ranges = get_ranges_2(&rules);

    let p1 = split
        .next()
        .unwrap()
        .lines()
        .map(extract_nums)
        .filter(|r| is_valid_rating(r, &valid_ranges))
        .flatten()
        .sum::<u32>();

    let p2 = valid_ranges
        .into_iter()
        .map(|r| r.into_iter().map(|p| p.count()).product::<usize>())
        .sum();

    (p1, p2)
}

aoc_2023::main! {
    solve(include_str!("../../inputs/19.in"))
}
