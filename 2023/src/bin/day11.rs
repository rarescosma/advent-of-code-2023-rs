use aoc_2023::ConstMap;
use aoc_prelude::PrimInt;
use std::ops::AddAssign;

fn psa_elements<'a, const M: usize, P: PrimInt + 'a>(
    c_map: &'a ConstMap<M>,
    scale: P,
) -> impl Iterator<Item = P> + 'a {
    c_map.inner.iter().map(move |content| {
        if content.iter().all(|c| c == &'.') {
            scale
        } else {
            P::from(1).unwrap()
        }
    })
}

fn make_psa<const M: usize, P: PrimInt>(xs: impl Iterator<Item = P>) -> [P; M] {
    let mut res = [P::from(0).unwrap(); M];
    let mut l_sum = P::from(0).unwrap();
    for (i, x) in xs.enumerate() {
        let n_sum = l_sum + x;
        res[i] = n_sum;
        l_sum = n_sum;
    }
    res
}

fn total_distance<const M: usize, P: PrimInt + AddAssign>(c_map: &ConstMap<M>, scale: P) -> P {
    let psa = make_psa::<M, P>(psa_elements(c_map, scale));

    let mut total = P::from(0).unwrap();
    let mut num_seen = P::from(0).unwrap();
    let mut cumulative = P::from(0).unwrap();
    c_map
        .inner
        .iter()
        .map(|x| x.iter().filter(|&c| c == &'#').count())
        .enumerate()
        .for_each(|(index, count)| {
            let count_p = P::from(count).unwrap();
            total += (num_seen * psa[index] - cumulative) * count_p;
            cumulative += psa[index] * count_p;
            num_seen += count_p;
        });
    total
}

fn solve() -> (u64, u64) {
    let mut c_map = include_str!("../../inputs/11.in")
        .replace('\n', "")
        .trim()
        .parse::<ConstMap<140>>()
        .expect("nope");

    let p1_rows = total_distance(&c_map, 2);
    let p2_rows = total_distance(&c_map, 1000000);

    c_map.transpose();

    let p1 = p1_rows + total_distance(&c_map, 2);
    let p2 = p2_rows + total_distance(&c_map, 1000000);

    (p1, p2)
}

aoc_2023::main! {
    solve()
}
