use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let input_file_path = "input.txt";
    let file = File::open(input_file_path)?;
    let reader = BufReader::new(file);

    let mut weighted_sum = 0;
    let mut frequency_map = HashMap::new();

    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;

    for line in &lines {
        let mut numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());
        if let (Some(_), Some(right)) = (numbers.next(), numbers.next()) {
            *frequency_map.entry(right).or_insert(0) += 1;
        }
    }

    for line in lines {
        let mut numbers = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());
        if let (Some(left), Some(_)) = (numbers.next(), numbers.next()) {
            if let Some(&count) = frequency_map.get(&left) {
                weighted_sum += left * count;
            }
        }
    }

    println!("Weighted sum: {}", weighted_sum);

    Ok(())
}
