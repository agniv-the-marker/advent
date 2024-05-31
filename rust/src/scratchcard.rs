use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn process_card(card: &str) -> (i32, i32) {
    let card = &card[card.find(':').unwrap()+2..];
    let parts: Vec<&str> = card.split('|').collect();
    let winning: HashSet<i32> = parts[0].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let numbers: Vec<i32> = parts[1].split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut score = 0;
    for num in numbers {
        if winning.contains(&num) {
            score += 1;
        }
    }
    
    if score == 0 { return (0, 0); }
    (2i32.pow((score - 1) as u32), score)
}

pub fn solve(input_directory: &str, file_name: &str) -> io::Result<()> {
    let mut total_points = 0;
    let mut total_cards = 0;

    let mut multiples: HashMap<usize, i32> = HashMap::new();

    let file_path = format!("{}{}", input_directory, file_name);
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for (ind, line) in reader.lines().enumerate() {
        let line = line?;

        if ind > 0 && multiples.contains_key(&(ind-1)) {
            multiples.remove(&(ind-1));
        }
        *multiples.entry(ind).or_default() += 1;
        total_cards += multiples[&ind];

        let (row_points, row_score) = process_card(line.trim());
        total_points += row_points;
        for next_card in 0..row_score {
            *multiples.entry(ind + 1 + next_card as usize).or_default() += multiples[&ind];
        }
    }
    println!("task 1: {}", total_points);
    println!("task 2: {}", total_cards);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CARD1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    const CARD2: &str = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
    const CARD3: &str = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
    const CARD4: &str = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
    const CARD5: &str = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
    const CARD6: &str = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_process_card() {
        assert_eq!(process_card(CARD1), (8, 4));
        assert_eq!(process_card(CARD2), (2, 2));
        assert_eq!(process_card(CARD3), (2, 2));
        assert_eq!(process_card(CARD4), (1, 1));
        assert_eq!(process_card(CARD5), (0, 0));
        assert_eq!(process_card(CARD6), (0, 0));
    }
}