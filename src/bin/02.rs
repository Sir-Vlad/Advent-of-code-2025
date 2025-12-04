use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.split(",");
    let mut invalid_el = HashSet::new();
    for range_str in input {
        if range_str.is_empty() {
            continue;
        }
        let (start, end) = range_str.split_once('-').unwrap();
        let range = std::ops::Range {
            start: start.parse::<u64>().unwrap(),
            end: end
                .trim()
                .parse::<u64>()
                .expect(format!("Errore value '{end}'").as_str())
                + 1,
        };
        for el in range {
            let el_str = el.to_string();
            let len = el_str.len();
            if len % 2 == 0 {
                let midpoint = len / 2;

                if el_str[..midpoint] == el_str[midpoint..] {
                    invalid_el.insert(el);
                }
            }
        }
        println!("{:#?}", invalid_el);
    }
    Some(invalid_el.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.split(",");
    let mut invalid_el = HashSet::new();
    for range_str in input {
        if range_str.is_empty() {
            continue;
        }
        let (start, end) = range_str.split_once('-').unwrap();
        let range = std::ops::Range {
            start: start.parse::<u64>().unwrap(),
            end: end.trim().parse::<u64>().unwrap() + 1,
        };
        for el in range {
            let el_str = el.to_string();
            let len = el_str.len();
            for index in 0..=len / 2 {
                let count = len / (index + 1);
                let sub_string = &el_str[0..(index + 1)];
                //print!("[Count: {count}, Point: {index}, Len: {len_sub_string}] - ");

                let split_str = el_str
                    .as_bytes()
                    .chunks(index + 1)
                    .map(|x| unsafe { std::str::from_utf8_unchecked(x) })
                    .collect::<Vec<_>>();

                if split_str.len() > 1 && split_str.iter().all(|x| *x == sub_string) {
                    invalid_el.insert(el);
                }
            }
        }
    }
    Some(invalid_el.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
