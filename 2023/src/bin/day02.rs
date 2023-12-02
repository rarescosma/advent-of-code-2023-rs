use aoc_prelude::*;

#[derive(Parser)]
#[grammar = "parsers/day02.pest"]
pub struct GameParser;

#[derive(Clone, Debug, Default)]
struct Game {
    number: u16,
    draws: Vec<Draw>,
}

#[derive(Clone, Debug, Default)]
struct Draw {
    red: u8,
    green: u8,
    blue: u8,
}

impl Draw {
    fn is_possible(&self, max_draw: &Draw) -> bool {
        self.red <= max_draw.red && self.green <= max_draw.green && self.blue <= max_draw.blue
    }

    fn power(&self) -> usize {
        (self.red as usize) * (self.green as usize) * (self.blue as usize)
    }
}

fn process_line(line: Pair<Rule>) -> Game {
    let mut game = Game::default();

    line.into_inner().for_each(|r| match r.as_rule() {
        Rule::GameNumber => {
            game.number = r.as_str().parse().expect("invalid game number");
        }
        Rule::Draw => {
            let mut draw = Draw::default();
            let mut quant = 0;
            r.into_inner().for_each(|x| match x.as_rule() {
                Rule::number => quant = x.as_str().parse().expect("not a number"),
                Rule::color => match x.as_str() {
                    "red" => {
                        draw.red = quant;
                    }
                    "green" => {
                        draw.green = quant;
                    }
                    "blue" => {
                        draw.blue = quant;
                    }
                    _ => {}
                },
                _ => {}
            });
            game.draws.push(draw);
        }
        _ => {}
    });

    game
}

aoc_2023::main! {
    let input = include_str!("../../inputs/day02.txt").to_string();

    let games: Vec<_> = GameParser::parse(Rule::lines, &input)
        .expect("failed parse")
        .next()
        .unwrap()
        .into_inner()
        .filter(|x| x.as_rule() == Rule::line)
        .map(process_line)
        .collect();

    let max_draw = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    let p1: u16 = games
        .iter()
        .filter(|x| x.draws.iter().all(|d| d.is_possible(&max_draw)))
        .map(|g| g.number)
        .sum();

    let p2: usize = games
        .iter()
        .map(|g| {
            let mut limits = Draw::default();
            g.draws.iter().for_each(|d| {
                limits.red = max(limits.red, d.red);
                limits.green = max(limits.green, d.green);
                limits.blue = max(limits.blue, d.blue);
            });
            limits.power()
        })
        .sum();

    (p1, p2)
}
