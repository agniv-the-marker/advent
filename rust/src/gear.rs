use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Gear {
    parts: HashMap<(usize, usize), Vec<i32>>,
    ratios: HashMap<(usize, usize), Vec<i32>>,
}

fn process_board(board: &[String]) -> Gear {
    // Would benefit from using multiple functions.

    // Add padding to the board to make it easier to iterate over the board.
    let mut board: Vec<String> = board.iter().map(|line| format!(".{}.", line.trim())).collect();
    let line_len = board[0].len();
    board.insert(0, ".".repeat(line_len));
    board.push(".".repeat(line_len));

    // Find parts and ratios, do it in one pass
    let mut parts: HashMap<(usize, usize), i32> = HashMap::new();
    let mut ratios: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (i, row) in board.iter().enumerate() {
        let mut j = 0;
        let mut length = 0;
        while j < row.len() {
            if row.chars().nth(j).unwrap().is_numeric() {

                // Find the length of the number
                while row.chars().nth(j + length).unwrap().is_numeric() {
                    length += 1;
                }
                let number: i32 = row[j..j + length].parse().unwrap();

                // https://stackoverflow.com/questions/69170796/rust-negative-for-loops
                // We have to add -1 to the range because we are adding a row and column to the board.
                for x in 0..=length+1 {
                    for y in 0..=2 {
                        let part_x = i + y - 1;
                        let part_y = j + x - 1;
                        let char = board[part_x].chars().nth(part_y).unwrap();
                        if char != '.' && !char.is_numeric() {
                            parts.insert((i - 1, j - 1), number);
                        }
                        if char == '*' {
                            ratios.entry((part_x - 1, part_y - 1)).or_default().push(number);
                        }
                    }
                }
                j += length; // skip over the rest of the number
                length = 0;
            } else {
                j += 1;
            }
        }
    }

    Gear {
        parts: parts.into_iter().map(|(k, v)| (k, vec![v])).collect(),
        ratios: ratios.into_iter().filter(|(_, v)| v.len() == 2).collect(),
    }
}

pub fn solve(input_directory: &str, file_name: &str) -> io::Result<()> {
    let mut board = Vec::new();

    let file_path = format!("{}{}", input_directory, file_name);
    let file = File::open(file_path)?;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        board.push(line);
    }

    let data = process_board(&board);

    let part_sum: i32 = data.parts.values().flatten().sum();
    let gear_sum: i32 = data.ratios.values().map(|v| v[0] * v[1]).sum();

    println!("task 1: {}", part_sum);
    println!("task 2: {}", gear_sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Annoyingly cannot do `const BOARD: Vec<String> = vec![...]` because of the `to_string()` method + vec![] macro.
    // For more readable tests opting for this method.

    #[test]
    fn test_parts() {
        let board = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        let data = process_board(&board);
        let parts = data.parts;

        // Test parts
        assert_eq!(parts.len(), 8);
        assert_eq!(parts.get(&(2, 2)).unwrap(), &[35]);
        assert_eq!(parts.get(&(4, 0)).unwrap(), &[617]);
        assert_eq!(parts.get(&(9, 1)).unwrap(), &[664]);
        assert_eq!(parts.get(&(9, 5)).unwrap(), &[598]);
        assert_eq!(parts.get(&(2, 6)).unwrap(), &[633]);
        assert_eq!(parts.get(&(7, 6)).unwrap(), &[755]);
        assert_eq!(parts.get(&(0, 0)).unwrap(), &[467]);
        assert_eq!(parts.get(&(6, 2)).unwrap(), &[592]);
    }

    #[test]
    fn test_ratios() {
        let board = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        let data = process_board(&board);
        let ratios = data.ratios;

        // Test ratios
        assert_eq!(ratios.len(), 2);
        assert_eq!(ratios.get(&(1, 3)).unwrap(), &[467, 35]);
        assert_eq!(ratios.get(&(8, 5)).unwrap(), &[755, 598]);
    }
}