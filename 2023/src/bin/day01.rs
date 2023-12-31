const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn read_input() -> Vec<&'static str> {
    include_str!("../../inputs/01.in")
        .lines()
        .collect::<Vec<_>>()
}

fn solve() -> (u32, u32) {
    let input = read_input();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut p1_digits = Vec::new();
    let mut p2_digits = Vec::new();

    for line in input {
        p1_digits.clear();
        p2_digits.clear();

        for (idx, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let dig = c.to_digit(10).unwrap();
                p1_digits.push(dig);
                p2_digits.push(dig);
            }
            for (d_idx, v) in DIGITS.iter().enumerate() {
                if line[idx..].starts_with(v) {
                    p2_digits.push((d_idx + 1) as u32);
                }
            }
        }
        p1 += p1_digits[0] * 10 + p1_digits[p1_digits.len() - 1];
        p2 += p2_digits[0] * 10 + p2_digits[p2_digits.len() - 1];
    }

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
