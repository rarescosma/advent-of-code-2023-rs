use aoc_prelude::{lazy_static, HashMap};

lazy_static! {
    static ref FORWARD_MAP: HashMap<String, String> = {
        let mut res: HashMap<String, String> = HashMap::new();
        for (idx, digit) in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        {
            let c = digit.chars().collect::<Vec<_>>();
            res.insert(
                digit.to_string(),
                format!("{}{}{}", c[0], idx + 1, c[c.len() - 1]),
            );
        }
        res
    };
}

#[inline]
fn process_line_p1<S: AsRef<str>>(s: S) -> u32 {
    let s = s.as_ref().chars();

    let first = s.clone().find(|x| x.is_ascii_digit()).unwrap_or('0');
    let last = s.rev().find(|x| x.is_ascii_digit()).unwrap_or('0');

    first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
}

fn replace_digits<S: AsRef<str>>(s: S) -> String {
    let mut s = s.as_ref().to_string();

    for f_pat in FORWARD_MAP.keys() {
        s = s.replace(f_pat, &FORWARD_MAP[f_pat]);
    }
    s
}

fn read_input() -> Vec<String> {
    include_str!("../../inputs/day01.txt")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>()
}

aoc_2023::main! {
    let input = read_input();

    let p1: u32 = input.iter().map(process_line_p1).sum();
    let p2: u32 = input.iter().map(replace_digits).map(process_line_p1).sum();

    (p1, p2)
}
