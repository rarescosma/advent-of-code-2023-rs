use aoc_prelude::Itertools;
use std::fmt::{Display, Formatter};
use std::ptr;
use std::str::FromStr;
/// Macro for solution timing
/// Credits: https://github.com/AxlLind/
#[macro_export]
macro_rules! main {
  ($($body:tt)+) => {
    fn main() {
      let now = std::time::Instant::now();
      let (p1,p2) = { $($body)+ };
      let elapsed = now.elapsed();
      println!("Part one: {}", p1);
      println!("Part two: {}", p2);
      if elapsed.as_millis() > 0 {
        println!("Time: {}ms", elapsed.as_millis());
      } else {
        println!("Time: {}μs", elapsed.as_micros());
      }
    }
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ConstMap<const M: usize> {
    pub inner: [[char; M]; M],
}

impl<const M: usize> Display for ConstMap<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .inner
                .iter()
                .map(|x| x.iter().collect::<String>())
                .join("\n"),
        )
    }
}

impl<const M: usize> FromStr for ConstMap<M> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = [[' '; M]; M];
        for (i, c) in s[0..M * M].chars().enumerate() {
            inner[i / M][i % M] = c;
        }
        Ok(ConstMap { inner })
    }
}

impl<const M: usize> ConstMap<M> {
    pub fn size(&self) -> usize {
        M
    }

    pub fn transpose(&mut self) {
        for r in 0..M {
            for c in r..M {
                // trust me
                if c != r {
                    unsafe {
                        ptr::swap(&mut self.inner[r][c], &mut self.inner[c][r]);
                    }
                }
            }
        }
    }

    pub fn flip_vertical(&mut self) {
        for row in &mut self.inner {
            row.reverse();
        }
    }
}
