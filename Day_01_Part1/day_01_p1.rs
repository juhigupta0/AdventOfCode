use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let filename = "input.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut vecLeft = Vec::new();
    let mut vecRight = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut nums = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());
        if let (Some(left), Some(right)) = (nums.next(), nums.next()) {
            vecLeft.push(left);
            vecRight.push(right);
        }
    }

    vecLeft.sort_unstable();
    vecRight.sort_unstable();

    let sum: i32 = vecLeft
        .iter()
        .zip(vecRight.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("Sum: {}", sum);

    Ok(())
}
