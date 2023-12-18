struct Quad {
    a: u64,
    rev_b: u64,
    c: u64,
}

impl Quad {
    fn as_floats(&self) -> (f64, f64, f64) {
        (self.a as f64, -(self.rev_b as f64), self.c as f64)
    }

    fn solve(&self) -> Option<(f64, f64)> {
        let (a, b, c) = self.as_floats();

        let d = b * b - 4f64 * a * c;
        if d >= 0f64 {
            let x1 = (-b + d.sqrt()) / 2f64;
            let x2 = (-b - d.sqrt()) / 2f64;
            return if x1 < x2 {
                Some((x1, x2))
            } else {
                Some((x2, x1))
            };
        }
        None
    }

    fn eval(&self, x: f64) -> f64 {
        let (a, b, c) = self.as_floats();
        a * x * x + b * x + c
    }
}

fn extract_numbers(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .filter_map(|w| w.parse::<u64>().ok())
        .collect()
}

fn count_solutions(tt: u64, dmin: u64) -> f64 {
    let quad = Quad {
        a: 1,
        rev_b: tt,
        c: dmin,
    };
    let (x1, x2) = quad.solve().expect("unsolvable quadratic");

    let x1 = x1.ceil();
    let x2 = x2.floor();

    let p1 = quad.eval(x1);
    let p2 = quad.eval(x2);

    if p1 < 0f64 && p2 < 0f64 {
        // eval(ceil(x1)) < 0 AND eval(floor(x2)) < 0
        // => count integers in between but also include ceil(x1) and floor(x2)
        x2 - x1 + 1f64
    } else if p1 == 0f64 && p2 == 0f64 {
        // both solutions are integers => count integers between
        x2 - x1 - 1f64
    } else {
        panic!("weird quadratic")
    }
}

fn concat(v: &[u64]) -> u64 {
    v.iter()
        .fold("".to_owned(), |acc, x| format!("{acc}{x}"))
        .parse::<u64>()
        .expect("invalid input")
}

fn solve() -> (u64, u64) {
    let input = include_str!("../../inputs/06.in")
        .to_string()
        .lines()
        .map(extract_numbers)
        .collect::<Vec<_>>();

    let p1 = input[0]
        .iter()
        .zip(input[1].iter())
        .map(|(tt, dmin)| count_solutions(*tt, *dmin))
        .product::<f64>();

    let p2 = count_solutions(concat(&input[0]), concat(&input[1]));

    (p1 as u64, p2 as u64)
}

aoc_2023::main! {
    solve()
}
