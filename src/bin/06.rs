advent_of_code::solution!(6);

use itertools::Itertools;

enum Operator {
    Plus { a: u64, b: u64 },
    Multiply { a: u64, b: u64 },
}

impl Operator {
    pub fn new(a: u64, b: u64, operator: char) -> Operator {
        match operator {
            '+' => Operator::Plus { a, b },
            '*' => Operator::Multiply { a, b },
            _ => panic!("Unknown operator: {}", operator),
        }
    }

    pub fn apply(&self) -> u64 {
        match self {
            Operator::Plus { a, b } => a + b,
            Operator::Multiply { a, b } => match a.checked_mul(*b) {
                Some(v) => v,
                None => panic!("attempt to multiply with overflow - {a} * {b}"),
            },
        }
    }
}

fn elaborate_input_part_one(input: &str) -> Vec<(Vec<u64>, char)> {
    let mut split = input
        .trim()
        .split("\n")
        .map(|row| row.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    let opx = split.pop().unwrap();

    let mut result: Vec<(Vec<u64>, char)> = Vec::with_capacity(opx.len());

    for (index, &op) in opx.iter().enumerate() {
        for values in split.iter() {
            let value = values[index].parse::<u64>().unwrap();

            if result.get(index).is_some() {
                result[index].0.push(value);
            } else {
                result.push((vec![value], op.chars().last().unwrap()));
            }
        }
    }

    result
}

fn elaborate_input_part_two(input: &str) -> Vec<(Vec<u64>, char)> {
    let mut lines: Vec<&str> = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    let operators_line = lines.pop().unwrap();
    let column_widths = calculate_column_widths(&lines, operators_line);

    let parsed_rows = parse_rows_into_columns(&mut lines, &column_widths);
    let operators: Vec<&str> = operators_line.split_whitespace().collect();

    build_result(&parsed_rows, &operators)
}

fn build_result(parsed_rows: &Vec<Vec<String>>, operators: &Vec<&str>) -> Vec<(Vec<u64>, char)> {
    operators
        .iter()
        .enumerate()
        .map(|(col_idx, &op)| {
            let operands = extract_column_operands(parsed_rows, col_idx);
            let operator = extract_operator(op);
            (operands, operator)
        })
        .collect()
}

fn extract_operator(op_str: &str) -> char {
    op_str
        .chars()
        .last()
        .unwrap_or_else(|| panic!("Operatore inatteso: {}", op_str))
}

fn extract_column_operands(parsed_rows: &Vec<Vec<String>>, column_index: usize) -> Vec<u64> {
    let mut operands: Vec<String> = vec![String::new(); parsed_rows.len()];

    for row in parsed_rows {
        if let Some(value) = row.get(column_index) {
            for (row_idx, ch) in value.chars().enumerate() {
                if ch != ' ' {
                    operands[row_idx].push(ch);
                }
            }
        }
    }

    operands
        .iter()
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn parse_rows_into_columns(lines: &mut Vec<&str>, column_widths: &Vec<usize>) -> Vec<Vec<String>> {
    let parsed_rows = lines
        .iter()
        .map(|row| parse_single_rows(row, column_widths))
        .collect::<Vec<_>>();
    parsed_rows
}

fn parse_single_rows(row: &&str, column_widths: &Vec<usize>) -> Vec<String> {
    let mut result = Vec::with_capacity(column_widths.len());
    let mut chars = row.chars();

    if let Some(&first_width) = column_widths.first() {
        let chunk = chars.by_ref().take(first_width).collect();
        result.push(chunk);
    }

    for &width in column_widths.iter().skip(1) {
        let chuck: String = chars.by_ref().skip(1).take(width).collect();
        if !chuck.is_empty() {
            result.push(chuck);
        }
    }
    result
}

fn calculate_column_widths(lines: &Vec<&str>, operators_line: &str) -> Vec<usize> {
    let mut widths: Vec<_> = operators_line
        .split(|ch: char| !ch.is_whitespace())
        .skip(1)
        .take_while(|s| !s.is_empty())
        .map(|spaces| spaces.len())
        .collect();

    let max_line_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let sum_widths = widths.iter().sum::<usize>() + widths.len();
    widths.push(max_line_length.saturating_sub(sum_widths));

    widths
}

fn calculate_result((values, op): (Vec<u64>, char)) -> u64 {
    if values.is_empty() {
        return 0;
    }

    if values.len() < 2 {
        panic!(
            "Must have more two values: {:?} - {} >= 2",
            values,
            values.len()
        );
    }

    let operators = Operator::new(values[0], values[1], op);
    let mut tmp = operators.apply();

    for value in values.iter().skip(2) {
        let operator = Operator::new(tmp, *value, op);
        tmp = operator.apply();
    }

    tmp
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = elaborate_input_part_one(input);
    if input.is_empty() {
        panic!("L'input è vuoto");
    }

    let mut count: u64 = 0;

    for el in input {
        count += calculate_result(el);
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = elaborate_input_part_two(input);
    if input.is_empty() {
        panic!("L'input è vuoto");
    }

    let mut count: u64 = 0;

    for el in input {
        count += calculate_result(el);
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
