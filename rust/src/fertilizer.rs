use std::fs::read_to_string;
use std::io;

fn short_order(data: Vec<String>) -> i64 {
    let mut orders: Vec<i64> = data[0][7..].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let maps = &data[1..];
    for almanac in maps {
        let ranges: Vec<Vec<i64>> = almanac.split('\n').skip(1).map(|triple| triple.split_whitespace().map(|s| s.parse().unwrap()).collect()).collect();
        for order in orders.iter_mut() {
            for range in &ranges {
                let (dest_start, source_start, range_len) = (range[0], range[1], range[2]);
                if source_start <= *order && *order < source_start + range_len {
                    *order = dest_start + *order - source_start;
                    break // do not repeat mappings
                }
            }
        }
    }
    let ans = orders.iter().min().unwrap();
    *ans
}

fn range_order(data: Vec<String>) -> i64 {
    data[0][7..].split_whitespace().map(|s| s.parse::<i64>().unwrap()).sum()
}

pub fn solve(input_directory: &str, file_name: &str) -> io::Result<()> {
    let file_path = format!("{}{}", input_directory, file_name);
    let read = read_to_string(file_path)?;
    let data: Vec<String> = read.split("\n\n").map(|s| s.to_string()).collect();

    println!("task 1: {}", short_order(data.clone()));
    println!("task 2: {}", range_order(data));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_short_order() {
        let data = DATA.split("\n\n").map(|s| s.to_string()).collect();
        let result = short_order(data);
        assert_eq!(result, 35);
    }
}