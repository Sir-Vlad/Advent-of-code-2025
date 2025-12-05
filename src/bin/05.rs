use std::cmp::{max, min};
use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn elaborate_input(input: &str) -> (Vec<&str>, Vec<u64>) {
    let mut split = input.split("\n\n");

    let ranges = split.next().unwrap().split('\n').collect::<Vec<_>>();
    let fresh_idx = split
        .next()
        .unwrap()
        .split('\n')
        .map(move |el| el.parse().expect(format!("Input invalid: {el}").as_str()))
        .collect::<Vec<u64>>();

    (ranges, fresh_idx)
}
fn create_range(str: &str) -> RangeInclusive<u64> {
    let split = str.split("-").collect::<Vec<_>>();
    if split.len() != 2 {
        panic!("The input is not range valid");
    }

    let start = split.get(0).unwrap().parse().unwrap();
    let end = split.get(1).unwrap().parse().unwrap();

    RangeInclusive::new(start, end)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, fresh_idx) = elaborate_input(input.trim());

    let count = fresh_idx
        .iter()
        .filter(|fresh_id| {
            ranges
                .iter()
                .any(|range_str| create_range(range_str).contains(fresh_id))
        })
        .count() as u64;

    Some(count)
}

#[inline]
fn can_merge(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    max(*a.start(), *b.start()) <= min(*a.end(), *b.end()) + 1
}

#[inline]
fn join_range(r1: RangeInclusive<u64>, r2: RangeInclusive<u64>) -> RangeInclusive<u64> {
    let new_start = *min(r1.start(), r2.start());
    let new_end = *max(r1.end(), r2.end());
    RangeInclusive::new(new_start, new_end)
}

#[inline]
fn merge_ranges(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return vec![];
    }

    let mut sorted = ranges;
    sorted.sort_by_key(|r| *r.start());

    let mut merged = vec![sorted[0].clone()];

    for range in sorted.into_iter().skip(1) {
        let last = merged.pop().unwrap();

        if can_merge(&last, &range) {
            let merged_range = join_range(range, last);
            merged.push(merged_range);
        } else {
            merged.push(last);
            merged.push(range);
        }
    }

    merged
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _fresh_idx) = elaborate_input(input.trim());

    let all_ranges: Vec<RangeInclusive<u64>> = ranges
        .iter()
        .map(|range_str| create_range(range_str))
        .collect();

    let range_valid = merge_ranges(all_ranges);

    let count: u64 = range_valid
        .into_iter()
        .fold(0, move |acc, x| acc + x.count() as u64);

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
