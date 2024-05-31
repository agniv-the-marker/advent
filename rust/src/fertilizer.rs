use std::fs::File;
use std::io::{self, BufRead};

fn short_order(data: Vec<String>) -> i64 { // Broken right now because of faulty data formatting
    let mut orders: Vec<i64> = data[0][7..].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let maps = &data[1..];
    for almanac in maps {
        if almanac.is_empty() || !almanac.chars().next().unwrap().is_numeric() { continue }
        let range: Vec<i64> = almanac.split(' ').map(|x| x.parse().unwrap()).collect();
        let (dest_start, source_start, range_len) = (range[0], range[1], range[2]);
        for order in orders.iter_mut() {
            if *order >= source_start && *order < source_start + range_len {
                *order = dest_start + *order - source_start;
            }
        }
    }
    *orders.iter().min().unwrap()
}

fn range_order(data: Vec<String>) -> i64 {
    let orders: Vec<i64> = data[0][7..].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut inputs: Vec<(i64, i64)> = orders.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    for mapping in &data[1..] {
        let mut new_inputs = Vec::new();
        for (start, length) in inputs {
            let mut start = start;
            let mut length = length;
            while length > 0 {
                let mut found = false;
                for triple in mapping.split('\n').skip(1) {
                    let parts: Vec<i64> = triple.split_whitespace().map(|s| s.parse().unwrap()).collect();
                    let (dest_start, source_start, range_len) = (parts[0], parts[1], parts[2]);
                    let delta = start - source_start;
                    if delta >= 0 && delta < range_len {
                        let new_interval_length = std::cmp::min(range_len - delta, length);
                        new_inputs.push((dest_start + delta, new_interval_length));
                        start += new_interval_length;
                        length -= new_interval_length;
                        found = true;
                        break;
                    }
                }
                if !found {
                    new_inputs.push((start, length));
                    break;
                }
            }
        }
        inputs = new_inputs;
    }
    inputs.iter().map(|(start, _)| *start).min().unwrap()
}

pub fn solve(input_directory: &str, file_name: &str) -> io::Result<()> {
    let file_path = format!("{}{}", input_directory, file_name);
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    
    let file_data: Vec<String> = reader.lines().map(|line| line.unwrap()).collect(); // TODO: fix to do split
    println!("task 1: {}", short_order(file_data.clone()));
    println!("task 2: {}", range_order(file_data));
    Ok(())
}