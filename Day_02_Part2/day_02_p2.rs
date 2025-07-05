use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn verify_report(levels: &[i32]) -> bool {
    let diffs: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();

    let all_increasing = diffs.iter().all(|&d| d > 0 && d <= 3);
    let all_decreasing = diffs.iter().all(|&d| d < 0 && d >= -3);

    all_increasing || all_decreasing
}

fn is_safe(report: &str) -> bool {
    let mut levels: Vec<i32> = report
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let result = verify_report(&levels);

    if (result) {
        return true;
    } else {
        
        for i in 0..levels.len() {
            let mut temp_levels = levels.clone();
            temp_levels.remove(i);
            if verify_report(&temp_levels) {
                return true;
            }
        }
    }
    
    result
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
