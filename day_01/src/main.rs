use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "input.txt";
    let sum = process_file(path)?;
    println!("La suma de todos los valores de calibración es: {}", sum);
    Ok(())
}

fn process_file<P>(filename: P) -> io::Result<i32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        sum += calculate_calibration_value(&line);
    }

    Ok(sum)
}

fn calculate_calibration_value(line: &str) -> i32 {
    let bytes = line.as_bytes();
    let first_digit = bytes.iter().find(|&&c| c.is_ascii_digit());
    let last_digit = bytes.iter().rev().find(|&&c| c.is_ascii_digit());

    match (first_digit, last_digit) {
        (Some(&first), Some(&last)) => {
            let value = format!("{}{}", first as char, last as char);
            value.parse::<i32>().unwrap_or(0)
        }
        _ => 0,
    }
}
