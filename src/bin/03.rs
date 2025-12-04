use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let split = input.split('\n');
    let mut result = 0;
    for row in split {
        if row.is_empty() {
            continue;
        }

        let batteries: Vec<char> = row.chars().collect();

        let mut values = Vec::new();
        for i in 0..batteries.len() {
            for j in i + 1..batteries.len() {
                values.push(format!("{}{}", batteries[i], batteries[j]));
            }
        }
        values.sort();
        values.dedup();

        result += values.last().unwrap().parse::<u64>().unwrap();
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let split = input.split('\n');
    let mut result = 0;
    for row in split {
        if row.is_empty() {
            continue;
        }

        let batteries: Vec<char> = row.chars().collect();

        if batteries.len() < 12 {
            continue;
        }

        let mut selected = String::new();
        let mut start_idx = 0;

        for position in 0..12 {
            // How many more digits we need to select after this one
            let remaining_needed = 12 - position - 1;
            // Latest index we can pick from (must leave room for remaining digits)
            let search_end = batteries.len() - remaining_needed;

            // Find the maximum digit in valid range
            let mut max_digit = '0';
            let mut max_idx = start_idx;

            for i in start_idx..search_end {
                if batteries[i] > max_digit {
                    max_digit = batteries[i];
                    max_idx = i;
                }
            }

            selected.push(max_digit);
            start_idx = max_idx + 1; // Continue search after selected position
        }

        let value = selected.parse::<u64>().unwrap();
        result += value;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
