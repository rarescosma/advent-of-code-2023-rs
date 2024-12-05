use aoc_prelude::*;
use rayon::prelude::*;

#[derive(Parser)]
#[grammar = "parsers/day08.pest"]
pub struct NodeParser;

type Node<'a> = &'a str;

type Graph<'a> = HashMap<Node<'a>, (Node<'a>, Node<'a>)>;

fn extract_node<'a>(pairs: &mut Pairs<'a, Rule>) -> Node<'a> {
    pairs.next().unwrap().as_str()
}

fn steps_until<P: Fn(Node) -> bool>(
    graph: &Graph,
    instr: &mut impl Iterator<Item = char>,
    start: Node,
    accept: P,
) -> BigInt {
    let mut ans = BigInt::from(0);
    let mut cur = start;
    for i in instr {
        if accept(cur) {
            break;
        }
        ans += 1;
        match i {
            'L' => cur = graph[cur].0,
            'R' => cur = graph[cur].1,
            _ => unimplemented!(),
        }
    }
    ans
}

fn solve() -> (BigInt, BigInt) {
    let mut input = include_str!("../../inputs/08.in").lines();
    let instructions = input.next().expect("no lines").chars().cycle();

    let graph = input
        .filter_map(|line| NodeParser::parse(Rule::line, line).ok())
        .map(|ref mut pairs| {
            (
                extract_node(pairs),
                (extract_node(pairs), extract_node(pairs)),
            )
        })
        .collect::<Graph>();

    let p1 = steps_until(&graph, &mut instructions.clone(), "AAA", |cur| cur == "ZZZ");

    let p2 = graph
        .keys()
        .filter(|x| x.ends_with('A'))
        .par_bridge()
        .map(|start| {
            steps_until(&graph, &mut instructions.clone(), start, |cur| {
                cur.ends_with('Z')
            })
        })
        // there's only one matching target node for each starting node, so lcm is alright!
        .reduce(|| BigInt::from(1), num_integer::lcm);

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
