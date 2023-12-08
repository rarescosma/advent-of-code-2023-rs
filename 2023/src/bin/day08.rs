use aoc_prelude::*;
use num_bigint::BigInt;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "parsers/day08.pest"]
pub struct NodeParser;

#[derive(Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct Node(String);

impl From<&mut Pairs<'_, Rule>> for Node {
    fn from(value: &mut Pairs<Rule>) -> Self {
        Self(value.next().unwrap().as_str().to_owned())
    }
}

fn steps_until<P: Fn(&Node) -> bool>(
    graph: &HashMap<Node, (Node, Node)>,
    instr: &mut impl Iterator<Item = char>,
    start: &Node,
    accept: P,
) -> BigInt {
    let mut ans = BigInt::from(0);
    let mut cur = start;
    for i in instr {
        if accept(cur) {
            break;
        } else {
            ans += 1;
        }
        match i {
            'L' => cur = &graph[cur].0,
            'R' => cur = &graph[cur].1,
            _ => unimplemented!(),
        }
    }
    ans
}

fn solve() -> (BigInt, BigInt) {
    let mut input = include_str!("../../inputs/day08.txt").lines();
    let instr = input.next().unwrap().chars().collect::<Vec<_>>();

    let mut graph = HashMap::new();
    for line in input {
        if let Ok(ref mut parsed) = NodeParser::parse(Rule::line, line) {
            let root: Node = parsed.into();
            let left: Node = parsed.into();
            let right: Node = parsed.into();

            graph.insert(root, (left, right));
        }
    }

    let target = &Node("ZZZ".to_owned());
    let p1 = steps_until(
        &graph,
        &mut instr.clone().into_iter().cycle(),
        &Node("AAA".to_owned()),
        |n| n == target,
    );

    let p2 = graph
        .keys()
        .filter(|x| x.0.ends_with('A'))
        .sorted()
        .map(|start_node| {
            steps_until(
                &graph,
                &mut instr.clone().into_iter().cycle(),
                start_node,
                |n| n.0.ends_with('Z'),
            )
        })
        .fold(BigInt::from(1), num_integer::lcm);

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
