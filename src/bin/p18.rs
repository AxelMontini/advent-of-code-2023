use std::fs;

type Pos = (isize, isize);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Color(u32);

impl Color {
    pub fn new(v: u32) -> Option<Self> {
        if v <= 0xFFFFFF {
            Some(Self(v))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct Line {
    len: usize,
    dir: Dir,
    color: Color,
}

fn dir_step(d: Dir) -> (isize, isize) {
    match d {
        Dir::Up => (0, -1),
        Dir::Right => (1, 0),
        Dir::Down => (0, 1),
        Dir::Left => (-1, 0),
    }
}

fn main() {
    let content = fs::read_to_string("inputs/p18/input.txt").expect("reading input");

    let lines_iter = content.lines().map(parse_line);

    println!("?????");
    // first find position boundary by recording max, min x and y coords
    let (_, xmin, xmax, ymin, ymax) = lines_iter.clone().fold(
        ((0isize, 0), 0, 0, 0, 0),
        |(mut p, xmin, xmax, ymin, ymax), l| {
            let step = dir_step(l.dir);
            p = (p.0 + step.0 * l.len as isize, p.1 + step.1 * l.len as isize);
            (
                p,
                xmin.min(p.0),
                xmax.max(p.0),
                ymin.min(p.1),
                ymax.max(p.1),
            )
        },
    );

    println!("xmin={xmin} xmax={xmax} ymin={ymin} ymax={ymax}");

    let mut pos = (xmin.abs() as usize, ymin.abs() as usize);
    let width = (xmax + 1 - xmin) as _;
    let height = (ymax + 1 - ymin) as _;
    let mut bitmap: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let mut up_down = vec![vec![None; width]; height];
    bitmap[pos.1][pos.0] = true;
    for line in lines_iter {
        let step = dir_step(line.dir);
        if up_down[pos.1][pos.0].is_none() && (line.dir == Dir::Up || line.dir == Dir::Down) {
            up_down[pos.1][pos.0] = Some(line.dir);
        }
        for _ in 0..line.len {
            pos = (
                pos.0.checked_add_signed(step.0).unwrap(),
                pos.1.checked_add_signed(step.1).unwrap(),
            );
            bitmap[pos.1][pos.0] = true;
            if up_down[pos.1][pos.0].is_none() && (line.dir == Dir::Up || line.dir == Dir::Down) {
                up_down[pos.1][pos.0] = Some(line.dir);
            }
        }
    }

    // need to use winding-number algorithm

    let mut area = 0;
    for ri in 0..height {
        let mut c = 0i32;
        let mut last_dir = Dir::Left;
        for (b, (_, d)) in bitmap[ri].iter().zip(up_down[ri].iter().enumerate()) {
            match d {
                Some(Dir::Up) if last_dir != Dir::Up => {
                    c += 1;
                    last_dir = Dir::Up
                }
                Some(Dir::Down) if last_dir != Dir::Down => {
                    c -= 1;
                    last_dir = Dir::Down
                }
                _ => (),
            };

            if c != 0 || *b {
                area += 1;
            }
        }
    }

    println!("[PART1] Area: {area}");
}

fn parse_line(s: &str) -> Line {
    let mut elems = s.split_whitespace();
    let dir = match elems.next().unwrap() {
        "D" => Dir::Down,
        "R" => Dir::Right,
        "U" => Dir::Up,
        "L" => Dir::Left,
        d => panic!("invalid direction {d} in input lines"),
    };
    let len = elems.next().unwrap().parse().expect("parsing line length");
    let color = u32::from_str_radix(elems.next().unwrap().trim_matches(&['(', ')', '#']), 16)
        .expect("parsing color");
    let color = Color::new(color).expect("invalid color value");

    Line { dir, len, color }
}
