use std::fs::File;
use std::io::{self, BufRead};

fn get_first_and_last_digit(sequence: &str, alpha: bool) -> i32 {
    let digit_mapping = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut sequence = sequence.to_string();
    sequence.push_str("    "); // pad out

    let mut first_digit = None;
    let mut last_digit = None;

    for (i, char) in sequence.chars().enumerate() {
        if char.is_numeric() {
            if first_digit.is_none() {
                first_digit = Some(char);
            }
            last_digit = Some(char);
        } else if char.is_alphabetic() && alpha {
            for (key, value) in &digit_mapping {
                if char == key.chars().next().unwrap() && sequence[i..i+key.len()] == **key {
                    if first_digit.is_none() {
                        first_digit = Some(value.chars().next().unwrap());
                    }
                    last_digit = Some(value.chars().next().unwrap());
                    break;
                }
            }
        }
    }

    let result = format!("{}{}", first_digit.unwrap(), last_digit.unwrap());
    result.parse::<i32>().unwrap()
}

pub fn solve(input_directory: &str, file_name: &str, alpha: bool) -> io::Result<()> {

    let mut calibrations = 0;
    let file_path = format!("{}{}", input_directory, file_name);
    let file = File::open(file_path)?;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        calibrations += get_first_and_last_digit(line.trim(), alpha);
    }

    if alpha {
        println!("task 2: {}", calibrations);
    } else {
        println!("task 1: {}", calibrations);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_and_last_digit_numeric() {
        let sequence = "12345";
        let alpha = false;
        let result = get_first_and_last_digit(sequence, alpha);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_get_first_and_last_digit_alpha() {
        let sequence = "one two three four five";
        let alpha = true;
        let result = get_first_and_last_digit(sequence, alpha);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_get_first_and_last_digit_mixed() {
        let sequence = "1 two 3 four five";
        let alpha = true;
        let result = get_first_and_last_digit(sequence, alpha);
        assert_eq!(result, 15);
    }
}