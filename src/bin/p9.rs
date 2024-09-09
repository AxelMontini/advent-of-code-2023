use core::fmt;
use std::{borrow::Cow, fs};

#[derive(Clone, Debug)]
struct History<'d> {
    data: Cow<'d, [i64]>,
    /// History's length is at most `(data.len() * (data.len() - 1)) / 2`.
    history: Vec<i64>,
    /// amount of rows, so I don't have to compute it later
    rows: usize,
}

impl<'d> History<'d> {
    pub fn new(data: impl Into<Cow<'d, [i64]>>) -> Self {
        let data = data.into();
        // initialize history with the diff between each consecutive pair of values.
        let mut history: Vec<i64> = data.windows(2).map(|pair| pair[1] - pair[0]).collect();
        // now keep doing it until the diff is 0
        let mut prev = 0;
        let mut rows = 0;
        for row_i in 0..data.len() {
            for col_i in 0..(data.len() - row_i - 2) {
                history.push(history[prev + col_i + 1] - history[prev + col_i]);
            }

            prev = prev + data.len() - 1 - row_i;

            if history[prev..].iter().all(move |&d| d == 0) {
                rows = row_i + 1;
                break;
            }
        }

        Self {
            data,
            history,
            rows,
        }
    }

    pub fn extrapolate_back(&self) -> i64 {
        // each step is `x := b + v`, until the top
        // a b x
        //  c v
        //
        //  Iter from last element of second-last row of history, until last element of first row.
        //  Last row is all zeros, so no need to read it, just init `v := 0`
        let mut v = 0;
        let mut i = self.history.len() - 1 - self.data.len() + self.rows + 1; // TODO: fix if 1 row
        let mut row_i = self.rows - 1;
        loop {
            let b = self.history[i];
            v = b + v;

            if row_i == 0 {
                break;
            }
            i -= self.data.len() - 1 - row_i;
            row_i -= 1;
        }

        v + self.data.last().unwrap()
    }

    pub fn extrapolate_front(&self) -> i64 {
        // each step is `x := a - v`, until the top
        // x a b
        //  v c
        //
        //  Iter from last element of second-last row of history, until last element of first row.
        //  Last row is all zeros, so no need to read it, just init `v := 0`
        let mut v = 0;
        let mut row_i = self.rows - 1;
        let mut i = (1..=row_i).fold(0, |acc, i| acc + self.data.len() - i);
        loop {
            println!("Iter on row {row_i}");
            // assert_eq!(i, (0..=row_i).fold(0, |acc, i| acc + self.data.len() - i));

            let a = self.history[i];
            println!("a={a}, v={v} ==> v={}", a - v);
            v = a - v;

            if row_i == 0 {
                break;
            }
            i -= self.data.len() - row_i;
            row_i -= 1;
        }

        self.data.first().unwrap() - v
    }
}

fn main() {
    let content = fs::read_to_string("inputs/p9/data.txt").expect("reading input data");
    let data: Vec<History<'_>> = content
        .lines()
        .map(|line| {
            History::new(
                line.split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let sum: i64 = data.iter().map(|h| h.extrapolate_back()).sum();
    println!("[PART 1] Sum = {sum}");
    let sum: i64 = data.iter().map(|h| h.extrapolate_front()).sum();
    println!("[PART 2] Sum = {sum}");
}

#[cfg(test)]
mod tests {
    use crate::History;

    #[test]
    fn new_history() {
        let values = &[
            (&[1, 1][..], &[0][..]),
            (&[1, 2, 3][..], &[1, 1, 0][..]),
            (&[1, 3, 7, 13][..], &[2, 4, 6, 2, 2, 0][..]),
        ][..];

        for &(input, expected) in values {
            let out = History::new(input);
            assert_eq!(input, out.data.as_ref());
            assert_eq!(
                expected, &out.history,
                "Expected history {expected:?}, but got {out:?}",
            );
        }
    }

    #[test]
    fn extrapolate() {
        let values = &[
            (&[1, 1, 1, 1][..], 1),
            (&[1, 2, 3, 4][..], 5),
            (&[1, 3, 7, 13][..], 21),
            /*
             * 1  3  7  13 |21
             *   2  4  6  |8
             *     2  2  |2
             *       0  |0
             */
        ][..];

        for &(input, expected) in values {
            assert_eq!(expected, History::new(input).extrapolate_back());
        }

        // High-degree polynomial test
        {
            let f = |x: i64| x.pow(10);
            let input: Vec<_> = (-50..=50).map(|x| f(x)).collect();

            let h = History::new(input);
            assert_eq!(f(51), h.extrapolate_back());
            assert_eq!(f(-51), h.extrapolate_front(), "with history {:?}", h);
        }
    }
}
