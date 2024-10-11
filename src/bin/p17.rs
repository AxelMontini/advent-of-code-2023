use std::{collections::VecDeque, fs, ops::RangeInclusive};

use Direction::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right = 0,
    Up = 1,
    Left = 2,
    Down = 3,
}

impl Direction {
    fn translate(
        self,
        moves: usize,
        (x, y): (usize, usize),
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        assert!((1..=3).contains(&moves));
        match self {
            Right => (x + 1 < width).then_some((x + 1, y)),
            Up => y.checked_sub(1).map(|y| (x, y)),
            Left => x.checked_sub(1).map(|x| (x, y)),
            Down => (y + 1 < height).then_some((x, y + 1)),
        }
    }
}

fn main() {
    // Can move at most 3 times in one direction.
    // Map is heat loss per cell.
    // From top-left to bottom-right corner, find valid path
    // that minimizes heat loss.
    let content = fs::read_to_string("inputs/p17/map.txt").expect("reading input file");
    let width = content.lines().next().unwrap().len();
    let grid: Vec<u8> = content
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| c.to_digit(10).unwrap() as u8) // you better be valid..
        .collect();
    let height = grid.len() / width;

    let part1 = it_is_horrible(&grid, width, height, 1..=3);
    println!("[PART 1] Min cost to reach destination: {part1}");
    let part2 = it_is_horrible(&grid, width, height, 4..=10);
    println!("[PART 2] Min cost to reach destination: {part2}");

    // Now literally same problem but with different constraints.... (has to move at least 4 blocks
    // straight, at most 10)
    // Too lazy to make the solution generic, so here we go...
}

/// Takes the grid and its dimension. Also it takes a range of moves, where the minimum
fn it_is_horrible(
    grid: &[u8],
    width: usize,
    height: usize,
    can_move: RangeInclusive<usize>,
) -> u32 {
    // now, the thing is..
    // I need to keep track of the cost of reaching each tile. Also, i need to know what move
    // (direction) led to this result. The algorithm is: on a tile, that has been reached by moving
    // in a direction, move in all other possible directions, for all possible moves. If the
    // newly-reached tile has improved (lower cost), then re-add it to the queue for recomputation.

    let idx = |x: usize, y: usize, dir: Direction| {
        x + y * width + (dir as u8 as usize) * (width * height)
    };
    let mut dp = vec![u32::MAX; grid.len() * 4];
    [Right, Down]
        .into_iter()
        .for_each(|dir| dp[idx(0, 0, dir)] = grid[0] as _);

    let mut q = VecDeque::new();
    q.push_back((0, 0, Right));
    q.push_back((0, 0, Down));

    // For each (x, y, moved_dir) cell, move in all other directions compared to moved_dir and
    // compute the costs.
    // If a cell has lower cost than before, add it to the queue (needs to be recomputed).
    // There are no cycles, so All Good (tm)

    while let Some((x, y, dir)) = q.pop_front() {
        let dirs = [Right, Up, Down, Left]
            .into_iter()
            .filter(move |&d| d != dir);

        for d in dirs {
            let mut pos = (x, y);
            let mut cost = dp[idx(x, y, dir)];
            for m in 1..=*can_move.end() {
                if let Some(new_pos) = d.translate(1, pos, width, height) {
                    cost = cost + grid[new_pos.0 + new_pos.1 * width] as u32;
                    // move at least X distance before turning, see part 2
                    if can_move.contains(&m) {
                        if dp[idx(new_pos.0, new_pos.1, d)] > cost {
                            dp[idx(new_pos.0, new_pos.1, d)] = cost;
                            q.push_back((new_pos.0, new_pos.1, d));
                        }
                    }
                    pos = new_pos;
                } else {
                    break;
                }
            }
        }
    }

    [Down, Right]
        .into_iter()
        .map(|d| dp[idx(width - 1, height - 1, d)])
        .min()
        .unwrap()
}
