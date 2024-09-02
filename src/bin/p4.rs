use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/p4/scratchcards.txt").expect("reading problem input");
    let score: u32 = content.lines().map(|line| {
        let mut line_nums = line.split(':').skip(1).flat_map(|rest| rest.split('|'))
            .flat_map(|nums| nums.split_whitespace())
            .map(|n| u32::from_str_radix(n, 10))
            .collect::<Result<Vec<_>, _>>().expect("parsing number in file");
        line_nums.sort();
        let matches = line_nums.windows(2).filter(|pair| pair[0] == pair[1]).count();
        2u32.pow(matches as u32) / 2
    }).sum();
    println!("Score: {score}");
}
