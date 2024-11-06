use std::{
    fs,
    ops::{Add, Mul, Sub},
};

/// Cartesian Position `(x,y)`
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Pos(isize, isize);

impl From<(isize, isize)> for Pos {
    fn from(value: (isize, isize)) -> Self {
        Self(value.0, value.1)
    }
}

impl<Rhs: Into<Pos>> Sub<Rhs> for Pos {
    type Output = Self;

    fn sub(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.into();
        (self.0 - rhs.0, self.1 - rhs.1).into()
    }
}

impl<Rhs: Into<Pos>> Add<Rhs> for Pos {
    type Output = Self;

    fn add(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.into();
        (self.0 + rhs.0, self.1 + rhs.1).into()
    }
}

impl Mul<isize> for Pos {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        (self.0 * rhs, self.1 * rhs).into()
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn is_right_turn(self, rhs: Self) -> bool {
        rhs.is_left_turn(self)
    }
    fn is_left_turn(self, rhs: Self) -> bool {
        use Dir::*;
        matches!(
            (self, rhs),
            (Up, Left) | (Left, Down) | (Down, Right) | (Right, Up)
        )
    }
    /// returns the direction in vector format (as Pos)
    fn dir_step(self) -> Pos {
        match self {
            Dir::Up => (0, -1).into(),
            Dir::Right => (1, 0).into(),
            Dir::Down => (0, 1).into(),
            Dir::Left => (-1, 0).into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Motion {
    len: usize,
    dir: Dir,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Edge(Pos, Pos);

impl Edge {
    pub fn s(&self) -> Pos {
        self.0
    }
    pub fn e(&self) -> Pos {
        self.1
    }
    /// only horizontal & vertical ==> equal to L2 norm lmaooo
    pub fn l1_norm(&self) -> usize {
        let d = self.0 - self.1;
        d.0.unsigned_abs() + d.1.unsigned_abs()
    }
}

/// Produces a closed-loop of edges given the motion list. It always outputs the OUTSIDE PERIMETER
/// of the shape produced by digging.
/// Internally, it calculates both the inner and outside one, since it does not know which is which
/// until completion of the algoritm.
/// Edges are in order, but they _do not_ necessarily correspond to the motion at the same index.
fn produce_edges(motions: &[Motion]) -> Vec<Edge> {
    let m = |idx| motions[idx % motions.len()];

    // Pair of edges resulting from `m` at grid position grid_pos (not pos of edge, but of the
    // digger).
    let edge_pair = |grid_pos: Pos, m_pre: Motion, m: Motion, m_next: Motion| {
        let (start_left, start_right) = match m.dir {
            Dir::Up => (grid_pos, grid_pos + (1, 0)),
            Dir::Right => (grid_pos + (1, 0), grid_pos + (1, 1)),
            Dir::Down => (grid_pos + (1, 1), grid_pos + (0, 1)),
            Dir::Left => (grid_pos + (0, 1), grid_pos),
        };
        let (end_left, end_right) = (
            start_left + m.dir.dir_step() * (m.len - 1) as isize,
            start_right + m.dir.dir_step() * (m.len - 1) as isize,
        );

        // println!("\tsl = {start_left:?} sr = {start_right:?}");

        let (start_left, start_right) = match () {
            _ if m_pre.dir.is_left_turn(m.dir) => (start_left, start_right - m.dir.dir_step()),
            _ if m_pre.dir.is_right_turn(m.dir) => (start_left - m.dir.dir_step(), start_right),
            _ => (start_left, start_right),
        };
        // println!("\tsl = {start_left:?} sr = {start_right:?}");

        let (end_left, end_right) = match () {
            _ if m.dir.is_left_turn(m_next.dir) => (end_left, end_right + m.dir.dir_step()),
            _ if m.dir.is_right_turn(m_next.dir) => (end_left + m.dir.dir_step(), end_right),
            _ => (end_left, end_right),
        };

        (Edge(start_left, end_left), Edge(start_right, end_right))
    };

    let mut edges_left: Vec<Edge> = vec![];
    let mut edges_right: Vec<Edge> = vec![];
    let mut pos = (0, 0).into(); // initial pos of the digger at m

    // multiple cases, with right/left. I don't think the move can go straight, I will assert for
    // memes
    for i in 0..motions.len() {
        let a = m(i);
        let b = m(i + 1);
        let c = m(i + 2);

        assert_ne!(a.dir, b.dir);
        assert_ne!(c.dir, b.dir);

        // println!("{a:?}-{b:?}-{c:?}");
        let (el, er) = edge_pair(pos, a, b, c);
        // println!("el = {el:?}, er = {er:?}");
        // println!("==================================");
        if i != 0 {
            assert_eq!(edges_left.last().unwrap().1, el.0);
            assert_eq!(edges_right.last().unwrap().1, er.0);
        }

        edges_left.push(el);
        edges_right.push(er);

        pos = pos + b.dir.dir_step() * b.len as isize;
    }

    assert_eq!(edges_left[0].0, edges_left.last().unwrap().1);
    assert_eq!(edges_right[0].0, edges_right.last().unwrap().1);

    print_edges(&edges_left, &edges_right).unwrap();

    // get the max perimeter edge loop (it's the outside one)

    [edges_left, edges_right]
        .into_iter()
        .max_by_key(|ed| ed.iter().map(Edge::l1_norm).sum::<usize>())
        .unwrap()
}

#[allow(unused)]
fn print_edges(edges_1: &[Edge], edges_2: &[Edge]) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "plotters")]
    {
        use plotters::prelude::*;
        let path = format!(
            "./inputs/p18/edges-{}-{}.png",
            edges_1.iter().map(Edge::l1_norm).sum::<usize>(),
            edges_2.iter().map(Edge::l1_norm).sum::<usize>(),
        );
        let root = BitMapBackend::new(&path, (400, 400)).into_drawing_area();

        root.fill(&WHITE)?;

        let min_x = edges_1.iter().map(|e| e.s().0).min().unwrap();
        let max_x = edges_1.iter().map(|e| e.s().0).max().unwrap();
        let max_y = -edges_1.iter().map(|e| e.s().1).min().unwrap();
        let min_y = -edges_1.iter().map(|e| e.s().1).max().unwrap();
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(10)
            .y_label_area_size(10)
            .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        let plotting_area = chart.plotting_area();

        plotting_area.draw(&PathElement::new(
            edges_1
                .iter()
                .chain(Some(&edges_1[0]))
                .map(|e| (e.s().0, -e.s().1))
                .collect::<Vec<_>>(),
            RED.mix(0.5),
        ))?;
        plotting_area.draw(&PathElement::new(
            edges_2
                .iter()
                .chain(Some(&edges_2[0]))
                .map(|e| (e.s().0, -e.s().1))
                .collect::<Vec<_>>(),
            GREEN.mix(0.5),
        ))?;
        root.present()?;
    }
    Ok(())
}

fn compute_area(edges: &[Edge]) -> i64 {
    // inspired by https://en.wikipedia.org/wiki/Shoelace_formula
    // We only have vertical and horizontal edges, literally just consider the horizontal ones and
    // area formula is easy af. Can also consider vertical ones, but their resulting area is zero,
    // so...

    // assume y is equal for both points of the edge. If not horizontal, then x part
    // is zero anyway, does not matter
    let area = edges
        .iter()
        .map(|edge| (edge.s().0 - edge.e().0) as i64 * (edge.s().1) as i64)
        .sum();

    area
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("inputs/p18/input.txt").expect("reading input");

    // Well, part1's algorithm is doomed (part2 requires a complete rewrite).
    // New approach:
    // 1. Produce _edges_ on the outside of the shape (until completion, we don't know
    // which part is the inside). To do so, consider 3 consecutive motions, which allows us to
    // construct the inside and outside edges (we don't know which is which) WITHOUT overlapping
    // the next ones (considering 3 motions at a time gives us this ability).
    // 2. Figure out which one is the longest (should be the one outside I hope lmao), discard the
    //    other.
    // 3. Use a formula inspired from the Shoelace formula (simpler, we only have to consider
    //    horizontal lines for the area...) to compute the area. GG, we are done.
    //
    let (motion_1, motion_2): (Vec<_>, Vec<_>) = content.lines().map(parse_line).unzip();
    let edges_1 = produce_edges(&motion_1);
    let edges_2 = produce_edges(&motion_2);

    let area_1 = compute_area(&edges_1);
    let area_2 = compute_area(&edges_2);
    println!("[PART1] Area = {area_1}");
    println!("[PART2] Area = {area_2}");
    Ok(())
}

/// Parses the current line, and returns the pair
/// of different interpretations for part1 and part2, respectively.
/// - Part 1: `D 1 (#color)`, with `D` direction and `L` length, `color` useless.
/// - Part 2: `_ _ (#llllld)`, with `lllll` length (5 hex digits) and `d` direction (1 hex digit).
fn parse_line(s: &str) -> (Motion, Motion) {
    let mut elems = s.split_whitespace();
    let dir_1 = match elems.next().unwrap() {
        "D" => Dir::Down,
        "R" => Dir::Right,
        "U" => Dir::Up,
        "L" => Dir::Left,
        d => panic!("invalid direction {d} in input lines"),
    };
    let len_1 = elems.next().unwrap().parse().expect("parsing line length");

    let color = usize::from_str_radix(elems.next().unwrap().trim_matches(&['(', ')', '#']), 16)
        .expect("parsing color");
    let dir_2 = [Dir::Right, Dir::Down, Dir::Left, Dir::Up][color & 0b1111];
    let len_2 = color >> 4;

    (
        Motion {
            dir: dir_1,
            len: len_1,
        },
        Motion {
            dir: dir_2,
            len: len_2,
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::Pos;

    #[test]
    fn pos_ops() {
        let pos = Pos(0, 0);
        assert_eq!(pos * 1, pos);
        assert_eq!(pos * 2, pos);
        let pos = pos + (0, 1);
        assert_eq!(pos * 1, pos);
        assert_eq!(pos * 2, pos + pos);
        assert_eq!(pos * 2, (0, 2).into());
    }
}
