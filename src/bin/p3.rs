use std::fs;

fn main() {
    let contents: Vec<Vec<u8>> = fs::read("inputs/p3/schematic.txt")
        .expect("reading file")
        .split(move |&c| c == b'\n')
        .map(|s| s.to_vec())
        .collect();

    // idea: find symbols. Check if there is a number adjacent ot it
    // (must touch any of the four sides, corners are also ok).
    let mut part_ids = vec![];
    let mut sum_of_ratios = 0;

    for (r_idx, row) in contents.iter().enumerate() {
        for (c_idx, &c) in row.iter().enumerate() {
            let c = c as char;
            match c {
                '.' | '0'..='9' => (), // not a symbol
                _ if c.is_ascii_punctuation() => {
                    if let Some((p, ids)) = get_part_id(&contents, r_idx, c_idx) {
                        // if part is a gear, accumulate gear ratio (two part numbers, ratio = a * b)
                        if p == '*' && ids.len() == 2 {
                            sum_of_ratios += ids[0].3 * ids[1].3;
                        }

                        // add part id in vector
                        for id in ids {
                            part_ids.push(id);
                        }
                    }
                }
                o => unreachable!("Got impossible char: {o:?}"),
            };
        }
    }

    part_ids.sort();
    part_ids.dedup();

    println!(
        "Sum of part ids: {}",
        part_ids.iter().map(|(_, _, _, n)| n).sum::<u32>()
    );
    println!("Sum of gear ratios: {sum_of_ratios}");
}

/// This can create duplicates. Be sure to remove them!
/// Returns the `(row, start_col, len, id)` of the part.
/// This is needed to remove duplicates safely, as I'm not sure whether part numbers can be
/// duplicated.
fn get_part_id(
    contents: &Vec<Vec<u8>>,
    r_idx: usize,
    c_idx: usize,
) -> Option<(char, Vec<(usize, usize, usize, u32)>)> {
    let val =
        |r_idx: usize, c_idx: usize| contents.get(r_idx).and_then(|row| row.get(c_idx)).copied();

    let digit_pos = [
        //edges
        (r_idx - 1, c_idx),
        (r_idx + 1, c_idx),
        (r_idx, c_idx - 1),
        (r_idx, c_idx + 1),
        //corners
        (r_idx - 1, c_idx - 1),
        (r_idx - 1, c_idx + 1),
        (r_idx + 1, c_idx - 1),
        (r_idx + 1, c_idx + 1),
    ]
    .into_iter()
    .filter(|p| {
        val(p.0, p.1)
            .map(move |c| (c as char).is_digit(10))
            .unwrap_or(false)
    });

    let mut part_ids: Vec<_> = digit_pos
        .map(|(dr, mut dc)| {
            // go left until the start of the number
            while dc > 0 {
                // cannot be None, otherwise puzzle makes no sense
                if (val(dr, dc - 1).unwrap() as char).is_digit(10) {
                    dc -= 1;
                } else {
                    break;
                }
            }

            let len = contents[dr][dc..]
                .iter()
                .position(move |&c| !(c as char).is_digit(10))
                .unwrap_or(contents[dr].len() - dc);
            let number_str = std::str::from_utf8(&contents[dr][dc..(dc + len)]).unwrap();
            let number = u32::from_str_radix(number_str, 10).unwrap();

            (dr, dc, len, number)
        })
        .collect();

    part_ids.sort();
    part_ids.dedup();

    if part_ids.is_empty() {
        None
    } else {
        Some((contents[r_idx][c_idx] as char, part_ids))
    }
}
