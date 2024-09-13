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
    for (id, block) in blocks.into_iter().enumerate() {
        // Mirror is at index `col` (between element `col-1` and `col`)
        part1 += get_vertical_refl_line(&block).unwrap_or(0);

        // Mirror is at index `row` (between rows `row-1` and `row`)
        part1 += 100 * get_horizontal_refl_line(&block).unwrap_or(0);
    }

    println!("[PART 1] Sum of cols/rows according to spec: {part1}");
}

fn get_refl_line<I, T>(it: I) -> Option<usize>
where
    I: Iterator<Item = T> + Clone + DoubleEndedIterator + ExactSizeIterator,
    T: Eq + std::fmt::Debug,
{
    let len = it.len();
    for idx in 1..len {
        let min_length = idx.min(len - idx);
        let halves_equal = it
            .clone()
            .skip(idx - min_length)
            .take(min_length)
            .rev()
            .eq(it.clone().skip(idx).take(min_length));

        println!(
            "Up {idx}: {:?}",
            it.clone()
                .skip(idx - min_length)
                .take(min_length)
                .rev()
                .collect::<Vec<_>>()
        );
        println!(
            "Down {idx}: {:?}",
            it.clone().skip(idx).take(min_length).collect::<Vec<_>>()
        );
        if halves_equal {
            println!("AAA {idx}");
            return Some(idx);
        }
    }

    None
}

fn get_vertical_refl_line(block: &Block) -> Option<usize> {
    let col = get_refl_line(block.columns.iter().copied());
    col
}

fn get_horizontal_refl_line(block: &Block) -> Option<usize> {
    let row = get_refl_line(block.rows.iter().copied());
    row
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
}
