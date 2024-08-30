use std::fs;

fn main() {
    let content = fs::read_to_string("inputs/p1/calibration.txt").unwrap();

    let mut code = 0;
    for l in content.lines() {
        // find first, then find last. Add to code
        for i in 0..l.len() {
            if let Some(digit) = str_to_digit(&l[i..]) {
                code += 10*digit; 
                break;
            }
        }
        for i in (0..l.len()).rev() {
            if let Some(digit) = str_to_digit(&l[i..]) {
                code += digit; 
                break;
            }
        }
    }

    println!("Code (with spelled digits): {code}");
}

fn str_to_digit(s: &str) -> Option<u32> {
    let digit = s.chars().next().and_then(|c| c.to_digit(10)).or_else(|| {
        [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        .find_map(|(i, d)| if s.starts_with(d) { Some(i as u32 + 1) } else { None })
    });

    digit
}
