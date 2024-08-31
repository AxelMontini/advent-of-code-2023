use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "bin/p2.pest"]
struct GameParser;

fn main() {
    let content = fs::read_to_string("inputs/p2/games.txt").expect("reading file");

    let file = GameParser::parse(Rule::file, &content)
        .expect("parsing file")
        .next()
        .unwrap();
    let mut games = vec![];
    for game in file.into_inner() {
        if game.as_rule() != Rule::game {
            break;
        }

        let mut game_iter = game.into_inner();
        let game_id: u32 = game_iter.next().unwrap().as_str().parse().unwrap();
        let draws: Vec<[u32; 3]> = game_iter
            .map(|draw| {
                let mut draw_count = [0, 0, 0]; // r,g,b
                for colorcount in draw.into_inner() {
                    let mut colorcount_iter = colorcount.into_inner();
                    let count: u32 = colorcount_iter.next().unwrap().as_str().parse().unwrap();
                    let color = colorcount_iter.next().unwrap().as_str();
                    match color {
                        "red" => draw_count[0] = count,
                        "green" => draw_count[1] = count,
                        "blue" => draw_count[2] = count,
                        _ => unreachable!(),
                    }
                }
                draw_count
            })
            .collect();
        games.push((game_id, draws));
    }

    let pp = [(12, 13, 14)];

    for (r, g, b) in pp {
        println!(
            "ID-sum of possible games with r={r} g={g} b={b}: {}",
            possible_games(&games, r, g, b).sum::<u32>()
        );
    }


    // Find sum of power for all games
    let power_sum: u32 = games.iter().map(|(_game_id, draws)| min_power(draws)).sum();
    println!("Power sum of all games: {power_sum}");
}

type GameId = u32;

/// Min power of the set for this game
fn min_power(draws: &[[u32; 3]]) -> u32 {
    draws
        .iter()
        .fold([0, 0, 0], |[r1, g1, b1], [r2, g2, b2]| {
            [r1.max(*r2), g1.max(*g2), b1.max(*b2)]
        })
        .iter()
        .product()
}

/// Returns an iterator on the IDs of the games that are possible, given the min (r,g,b) count of
/// cubes in the bag
fn possible_games(
    games: &[(u32, Vec<[u32; 3]>)],
    r: u32,
    g: u32,
    b: u32,
) -> impl Iterator<Item = GameId> + '_ {
    games.iter().filter_map(move |(id, draws)| {
        if draws.iter().all(|d| d[0] <= r && d[1] <= g && d[2] <= b) {
            Some(*id)
        } else {
            None
        }
    })
}
