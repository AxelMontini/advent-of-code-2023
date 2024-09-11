//! ## Part 1
//!Length of shortest path between a pair of galaxies `a, b` is `|ax - bx| + |ay - by|`.
//! I have not seen part 2 (at the time of writing this), so idk if it makes sense to actually
//! duplicate the empty rows/cols or to just count them when calculating distances.
//! For now I'll actually duplicate.
//! ## Part 2
//! Well, fuck. Part 2 is not doable with the current implementation (definitely unexpected,
//! kappachungus maximus deluxe...).
//! Instead I will keep track of empty rows and columns in two lists,
//! and I will add them to the distances accordingly. These lists are indexed
//! by row_idx and col_idx respectively, so this should be easy. At this point I'll rewrite part1's
//! code too, so I can use the same distance counting function, just changing the empty row
//! coefficient.

use std::{fs, num::NonZeroU16};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Cell {
    Empty,
    Galaxy(NonZeroU16),
}

const _: () = assert!(std::mem::size_of::<Cell>() == std::mem::size_of::<u16>());

fn main() {
    let content = fs::read_to_string("inputs/p11/picture.txt").expect("reading input file");
    let mut counter = 0;
    let galaxy_map: Vec<Vec<Cell>> = content
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '#' => {
                        counter += 1;
                        Cell::Galaxy(counter.try_into().unwrap())
                    }
                    o => panic!("invalid character '{o}' in picture input!"),
                })
                .collect()
        })
        .collect();

    let empty_rows: Vec<_> = galaxy_map
        .iter()
        .enumerate()
        .filter_map(|(ri, row)| row.iter().all(|&e| e == Cell::Empty).then_some(ri))
        .collect();
    let empty_cols: Vec<_> = (0..galaxy_map[0].len())
        .filter(|&j| galaxy_map.iter().all(|row| row[j] == Cell::Empty))
        .collect();

    // (y, x, &cell)
    let galaxies: Vec<(usize, usize, &Cell)> = galaxy_map
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| matches!(cell, Cell::Galaxy(_)))
                .map(move |(c, cell)| (r, c, cell))
        })
        .collect();

    let distance = |coeff: u64| {
        let mut dist = 0;
        for i in 0..(galaxies.len()) {
            for j in i..(galaxies.len()) {
                // compute distance between pair a, b (easy af)
                let (a, b) = (galaxies[i], galaxies[j]);
                assert_eq!(Cell::Galaxy((i as u16 + 1).try_into().unwrap()), *a.2);
                assert_eq!(Cell::Galaxy((j as u16 + 1).try_into().unwrap()), *b.2);

                let ey = empty_rows
                    .binary_search(&a.0)
                    .unwrap_err()
                    .abs_diff(empty_rows.binary_search(&b.0).unwrap_err())
                    as u64;
                let ex = empty_cols
                    .binary_search(&a.1)
                    .unwrap_err()
                    .abs_diff(empty_cols.binary_search(&b.1).unwrap_err())
                    as u64;
                let empty_space = ey as u64 * coeff + ex as u64 * coeff;

                dist += a.0.abs_diff(b.0) as u64 + a.1.abs_diff(b.1) as u64 + empty_space - ey - ex;
            }
        }
        dist
    };

    println!(
        "[PART 1] Sum of distance between unique pairs: {}",
        distance(2)
    );
    println!(
        "[PART 2] Sum of distance between unique pairs: {}",
        distance(1000000)
    );
}
