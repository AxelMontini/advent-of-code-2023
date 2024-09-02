use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/p4/scratchcards.txt").expect("reading problem input");
    let card_matches: Vec<_> = content
        .lines()
        .map(|line| {
            let mut line_nums = line
                .split(':')
                .skip(1)
                .flat_map(|rest| rest.split('|'))
                .flat_map(|nums| nums.split_whitespace())
                .map(|n| u32::from_str_radix(n, 10))
                .collect::<Result<Vec<_>, _>>()
                .expect("parsing number in file");
            line_nums.sort();
            let matches = line_nums
                .windows(2)
                .filter(|pair| pair[0] == pair[1])
                .count();
            matches
        })
        .collect();

    let score: u32 = card_matches
        .iter()
        .map(move |&matches| 2u32.pow(matches as u32) / 2)
        .sum();

    println!("Score: {score}");

    // now handle duplication of cards
    // For each card appearing `n` times containing `m` matches, increment the subsequent `m` cards by `n` each.
    let cards_len = content.lines().count();
    let mut card_count = vec![1u64; cards_len];

    for (card_idx, m) in card_matches.into_iter().enumerate() {
        let n = card_count[card_idx];
        if let Some(to_increment) =
            card_count.get_mut((card_idx + 1)..cards_len.min(card_idx + 1 + m as usize))
        {
            to_increment.iter_mut().for_each(|count| *count += n);
        }
    }

    println!("Total cards: {}", card_count.iter().sum::<u64>());
}
