use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pos(usize, usize);

impl Pos {
    /// Translates the current position by the given direction, if it fits in the grid
    fn translate(self, dir: Direction, width: usize, height: usize) -> Option<Self> {
        match dir {
            Direction::Up => Pos(self.0, self.1.checked_sub(1)?),
            Direction::Down => Pos(self.0, self.1 + 1),
            Direction::Left => Pos(self.0.checked_sub(1)?, self.1),
            Direction::Right => Pos(self.0 + 1, self.1),
        }
        .fit(width, height)
    }

    /// Returns Some(self) if it fits in the grid
    fn fit(self, width: usize, height: usize) -> Option<Self> {
        (self.0 < width && self.1 < height).then_some(self)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrorUR,
    MirrorDR,
    SplitterHorizontal,
    SplitterVertical,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;
        match value {
            '.' => Empty,
            '/' => MirrorUR,
            '\\' => MirrorDR,
            '|' => SplitterVertical,
            '-' => SplitterHorizontal,
            o => panic!("Invalid tile character: {o:?}"),
        }
    }
}

fn count_energized_tiles(grid: &[Tile], width: usize, start: Pos, dir: Direction) -> usize {
    use Direction::*;
    use Tile::*;
    let height = grid.len() / width;

    // On any edge/corner
    assert!(start.0 == 0 || start.0 == width - 1 || start.1 == 0 || start.1 == height - 1);
    // Top-left corner
    assert!(!(start.1 == 0 && start.0 == 0) || (dir == Down || dir == Right));
    // Top-right corner
    assert!(!(start.1 == 0 && start.0 == width - 1) || (dir == Down || dir == Left));
    // Bottom-left corner
    assert!(!(start.1 == height - 1 && start.0 == 0) || (dir == Up || dir == Right));
    // Bottom-right corner
    assert!(!(start.1 == height - 1 && start.0 == width - 1) || (dir == Up || dir == Left));
    // Top edge
    assert!(!(start.1 == 0 && start.0 > 0 && start.0 < width - 1) || dir == Down);
    // Bottom edge
    assert!(!(start.1 == height - 1 && start.0 > 0 && start.0 < width - 1) || dir == Up);
    // Left edge
    assert!(!(start.0 == 0 && start.1 > 0 && start.1 < height - 1) || dir == Right);
    // Right edge
    assert!(!(start.0 == width - 1 && start.1 > 0 && start.1 < height - 1) || dir == Left);

    fn recurse(
        grid: &[Tile],
        visited: &mut [u8],
        width: usize,
        pos: Pos,
        dir: Direction,
    ) -> Option<()> {
        let height = grid.len() / width;

        if pos.0 >= width || pos.1 >= height {
            return None; // out of bounds, do nothing
        }

        let idx = width * pos.1 + pos.0;

        if 0 != visited[idx] & dir as u8 {
            return None; // already went through this tile this was, nothing to be done. Other
                         // directions might lead to other paths though, thus the check
        }

        visited[idx] |= dir as u8;

        let _ = match (grid[idx], dir) {
            (MirrorDR, Left) | (MirrorUR, Right) => {
                recurse(grid, visited, width, pos.translate(Up, width, height)?, Up)
            }
            (MirrorDR, Right) | (MirrorUR, Left) => recurse(
                grid,
                visited,
                width,
                pos.translate(Down, width, height)?,
                Down,
            ),
            (MirrorDR, Up) | (MirrorUR, Down) => recurse(
                grid,
                visited,
                width,
                pos.translate(Left, width, height)?,
                Left,
            ),
            (MirrorDR, Down) | (MirrorUR, Up) => recurse(
                grid,
                visited,
                width,
                pos.translate(Right, width, height)?,
                Right,
            ),
            (SplitterHorizontal, Up) | (SplitterHorizontal, Down) => {
                if let Some(p) = pos.translate(Right, width, height) {
                    recurse(grid, visited, width, p, Right);
                }
                if let Some(p) = pos.translate(Left, width, height) {
                    recurse(grid, visited, width, p, Left);
                }
                Some(())
            }
            (SplitterVertical, Left) | (SplitterVertical, Right) => {
                if let Some(p) = pos.translate(Up, width, height) {
                    recurse(grid, visited, width, p, Up);
                }
                if let Some(p) = pos.translate(Down, width, height) {
                    recurse(grid, visited, width, p, Down);
                }
                Some(())
            }
            (_, d) => recurse(grid, visited, width, pos.translate(d, width, height)?, d),
        };

        Some(())
    }

    let mut visited = vec![0; grid.len()];

    recurse(grid, &mut visited, width, start, dir);
    visited.iter().filter(move |&&b| b != 0).count()
}

fn main() {
    use Direction::*;
    let content = fs::read_to_string("inputs/p16/tiles.txt").expect("reading problem input");

    let width = content.find('\n').unwrap(); // line length
    let grid: Vec<Tile> = content
        .lines()
        .flat_map(|l| l.chars().map(|c| c.into()))
        .collect();
    let height = grid.len() / width;

    // Start at top-left corner, going to the right. Then walk the path, splitting where required.
    // Keep a "visited" bitmap to avoid running into loops.

    let part1 = count_energized_tiles(&grid, width, Pos(0, 0), Direction::Right);
    println!("[PART 1] Energized tiles: {part1}");

    // find entry point & direction with max coverage
    let entries = (0..width)
        .map(|x| (Pos(x, 0), Down))
        .chain((0..width).map(|x| (Pos(x, height - 1), Up)))
        .chain((0..height).map(|y| (Pos(0, y), Right)))
        .chain((0..height).map(|y| (Pos(width - 1, y), Left)));
    let part2 = entries
        .map(|(pos, dir)| count_energized_tiles(&grid, width, pos, dir))
        .max()
        .unwrap();
    println!("[PART 2] Maximum energized tiles: {part2}");
}
