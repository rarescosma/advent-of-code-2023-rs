use aoc_prelude::HashMap;

#[inline]
fn process_line_p1<S: AsRef<str>>(s: S) -> u32 {
    let s = s.as_ref().chars();

    let first = s.clone().find(|x| x.is_ascii_digit()).unwrap_or('0');
    let last = s.rev().find(|x| x.is_ascii_digit()).unwrap_or('0');

    first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
}

fn replace_digits<S: AsRef<str>>(s: S) -> String {
    // luckily this is all the overlap we need for English digits
    let forward: HashMap<&str, &str> = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);

    let mut s = s.as_ref().to_string();

    for f_pat in forward.keys() {
        s = s.replace(f_pat, forward[f_pat]);
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
