use std::mem;

fn extract_nums(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect()
}

fn outer(xs: &[i32], buf: &mut Vec<i32>, inner_buf: &mut Vec<i32>) -> (i32, i32) {
    *buf = xs.to_vec();

    let mut post = 0;
    let mut pre = 0;
    let mut signum = 1;

    assert!(buf.len() >= 2);

    loop {
        let last = buf[buf.len() - 1];
        post += last;

        pre += signum * buf[0];
        signum = -signum;

        if last == 0 && buf[buf.len() - 2] == 0 {
            break;
        }
        inner(buf, inner_buf);
        mem::swap(buf, inner_buf);
    }

    (post, pre)
}

fn inner(xs: &[i32], buf: &mut Vec<i32>) {
    *buf = xs
        .iter()
        .skip(1)
        .zip(xs.iter())
        .map(|(t1, t2)| *t1 - *t2)
        .collect();
}

fn solve() -> (i32, i32) {
    let mut outer_buf = Vec::new();
    let mut inner_buf = Vec::new();

    let extrapolated = include_str!("../../inputs/day09.txt")
        .lines()
        .map(extract_nums)
        .filter(|nums| !nums.is_empty())
        .map(|xs| outer(&xs, &mut outer_buf, &mut inner_buf))
        .collect::<Vec<_>>();

    let p1 = extrapolated.iter().map(|x| x.0).sum::<i32>();
    let p2 = extrapolated.iter().map(|x| x.1).sum::<i32>();

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
