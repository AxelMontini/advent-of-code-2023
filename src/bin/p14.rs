use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Fixed,
    Rolling,
    Empty,
}

const _: () = assert!(size_of::<Cell>() == 1);

type Field = [Cell];

fn main() {
    let content = fs::read_to_string("inputs/p14/stones.txt").expect("reading input file");
    let width = content.lines().next().unwrap().len();
    let mut field: Vec<_> = content
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '.' => Cell::Empty,
                'O' => Cell::Rolling,
                '#' => Cell::Fixed,
                _ => panic!("Invalid input character '{c}'"),
            })
        })
        .collect();
    let height = field.len() / width;

    move_north(&mut field, width, height);
    let part1 = compute_load(&field, width, height);

    println!("[PART 1] Moving north causes load {part1}");

    // For part2, looks like I actually have to move the stones...
    // I will implement an Iterator-based way to move stones in a certain line, towards the start.
    // Then, hopefully, the thing becomes cyclic so I don't have to simulate that many steps...
    let cycles = 1000000000;

    for i in 0..cycles {
        move_west(&mut field, width, height);
        move_south(&mut field, width, height);
        move_east(&mut field, width, height);
        move_north(&mut field, width, height);
        let load = compute_load(&field, width, height);

        if i % 1000 == 0 {
            println!("Load {i}: {load}");
        }
    }
}

fn compute_load(field: &Field, width: usize, height: usize) -> usize {
    let mut load = 0;
    for col in 0..width {
        for row in 0..height {
            match field[row * width + col] {
                Cell::Rolling => load += height - row,
                _ => (),
            }
        }
    }

    load
}

fn move_south(field: &mut Field, width: usize, height: usize) {
    for col in 0..width {
        let new_line = move_line((0..height).rev().map(|row| field[row * width + col]));
        new_line
            .into_iter()
            .rev()
            .enumerate()
            .for_each(|(row, c)| field[row * width + col] = c);
    }
}

fn move_north(field: &mut Field, width: usize, height: usize) {
    for col in 0..width {
        let new_line = move_line((0..height).map(|row| field[row * width + col]));
        new_line
            .into_iter()
            .enumerate()
            .for_each(|(row, c)| field[row * width + col] = c);
    }
}

fn move_west(field: &mut Field, width: usize, height: usize) {
    for row in 0..height {
        let new_line = move_line((0..width).map(|col| field[row * width + col]));
        new_line
            .into_iter()
            .enumerate()
            .for_each(|(col, c)| field[row * width + col] = c);
    }
}

fn move_east(field: &mut Field, width: usize, height: usize) {
    for row in 0..height {
        let new_line = move_line((0..width).rev().map(|col| field[row * width + col]));
        new_line
            .into_iter()
            .rev()
            .enumerate()
            .for_each(|(col, c)| field[row * width + col] = c);
    }
}

fn move_line<I: Iterator<Item = Cell> + ExactSizeIterator>(mut line: I) -> Vec<Cell> {
    let mut rolling_stones = 0;
    let mut empty_stones = 0;
    let len = line.len();
    let mut out = vec![];

    for _ in 0..len {
        match line.next().unwrap() {
            Cell::Rolling => rolling_stones += 1,
            Cell::Fixed => {
                (0..rolling_stones).for_each(|_| out.push(Cell::Rolling));
                (0..empty_stones).for_each(|_| out.push(Cell::Empty));
                empty_stones = 0;
                rolling_stones = 0;
                out.push(Cell::Fixed);
            }
            Cell::Empty => empty_stones += 1,
        }
    }

    (0..rolling_stones).for_each(|_| out.push(Cell::Rolling));
    (0..empty_stones).for_each(|_| out.push(Cell::Empty));

    out
}

#[cfg(test)]
mod tests {
    use crate::{move_east, move_line, move_north, move_south, move_west, Cell, Field};

    #[test]
    fn move_line_easy() {
        let values = [(
            &[
                Cell::Rolling,
                Cell::Empty,
                Cell::Rolling,
                Cell::Fixed,
                Cell::Empty,
                Cell::Rolling,
                Cell::Rolling,
            ][..],
            &[
                Cell::Rolling,
                Cell::Rolling,
                Cell::Empty,
                Cell::Fixed,
                Cell::Rolling,
                Cell::Rolling,
                Cell::Empty,
            ][..],
        )];

        for (input, expected) in values {
            assert_eq!(move_line(input.into_iter().copied()), expected);
        }
    }

    #[test]
    fn moves() {
        let field = &mut [
            Cell::Rolling,
            Cell::Rolling,
            Cell::Rolling,
            Cell::Empty,
            Cell::Fixed,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ];

        move_north(field, 3, 3);
        let expected = &[
            Cell::Rolling,
            Cell::Rolling,
            Cell::Rolling,
            Cell::Empty,
            Cell::Fixed,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ];
        assert_eq!(expected, field);

        move_west(field, 3, 3);
        let expected = &[
            Cell::Rolling,
            Cell::Rolling,
            Cell::Rolling,
            Cell::Empty,
            Cell::Fixed,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ];
        assert_eq!(expected, field);

        move_east(field, 3, 3);
        let expected = &[
            Cell::Rolling,
            Cell::Rolling,
            Cell::Rolling,
            Cell::Empty,
            Cell::Fixed,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
        ];
        assert_eq!(expected, field);

        move_south(field, 3, 3);
        let expected = &[
            Cell::Empty,
            Cell::Rolling,
            Cell::Empty,
            Cell::Empty,
            Cell::Fixed,
            Cell::Empty,
            Cell::Rolling,
            Cell::Empty,
            Cell::Rolling,
        ];
        assert_eq!(expected, field);

        move_east(field, 3, 3);
        let expected = &[
            Cell::Empty,
            Cell::Empty,
            Cell::Rolling,
            Cell::Empty,
            Cell::Fixed,
            Cell::Empty,
            Cell::Empty,
            Cell::Rolling,
            Cell::Rolling,
        ];
        assert_eq!(expected, field);

        move_west(field, 3, 3);
        let expected = &[
            Cell::Rolling,
            Cell::Empty,
            Cell::Empty,
            Cell::Empty,
            Cell::Fixed,
            Cell::Empty,
            Cell::Rolling,
            Cell::Rolling,
            Cell::Empty,
        ];
        assert_eq!(expected, field);
    }
}