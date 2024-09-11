use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("inputs/p8/map.txt").expect("reading the input");

    let mut lines = content.lines();
    let instructions = lines.next().expect("reading instructions line");
    lines.next(); // skip empty line before list of nodes

    let adjacency_list: HashMap<&str, (&str, &str)> = lines
        .map(|line| {
            let (start, end) = line.split_once('=').unwrap();
            let parent = start.trim();
            let (left, right) = end.split_once(',').unwrap();
            let child_left = left.trim_matches(|c: char| !c.is_alphabetic());
            let child_right = right.trim_matches(|c: char| !c.is_alphabetic());
            (parent, (child_left, child_right))
        })
        .collect();

    let mut current = adjacency_list.get("AAA").expect("obtaining start node AAA");
    // follow instructions for part 1
    for (steps, m) in instructions.chars().cycle().enumerate() {
        let next = match m {
            'L' => current.0,
            'R' => current.1,
            _ => panic!("invalid move {m}"),
        };

        current = adjacency_list.get(next).expect("obtaining start node AAA");

        if next == "ZZZ" {
            println!("[PART 1] Found ZZZ in {} steps", steps + 1);
            break;
        }
    }

    // we are a ghost or something now. Start on ALL nodes ending with 'A' and step simultaneously, and
    // stop only when ALL paths reach a node ending with 'Z' together.
    // This takes too long. Instead, I will collect the step # at which we reach a Z node for each
    // ghost, and stop once we have at least 3 each. Hopefully these events are cyclic and we can
    // determine how many cycles we need a lot faster than by simulating everything.
    let mut ghosts: Vec<&str> = adjacency_list
        .keys()
        .filter(|node| node.ends_with('A'))
        .copied()
        .collect();
    println!("Ghosts {ghosts:?}");
    let mut steps_to_z = vec![vec![]; ghosts.len()];

    for (steps, m) in instructions.chars().cycle().enumerate() {
        for (ghost, to_z) in ghosts.iter_mut().zip(steps_to_z.iter_mut()) {
            let prev = *ghost;
            let children = adjacency_list.get(prev).unwrap();
            let next = match m {
                'L' => children.0,
                'R' => children.1,
                _ => panic!("invalid move {m}"),
            };

            *ghost = next;

            if ghost.ends_with('Z') {
                to_z.push(steps as u32 + 1);
            }
        }

        if steps_to_z.iter().all(|to_z| to_z.len() >= 3) {
            println!("Collected enough data for all ghosts");
            break;
        }
    }

    // check if cyclic
    steps_to_z.iter().enumerate().for_each(|(ghost_i, to_z)| {
        assert!(
            to_z.windows(3).all(|t| t[1] - t[0] == t[2] - t[1]),
            "Ghost {ghost_i} does not have a cyclic path to z: {to_z:?}"
        )
    });

    println!(
        "Steps to z: {:?}",
        steps_to_z
            .iter()
            .map(|to_z| &to_z[0..3])
            .collect::<Vec<_>>()
    );
    let interval_to_z = steps_to_z.iter().map(|to_z| (to_z[1] - to_z[0]) as u64);
    println!("{:?}", interval_to_z.clone().collect::<Vec<_>>());
    // Now figure out the LCM of all of these
    let steps_for_ghosts =
        interval_to_z.fold(1u64, |lcm, interval| lcm / gcd(lcm, interval) * interval);

    println!("[PART 2] Steps for all ghosts to reach a Z node: {steps_for_ghosts}");
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let new_b = a % b;
        a = b;
        b = new_b;
    }
    a
}
