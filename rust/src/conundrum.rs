use std::fs::File;
use std::io::{self, BufRead};

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn valid_game(description: &str, power: bool) -> i32 {
    let description = description.trim();
    let id_index = description.find(':').unwrap();
    let id = description[4..id_index].trim().parse::<i32>().unwrap();
    let game: Vec<Vec<&str>> = description[id_index+2..]
        .split(';')
        .map(|subgame| subgame.split(',').collect())
        .collect();

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for subgame in &game {
        for color in subgame {
            let trimmed = color.trim();
            let space_index = trimmed.find(' ').unwrap();
            let color_value = trimmed[..space_index].parse::<i32>().unwrap();
            match trimmed.chars().last().unwrap() {
                'd' => {
                    if !power && color_value > MAX_RED { return 0; }
                    if color_value > max_red { max_red = color_value; } 
                }
                'n' => {
                    if !power && color_value > MAX_GREEN { return 0; }
                    if color_value > max_green { max_green = color_value; } 
                }
                'e' => {
                    if !power && color_value > MAX_BLUE { return 0; }
                    if color_value > max_blue { max_blue = color_value; } 
                }
                _ => (),
            }
        }
    }

    if power { return max_red * max_green * max_blue; } 
    return id;
}

pub fn solve(input_directory: &str, file_name: &str, power: bool) -> io::Result<()> {
    let mut valid_sum = 0;

    let file_path = format!("{}{}", input_directory, file_name);
    let file = File::open(file_path)?;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        valid_sum += valid_game(&line, power);
    }

    if power {
        println!("task 2: {}", valid_sum);
    } else {
        println!("task 1: {}", valid_sum);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // USE FROM ADVENT
    const GAME1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    const GAME2: &str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    const GAME3: &str = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
    const GAME4: &str = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
    const GAME5: &str = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_valid_game_id() {
        assert_eq!(valid_game(GAME1, false), 1);
        assert_eq!(valid_game(GAME2, false), 2);
        assert_eq!(valid_game(GAME3, false), 0);
        assert_eq!(valid_game(GAME4, false), 0);
        assert_eq!(valid_game(GAME5, false), 5);
    }

    #[test]
    fn test_valid_game_power() {
        assert_eq!(valid_game(GAME1, true), 48);
        assert_eq!(valid_game(GAME2, true), 12);
        assert_eq!(valid_game(GAME3, true), 1560);
        assert_eq!(valid_game(GAME4, true), 630);
        assert_eq!(valid_game(GAME5, true), 36);
    }
}