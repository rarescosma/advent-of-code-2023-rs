use aoc_2023::ConstMap;
use aoc_cycles::multicycle;

use std::ptr;

trait Tilt {
    fn tilt_left(&mut self);
}

impl<const M: usize> Tilt for ConstMap<M> {
    fn tilt_left(&mut self) {
        let mut bins = [0; M];

        for row in self.inner.as_mut() {
            let mut i = 0;

            for (idx, &c) in row.iter().enumerate().skip(1) {
                if c == 'O' {
                    bins[i] = idx;
                    i += 1;
                }
            }
            for &pos in &bins[0..i] {
                let mut new_idx = pos;
                for inner in (0..pos).rev() {
                    if row[inner] != '.' {
                        break;
                    } else {
                        new_idx = inner;
                    }
                }
                // trust me
                if new_idx != pos {
                    unsafe {
                        ptr::swap(&mut row[pos], &mut row[new_idx]);
                    }
                }
            }
        }
    }
}

fn const_cycle<const M: usize>(m: &mut ConstMap<M>) {
    for _ in 0..4 {
        m.transpose();
        m.tilt_left();
        m.flip_vertical();
    }
}

fn const_load<const M: usize>(m: &ConstMap<M>) -> i32 {
    let mut ans = 0;
    for (r, row) in m.inner.into_iter().enumerate() {
        for el in row.into_iter() {
            if el == 'O' {
                ans += (M - r) as i32;
            }
        }
    }
    ans
}

fn solve() -> (i32, i32) {
    let c_map = include_str!("../../inputs/day14.txt")
        .replace('\n', "")
        .trim()
        .parse::<ConstMap<100>>()
        .expect("nope");

    let mut p1_map = c_map;
    p1_map.transpose();
    p1_map.tilt_left();
    p1_map.transpose();
    let p1 = const_load(&p1_map);

    let p2 = const_load(&multicycle(c_map, const_cycle, 1000000000));
    (p1, p2)
}

aoc_2023::main! {
    solve()
}
