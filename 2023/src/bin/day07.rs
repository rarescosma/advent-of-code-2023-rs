use aoc_prelude::{HashMap, Itertools};

// tuple within tuple => can compare
type Score = (u32, Option<(char, char, char, char, char)>);

#[derive(Default, Debug, Clone)]
struct Hand {
    bid: u32,
    p1_score: Score,
    p2_score: Score,
}

fn card_cmp(card: &str, p2: bool, counter: &mut HashMap<char, u32>) -> Score {
    counter.clear();
    let mut most_numerous = 'J';
    let mut max_tally = 0;

    for c in card.chars() {
        counter.entry(c).and_modify(|x| *x += 1).or_insert(1);
        if c != 'J' && counter[&c] > max_tally {
            max_tally = counter[&c];
            most_numerous = c;
        }
    }

    // replace the most numerous card with jokers for part 2
    if p2 && most_numerous != 'J' && counter.contains_key(&'J') {
        let num_jokers = counter[&'J'];
        counter
            .entry(most_numerous)
            .and_modify(|x| *x += num_jokers);
        counter.remove(&'J');
    }

    let rank = match counter.values().sorted().as_slice() {
        [5] => 100,
        [1, 4] => 90,
        [2, 3] => 80,
        [1, 1, 3] => 70,
        [1, 2, 2] => 60,
        [1, 1, 1, 2] => 50,
        [1, 1, 1, 1, 1] => 40,
        _ => unimplemented!(),
    };

    // downgrade joker value to '< 2' for part 2
    let j_replace = if p2 { '1' } else { 'w' };

    (
        rank,
        card.chars()
            .map(|c| match c {
                'A' => 'z',
                'K' => 'y',
                'Q' => 'x',
                'J' => j_replace,
                'T' => 'v',
                other => other,
            })
            .collect_tuple::<(_, _, _, _, _)>(),
    )
}

fn total_score(hands: &[Hand]) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * (h.bid as usize))
        .sum::<usize>()
}

fn solve() -> (usize, usize) {
    // re-use counter allocation
    let mut counter = HashMap::new();

    let input = include_str!("../../inputs/day07.txt");
    let mut hands = input
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            let cards = l.next().expect("invalid line");
            let bid = l
                .next()
                .expect("invalid line")
                .parse()
                .expect("invalid number");
            Hand {
                bid,
                p1_score: card_cmp(cards, false, &mut counter),
                p2_score: card_cmp(cards, true, &mut counter),
            }
        })
        .collect::<Vec<_>>();

    hands.sort_by(|lh, rh| lh.p1_score.cmp(&rh.p1_score));
    let p1 = total_score(&hands);

    hands.sort_by(|lh, rh| lh.p2_score.cmp(&rh.p2_score));
    let p2 = total_score(&hands);

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
