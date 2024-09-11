use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/p6/races.txt").expect("reading input file");
    let mut lines = content.lines();

    // of course this parsing logic is needed for the small input I've got.............
    // kappachungus maximus deluxe
    let times: Vec<u64> = lines
        .next()
        .expect("getting times line")
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse())
        .collect::<Result<_, _>>()
        .expect("parsing race time");
    let distances: Vec<u64> = lines
        .next()
        .expect("getting distances line")
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse())
        .collect::<Result<_, _>>()
        .expect("parsing race distance");
    // for every millisecond the button is held, the boat's speed increases by 1m/s (mm/ms)
    // Given the time allowed for each race, and the current record, find all the solutions that
    // beat the record (for each race). Note that v = t_held, thus I write speed = held

    // Solve distance < (time - held) * held
    // <==> held^2 - time*held + distance < 0
    // and we shillin
    //
    // If we were to solve for ... = 0 instead, we get
    // $$ held = (time +- sqrt(time^2 - 4*distance)) / 2 $$
    // and thus the amount of solutions is just the diff between the two plus 1 (rounding may apply)

    let ways = |time, distance| {
        // no integer sqrt in std........... balls
        let delta = ((time * time - 4 * distance) as f64).sqrt();
        let held1 = (time as f64 + delta) / 2.0;
        let held2 = (time as f64 - delta) / 2.0;

        (held1.floor() - held2.ceil()) as u64 + 1
    };

    let ways_product: u64 = times
        .into_iter()
        .zip(distances)
        .enumerate()
        .map(|(game_id, (time, distance))| (game_id, ways(time, distance)))
        .inspect(|(game_id, ways)| {
            println!("Game {game_id} allows for {ways} ways to beat the record.")
        })
        .map(|a| a.1)
        .product();

    println!("[Part 1] Product of ways: {ways_product}");

    // PART 2: Now we get to re-parse... Yay!!

    let mut lines = content.lines();
    // lmao the functional hell
    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();
    let distance = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();

    println!("Time {time:?}, distance {distance:?}");
    let time = u64::from_str_radix(&time, 10).unwrap();
    let distance = u64::from_str_radix(&distance, 10).unwrap();
    println!("[Part 2] Ways: {}", ways(time, distance));
}
