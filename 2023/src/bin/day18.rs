use std::ops::Index;

type Pt = (i64, i64);

struct Vertices<const M: usize> {
    pts: [Pt; M],
    len: usize,
}

impl<const M: usize> Vertices<M> {
    fn new() -> Self {
        Self {
            pts: [(0, 0); M],
            len: 0,
        }
    }
    fn clear(&mut self) {
        self.len = 0;
    }

    fn push(&mut self, p: Pt) {
        self.pts[self.len] = p;
        self.len += 1;
    }
}

impl<const M: usize> Index<usize> for Vertices<M> {
    type Output = Pt;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pts[index]
    }
}

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
    let mut words = s.split_whitespace();
    let (dist, dir) = words
        .nth(2)
        .map(|w| {
            let dir_idx = w.len() - 2;

            let dist = i64::from_str_radix(&w[2..=6], 16).expect("bad hex");
            let dir = match &w[dir_idx..=dir_idx] {
                "0" => (1, 0),
                "1" => (0, 1),
                "2" => (-1, 0),
                "3" => (0, -1),
                _ => unimplemented!(),
            };
            (dist, dir)
        })
        .expect("failed parse");

    (dir, dist)
}

// Shoelace theorem, there is no escaping
fn shoelace<const M: usize>(vertices: &Vertices<M>, diameter_cells: i64) -> i64 {
    let a0 = vertices[0].1 * (vertices[vertices.len - 1].0 - vertices[1].0);
    let area = a0
        + (1..vertices.len - 1)
            .map(|i| vertices[i].1 * (vertices[i - 1].0 - vertices[i + 1].0))
            .sum::<i64>();

    (diameter_cells + area) / 2 + 1
}

fn solve_part<const M: usize, F: Fn(&str) -> (Pt, i64)>(
    input: &str,
    extract_f: F,
    vx: &mut Vertices<M>,
) -> i64 {
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

    let mut vx = Vertices::<1024>::new();

    let p1 = solve_part(input, p1_extract, &mut vx);
    let p2 = solve_part(input, p2_extract, &mut vx);

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
