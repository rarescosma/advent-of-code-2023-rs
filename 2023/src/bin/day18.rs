type Pt = (i64, i64);

fn p1_extract(s: &str) -> (Pt, i64) {
    let mut words = s.split_whitespace();

    (
        match words.next().unwrap() {
            "R" => (1, 0),
            "D" => (0, 1),
            "L" => (-1, 0),
            "U" => (0, -1),
            _ => unimplemented!(),
        },
        words.next().unwrap().parse::<i64>().unwrap(),
    )
}

fn p2_extract(s: &str) -> (Pt, i64) {
    s.split_whitespace()
        .nth(2)
        .map(|w| {
            let dist = i64::from_str_radix(&w[2..=6], 16).expect("bad hex");
            let dir = match w.chars().nth(7).expect("bad hex") {
                '0' => (1, 0),
                '1' => (0, 1),
                '2' => (-1, 0),
                '3' => (0, -1),
                _ => unimplemented!(),
            };
            (dir, dist)
        })
        .expect("failed parse")
}

// Shoelace theorem, there is no escaping
fn shoelace(vertices: &Vec<Pt>, diameter_cells: i64) -> i64 {
    let a0 = vertices[0].1 * (vertices[vertices.len() - 1].0 - vertices[1].0);
    let area = a0
        + (1..vertices.len() - 1)
            .map(|i| vertices[i].1 * (vertices[i - 1].0 - vertices[i + 1].0))
            .sum::<i64>();

    (diameter_cells + area) / 2 + 1
}

fn solve_part<F: Fn(&str) -> (Pt, i64)>(input: &str, extract_f: F, vx: &mut Vec<Pt>) -> i64 {
    let mut cur: Pt = (0, 0);
    vx.clear();
    vx.push(cur);
    let mut diameter = 0;

    for line in input.lines() {
        let (dir, dist) = extract_f(line);
        let end = (cur.0 + dir.0 * dist, cur.1 + dir.1 * dist);
        vx.push(end);
        diameter += dist;
        cur = end;
    }

    // complete the diameter by going back to origin
    diameter += cur.0.abs() + cur.1.abs();

    shoelace(vx, diameter)
}

fn solve() -> Pt {
    let input = include_str!("../../inputs/day18.txt");

    let mut vx = Vec::with_capacity(1024);

    let p1 = solve_part(input, p1_extract, &mut vx);
    let p2 = solve_part(input, p2_extract, &mut vx);

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
