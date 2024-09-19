use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/p15/input.txt")
        .expect("reading input file containing initialization sequence");
    let part1 = content
        .split(',')
        .map(hash_str)
        .fold(0u32, |a, b| a + b as u32);
    println!("[PART 1] Sum of hashes: {part1}");

    // Map (Box, Slot) -> FocalLength
    let boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];
    let final_boxes = content
        .split(',')
        .filter_map(parse_op)
        .fold(boxes, |mut acc, op| {
            let _ = match op {
                Op::Remove(label) => acc[hash_str(label) as usize].retain(|&(l, _)| l != label),
                Op::Add(label, fc) => {
                    if let Some(replaced) = acc[hash_str(label) as usize]
                        .iter_mut()
                        .find(|(l, _)| *l == label)
                    {
                        replaced.1 = fc;
                    } else {
                        acc[hash_str(label) as usize].push((label, fc));
                    }
                }
            };
            acc
        });

    // sum of focusing power
    let part2: usize = final_boxes
        .into_iter()
        .enumerate()
        .map(|(idx, bx)| {
            (idx + 1)
                * bx.into_iter()
                    .enumerate()
                    .map(|(slot, (_, fc))| (slot + 1) * fc as usize)
                    .sum::<usize>()
        })
        .sum();

    println!("[PART 2] Sum of focusing power: {part2}");
}

enum Op<'s> {
    /// Remove lens with given label from the box obtained by hashing the label with `hash_str`.
    Remove(&'s str),
    /// Add lens with given label and focal length to the box obtained by hashing the label with `hash_str`.
    Add(&'s str, u8),
}

fn parse_op(s: &str) -> Option<Op<'_>> {
    if let Some((label, fc)) = s.split_once('=') {
        Some(Op::Add(label.trim(), fc.parse().unwrap()))
    } else if s.contains('-') {
        Some(Op::Remove(s.trim().trim_end_matches('-')))
    } else {
        None
    }
}

fn hash_str(s: &str) -> u8 {
    s.chars()
        .filter(|c| !c.is_whitespace())
        .fold(0, |acc, c| acc.wrapping_add(c as u8).wrapping_mul(17))
}
