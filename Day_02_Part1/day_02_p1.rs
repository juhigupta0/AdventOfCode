use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn is_safe(report: &str) -> bool {
    let levels: Vec<i32> = report
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if levels.len() < 2 {
        return false;
    }

    let diffs: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();

    let all_increasing = diffs.iter().all(|&d| d > 0 && d <= 3);
    let all_decreasing = diffs.iter().all(|&d| d < 0 && d >= -3);

    all_increasing || all_decreasing
}

fn main() -> io::Result<()> {
    let input_file_path = "input.txt";
    let file = File::open(input_file_path)?;
    let reader = BufReader::new(file);

    let safe_count = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| is_safe(line))
        .count();

    println!("safe_count: {}", safe_count);

    Ok(())
}
