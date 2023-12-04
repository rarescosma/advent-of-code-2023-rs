use aoc_prelude::*;

#[derive(Parser)]
#[grammar = "parsers/day04.pest"]
pub struct CardParser;

#[derive(Default, Debug, Clone)]
struct Card {
    draws: HashSet<u16>,
    haves: HashSet<u16>,
}

fn process_line(line: Pair<Rule>) -> Card {
    let mut game = Card::default();

    line.into_inner().for_each(|r| match r.as_rule() {
        Rule::Draw => {
            game.draws
                .insert(r.as_str().parse().expect("invalid number"));
        }
        Rule::Have => {
            game.haves
                .insert(r.as_str().parse().expect("invalid number"));
        }
        _ => {}
    });

    game
}

aoc_2023::main! {
    let input = include_str!("../../inputs/day04.txt").to_string();

    let cards = CardParser::parse(Rule::lines, &input)
        .expect("failed parse")
        .next()
        .unwrap()
        .into_inner()
        .filter(|x| x.as_rule() == Rule::line)
        .map(process_line)
        .collect::<Vec<_>>();

    let mut tally = (0..cards.len())
        .map(|x| (x, 1))
        .collect::<HashMap<usize, u32>>();

    let p1 = cards
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, c)| {
            let num = (c.haves.intersection(&c.draws)).collect::<Vec<_>>().len();

            let offset = tally[&idx];

            for j in idx + 1..=idx + num {
                if let Some(r) = tally.get_mut(&j) {
                    *r += offset;
                }
            }

            if num > 0 {
                2.pow((num - 1) as u32)
            } else {
                0
            }
        })
        .sum::<u32>();

    let p2 = tally.values().sum::<u32>();

    (p1, p2)
}
