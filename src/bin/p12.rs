//! # Overview
//!
//! The recursive operation first checks if there are any sets left. If not, then it returns
//! 1 when only spaces remain in the string (or empty), otherwise 0.
//! The recursive operation is, for all possible prefix space counts (from 1 to ...):
//! 1. Check if prefix is all possible spaces (`?` or `.`)
//! 2. Check if the current set exists starting after the spaces (`?` or `#`)
//! 3. If both conditions are true, run the operation again on the remainder of the string/sets and
//!    add the result to count
//! Once the loop is over, return the count.
//!
//! This operation is memoized for part 2, otherwise runtime quickly approaches balls.
//! Note that, to decrease runtime a bit, I trim `.` from the string. This must be done AFTER
//! the "repeat" operation in part 2, otherwise it breaks the result (I was doing this in `parse_line` initially,
//! and it caused a big headache before I realized why all tests were passing, but the program
//! was wrong)...

use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/p12/springs.txt").expect("reading input file");
    let count: usize = content
        .lines()
        .map(parse_line)
        .map(|(arrangement, sets)| count_arrangements(arrangement.trim_matches('.'), &sets[..]))
        .sum();

    println!("[PART 1] Sum of arrangements: {count}");

    let count: usize = content
        .lines()
        .map(parse_line)
        .map(|(ar, s)| (format!("{0}?{0}?{0}?{0}?{0}", ar), s.repeat(5)))
        .map(|(arrangement, sets)| count_arrangements(arrangement.trim_matches('.'), &sets[..]))
        .sum();

    println!("[PART 2] Sum of arrangements: {count}");
}

fn parse_line(l: &str) -> (&str, Vec<usize>) {
    let mut line = l.split_whitespace();
    // ==================
    // XXX: DO NOT DO THIS! THIS BREAKS PART 2. Triming must be done AFTER repeating!
    // dots at either end do not influence the solution, they can only add runtime
    // let arrangement = line.next().unwrap().trim_matches('.');
    let arrangement = line.next().unwrap();
    let sets: Vec<usize> = line
        .next()
        .unwrap()
        .split(',')
        .map(|l| l.parse())
        .collect::<Result<_, _>>()
        .expect("parsing contiguous set lengths");
    (arrangement, sets)
}

/// Memoized recursive counting algorithm.
fn count_arrangements(arrangement: &str, sets: &[usize]) -> usize {
    // There must be a ONE SPACE (.) prefix for the recursive algo to work
    let arrangement = format!(".{arrangement}");
    let max_spaces: usize = arrangement.len() - sets.iter().sum::<usize>();

    let ar_len_initial = arrangement.len();
    let mut memo = vec![0; (sets.len() + 1) * ar_len_initial + 1];
    let memo_idx = move |ar_len, sets_len| sets_len * ar_len_initial + ar_len;
    recurse(&arrangement, sets, max_spaces, &mut memo[..], &memo_idx)
}

fn is_working(c: char) -> bool {
    c == '.' || c == '?'
}
fn is_broken(c: char) -> bool {
    c == '#' || c == '?'
}

/// Advances set-by-set recursively.
/// 1. Checks if there is a valid space prefix of length >= 1.
/// 2. Checks if there is a valid set of length sets[0].
/// If both hold, it advances.
/// - If sets is empty, checks if the string only contains spaces. If so, return 1 (valid config),
/// otherwise 0 (invalid config, broken springs left at end of the row).
///
/// # Memoization
///
/// Each suffix can be memoized (that is, a configuration of `(ar.len(), sets.len()) -> count` is saved).
/// The memo array contains 0 is empty (not computed yet), otherwise it contains `count+1`.
fn recurse(
    ar: &str,
    sets: &[usize],
    max_spaces: usize,
    memo: &mut [usize],
    memo_idx: &dyn Fn(usize, usize) -> usize,
) -> usize {
    if memo[memo_idx(ar.len(), sets.len())] != 0 {
        return memo[memo_idx(ar.len(), sets.len())] - 1;
    }

    if sets.is_empty() {
        // we are done only if there are no more sets.
        // Success if the remainder is only spaces, otherwise failure.
        return ar.chars().all(is_working) as _;
    }

    let mut count = 0;
    for space_count in 1..=(max_spaces) {
        let (spaces, rest) = ar.split_at(space_count);
        let spaces_ok = spaces.chars().all(is_working);
        if !spaces_ok {
            break; // space prefix too long, no further solution is possible. We break here.
        }

        let (set, rest) = rest.split_at(sets[0]);
        let set_ok = set.chars().all(is_broken);

        if set_ok {
            count += recurse(rest, &sets[1..], max_spaces - space_count, memo, memo_idx);
        }
    }

    memo[memo_idx(ar.len(), sets.len())] = count + 1;

    count
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::count_arrangements;

    #[test]
    fn easy() {
        let trivial = [
            ("#", &[1][..], 1),
            ("##", &[2][..], 1),
            ("?", &[1][..], 1),
            ("?.", &[1][..], 1),
            ("#?", &[1][..], 1),
        ];
        let two = [("??", &[1][..], 2), ("?.?", &[1][..], 2)];
        let complex = [
            ("????????", &[3, 4][..], 1),
            ("?#??#????", &[1, 2, 2][..], 3),
        ];

        let test_set = trivial.into_iter().chain(two).chain(complex);

        for (arrangements, sets, expected) in test_set {
            let count = count_arrangements(arrangements, sets);
            assert_eq!(
                count, expected,
                "Ar = '{arrangements}', sets = {sets:?}, expected = {expected}, count = {count}"
            );
        }
    }

    #[test]
    fn quintuple() {
        let values = [
            (".??..??...?##.", &[1, 1, 3][..], 16384),
            ("?###????????", &[3, 2, 1][..], 506250),
        ];

        let test_set = values.into_iter();

        for (arrangements, sets, expected) in test_set {
            let arrangements = &[arrangements].repeat(5).join("?");
            let sets = &sets.repeat(5)[..];
            let count = count_arrangements(arrangements, sets);
            assert_eq!(
                count, expected,
                "Ar = '{arrangements}', sets = {sets:?}, expected = {expected}, count = {count}"
            );
        }
    }
}
