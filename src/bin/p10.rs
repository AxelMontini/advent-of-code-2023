use colored::Colorize;
use std::fs;

struct Walker<'s> {
    map: &'s [u8],
    width: usize,
    height: usize,
    /// X,Y
    pos: (usize, usize),
    /// previous position, to avoid going backwards.
    prev: (usize, usize),
    walk_len: usize,
}

impl<'s> Walker<'s> {
    pub fn new(x: usize, y: usize, map: &'s [u8], width: usize, height: usize) -> Self {
        assert_eq!(map.len(), width * height);
        Self {
            map,
            width,
            height,
            pos: (x, y),
            prev: (x, y),
            walk_len: 0,
        }
    }

    pub fn walk(&mut self) {}
}

fn main() {
    // it's ascii, rust strings don't allow indexing normally, since they are UTF-8
    let content = fs::read("inputs/p10/pipes.txt").expect("reading input file pipes.txt");
    let width = 1 + content
        .iter()
        .position(move |&b| (b as char).is_whitespace())
        .expect("cannot find end of line");
    let height = content.len() / width;
    // case in just
    assert_eq!(content.len(), width * height);

    // position X,Y of S, from top-left corner of the map
    let pos_to_idx = move |x: usize, y: usize| x + y * width;
    let idx_to_pos = move |idx: usize| (idx % width, idx / width);
    let s_pos = content
        .iter()
        .position(move |&b| b == b'S')
        .map(idx_to_pos)
        .unwrap();
    println!("S pos: {s_pos:?}");

    let pipe = |x: usize, y: usize| content.get(pos_to_idx(x, y));
    let connections = |x: usize, y: usize| {
        pipe(x, y)
            // .inspect(move |&p| println!("Read '{}'", *p as char))
            .and_then(|p| {
                Some(match p {
                    b'S' => &[(1, 0), (0, 1), (-1, 0), (0, -1)][..],
                    b'7' => &[(-1, 0), (0, 1)][..],
                    b'L' => &[(1, 0), (0, -1)][..],
                    b'F' => &[(1, 0), (0, 1)][..],
                    b'J' => &[(-1, 0), (0, -1)][..],
                    b'|' => &[(0, -1), (0, 1)][..],
                    b'-' => &[(-1, 0), (1, 0)][..],
                    _ => return None,
                })
            })
    };

    let next_segments = |cur: (usize, usize), prev: (usize, usize)| {
        connections(cur.0, cur.1)
            .unwrap()
            .iter()
            .filter_map(move |displ| {
                cur.0
                    .checked_add_signed(displ.0)
                    .zip(cur.1.checked_add_signed(displ.1))
            })
            .filter(move |&p| p != prev)
            .filter(move |&p| {
                [b'7', b'L', b'F', b'J', b'-', b'|', b'S'].contains(pipe(p.0, p.1).unwrap())
            })
    };

    // Now start at S. Two pointers walk the pipe in two opposite directions. When walker A meets B, the
    // output is B's distance.
    // [current, previous] positions, start at s_pos, then walk in opposite directions
    let mut start_segments = next_segments(s_pos, s_pos);
    let mut walker = (start_segments.next().unwrap(), s_pos, 1);
    let mut loop_bitmap = vec![0i32; content.len()];
    loop_bitmap[pos_to_idx(s_pos.0, s_pos.1)] = 1;
    println!("Start with S = {s_pos:?}, walker = {walker:?}");

    while pipe(walker.0 .0, walker.0 .1).copied() != Some(b'S') {
        // set bitmap, to be used for part 2
        loop_bitmap[pos_to_idx(walker.0 .0, walker.0 .1)] = walker.2 + 1;
        let next = next_segments(walker.0, walker.1).next().unwrap();

        walker.1 = walker.0;
        walker.0 = next;
        walker.2 += 1;
    }

    println!("[PART 1] Distance to furthest point: {}", walker.2 / 2,);

    // now, for part 2, we need to replace S with the correct pipe piece equivalent
    let content = {
        let adj = [
            (s_pos.0 - 1, s_pos.1),
            (s_pos.0 + 1, s_pos.1),
            (s_pos.0, s_pos.1 - 1),
            (s_pos.0, s_pos.1 + 1),
        ];

        let n1 = adj
            .into_iter()
            .filter(|&(x, y)| loop_bitmap[pos_to_idx(x, y)] != 0)
            .min_by_key(|&(x, y)| loop_bitmap[pos_to_idx(x, y)])
            .unwrap();
        let n2 = adj
            .into_iter()
            .filter(|&(x, y)| loop_bitmap[pos_to_idx(x, y)] != 0)
            .max_by_key(|&(x, y)| loop_bitmap[pos_to_idx(x, y)])
            .unwrap();

        let replacement = match ((n1.0 as i32 - s_pos.0 as i32, n1.1 as i32 - s_pos.1 as i32), (n2.0 as i32 - s_pos.0 as i32, n2.1 as i32 - s_pos.1 as i32)) {
            ((1, 0), (0, 1)) => b'F',
            ((1, 0), (-1, 0)) => b'-',
            ((1, 0), (0, -1)) => b'L',
            ((0, 1), (-1, 0)) => b'7',
            ((0, 1), (0, -1)) => b'|',
            ((-1, 0), (0, -1)) => b'J',
            (a, b) => unreachable!("given the order of choice of the next segment, this cannot happen! a = {a:?}, b = {b:?}"),
        };

        let mut c = content.clone();
        c[pos_to_idx(s_pos.0, s_pos.1)] = replacement;
        c
    };
    let pipe = |x: usize, y: usize| content.get(pos_to_idx(x, y));

    // Print the Bitmap
    println!("Bitmap:");
    let s = &mut [0; 1][..];
    loop_bitmap
        .chunks(width)
        .zip(content.chunks(width))
        .for_each(|(bit_row, c_row)| {
            bit_row.iter().zip(c_row.iter()).for_each(|(&b, &e)| {
                print!("{}", {
                    s[0] = e;
                    let st = std::str::from_utf8(s).unwrap();
                    if b != 0 {
                        st.green()
                    } else {
                        st.red()
                    }
                })
            });
        });

    // An adaptation of the winding number algorithm could be good. Having a bitmap marked with the
    // pipe path allows to then scan every line. Each time we encounter a pipe, we flip a boolean
    // (`in_loop`, initially `false`). Then every tile that is not part of the loop that we
    // encounter while it's `true` is inside of the loop. We count them.

    let tiles: u32 = loop_bitmap
        .chunks(width)
        .enumerate()
        .map(|(row_id, row)| {
            row.iter()
                .enumerate()
                .fold((0, 0), |(count, winding), (col_id, &tile_is_loop)| {
                    // adaptation of the winding number algorithm.
                    // If we cross a loop pipe that goes upwards, decrement winding.
                    // If we cross a loop pipe going downwards, increment winding.
                    // Otherwise, keep it unchanged.
                    // We are inside a the polygon (the loop) iff winding != 0
                    // Definition of "upwards": the tile above (or below, depending on 'J' or '7',
                    // ...) has greater distance than the current one

                    let s = &mut [0];
                    let p = (*pipe(col_id, row_id).unwrap() as char).encode_utf8(s);

                    let winding_incr = match (tile_is_loop, pipe(col_id, row_id).unwrap()) {
                        (0, _) => 0,
                        (d, b'L' | b'J') => loop_bitmap
                            .get(pos_to_idx(col_id, row_id - 1))
                            .map(|&v| d - v)
                            .map(|v| if v.abs() > 2 { -v.signum() } else { v })
                            .unwrap_or(0),
                        (d, b'7' | b'F') => loop_bitmap
                            .get(pos_to_idx(col_id, row_id + 1))
                            .map(|&v| v - d)
                            .map(|v| if v.abs() > 2 { -v.signum() } else { v })
                            .unwrap_or(0),
                        (d, b'|') => {
                            loop_bitmap
                                .get(pos_to_idx(col_id, row_id - 1))
                                .map(|&v| d - v)
                                .map(|v| if v.abs() > 2 { -v.signum() } else { v })
                                .unwrap_or(0)
                                * 2
                        }
                        _ => 0,
                    };
                    let next_winding = winding + winding_incr;
                    print!(
                        "{}",
                        match next_winding.cmp(&winding) {
                            std::cmp::Ordering::Less => p.red(),
                            std::cmp::Ordering::Equal =>
                                if winding != 0 {
                                    if tile_is_loop != 0 {
                                        p.blue()
                                    } else {
                                        p.yellow().on_purple()
                                    }
                                } else {
                                    p.clear()
                                },
                            std::cmp::Ordering::Greater => p.green(),
                        }
                    );
                    (
                        count + (winding != 0 && tile_is_loop == 0) as u32,
                        next_winding,
                    )
                })
                .0
        })
        .sum();
    println!("[PART 2] Tiles in loop: {tiles}");
}
