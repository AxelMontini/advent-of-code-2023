use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block {
    rows: Vec<u64>,
    columns: Vec<u64>,
}

fn main() {
    let content = fs::read_to_string("inputs/p13/reflections.txt").expect("reading input file");
    let blocks: Vec<_> = content.split("\n\n").map(parse_block).collect();

    // Sum of columns to the LEFT of the vertical reflection line,
    // plus 100 TIMES the rows above the horizontal refl. line.
    let mut part1 = 0;
    let mut part2 = 0;

    // scan the blocks' rows and columns. Start at a certain position, and try to match the two
    // halves
    for block in blocks.into_iter() {
        part1 += get_vertical_refl_line(&block, 0)
            .or_else(|| get_horizontal_refl_line(&block, 0).map(|v| v * 100))
            .expect("there must be a reflection line");

        part2 += get_vertical_refl_line(&block, 1)
            .or_else(|| get_horizontal_refl_line(&block, 1).map(|v| v * 100))
            .expect("there must be a reflection line");
    }

    println!("[PART 1] Sum of cols/rows according to spec: {part1}");
    println!("[PART 2] Sum of cols/rows according to spec: {part2}");
}

/// Compare the two sequences.
/// Find the index at which the second reflection starts.
/// Exactly `diffs` pairs can differ
fn get_refl_line(it: &[u64], diffs: u32) -> Option<usize> {
    let len = it.len();
    for idx in 1..len {
        let min_length = idx.min(len - idx);
        // Iterate over pairs of elements.
        // Get all the elements that DIFFER.
        // Get the count of all the ones that differ by ONE bit. Fail if any differs by more.
        // Exactly `diffs` pairs must differ (the Some(diffs) eq check is to ensure the prev two
        // properties).
        let halves_equal = it[..idx]
            .iter()
            .rev()
            .take(min_length)
            .zip(it[idx..].iter().take(min_length))
            .filter(|(a, b)| a != b)
            .try_fold(0, |acc, (a, b)| {
                ((a ^ b).count_ones() <= 1).then_some(acc + 1)
            })
            == Some(diffs);

        if halves_equal {
            return Some(idx);
        }
    }

    None
}

fn get_vertical_refl_line(block: &Block, diffs: u32) -> Option<usize> {
    get_refl_line(&block.columns, diffs)
}

fn get_horizontal_refl_line(block: &Block, diffs: u32) -> Option<usize> {
    get_refl_line(&block.rows, diffs)
}

fn parse_block(s: &str) -> Block {
    let rows = s
        .lines()
        .map(|l| encode_seq(l.chars()))
        .collect::<Option<_>>()
        .expect("encoding rows");
    let width = s.lines().next().unwrap().len();
    let columns = (0..width)
        .map(|col| encode_seq(s.lines().map(|l| l[col..].chars().next().unwrap())))
        .collect::<Option<_>>()
        .expect("encoding columns");

    Block { rows, columns }
}

/// Encode a line ONLY composed of '.' and '#', with length less than 64.
fn encode_seq<I>(mut s: I) -> Option<u64>
where
    I: Iterator<Item = char> + Clone,
{
    if s.clone().count() > u64::BITS as usize {
        return None;
    }

    s.try_fold(0, |acc, c| {
        Some(
            (acc << 1)
                | match c {
                    '.' => 0,
                    '#' => 1,
                    _ => return None,
                },
        )
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn encode_line() {
        let values = [(".", 0), ("#", 1), ("#.", 2), ("##", 3)];

        for (input, expected) in values {
            assert_eq!(Some(expected), crate::encode_seq(input.chars()));
        }
    }

    #[test]
    fn parse_block() {
        let values = [
            ("#", &[1][..], &[1][..]),
            (
                "####\n....\n####",
                &[5, 5, 5, 5][..],
                &[0b1111, 0, 0b1111][..],
            ),
        ];

        for (input, cols, rows) in values {
            let block = crate::parse_block(input);
            assert_eq!(cols, block.columns);
            assert_eq!(rows, block.rows);
        }
    }

    #[test]
    fn reflections() {
        let values = [
            (
                "#.##..##.\n\
            ..#.##.#.\n\
            ##......#\n\
            ##......#\n\
            ..#.##.#.\n\
            ..##..##.\n\
            #.#.##.#.",
                0,
                Some(5),
                None,
            ),
            (
                "#.##..##.\n\
            ..#.##.#.\n\
            ##......#\n\
            ##......#\n\
            ..#.##.#.\n\
            ..##..##.\n\
            #.#.##.#.",
                1,
                None,
                Some(3),
            ),
        ];

        for (input, diffs, exp_col, exp_row) in values {
            let block = crate::parse_block(input);
            assert_eq!(exp_col, crate::get_vertical_refl_line(&block, diffs));
            assert_eq!(exp_row, crate::get_horizontal_refl_line(&block, diffs));
        }
    }
}
