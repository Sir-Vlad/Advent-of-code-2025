use std::io::Write;
use std::sync::OnceLock;

advent_of_code::solution!(4);

#[derive(Debug)]
enum Position {
    Up { x: usize, y: usize },
    Down { x: usize, y: usize },
    Center { x: usize, y: usize },
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        match (x, y) {
            (0, y) => Position::Up { x, y },
            (x, y) if x == *MAX_SIZE.get().unwrap() => Position::Down { x, y },
            _ => Position::Center { x, y },
        }
    }
}

static MAX_SIZE: OnceLock<usize> = OnceLock::new();

type Matrix<T> = Vec<Vec<T>>;

pub fn part_one(input: &str) -> Option<u64> {
    /*
    let split = input.split('\n').collect::<Vec<_>>();
    MAX_SIZE.get_or_init(|| split[0].len() - 1);

    println!("Length: {}", MAX_SIZE.get().unwrap());

    let mut matrix = init_matrix(split);

    let mut valid = 0;
    for i in 0..matrix.len() + 1 {
        for j in 0..matrix[0].len() + 1 {
            let pos = Position::new(i, j);
            if matrix[i][j] == '.' {
                continue;
            }

            if check_matrix(&mut matrix, pos) {
                matrix[i][j] = 'x';
                valid += 1;
            }
        }
    }
    print_matrix(&matrix);

    Some(valid)
    
     */
    None
}

fn init_matrix(str: Vec<&str>) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(str.len());
    for (i, row) in str.iter().enumerate() {
        let chars: Vec<char> = row.chars().collect();
        if chars.is_empty() {
            continue;
        }
        matrix.push(chars);
    }
    matrix
}

fn print_matrix(matrix: &Matrix<char>) {
    for row in matrix {
        for el in row {
            print!("{} ", el);
        }
        println!();
    }
    println!();
}

fn check_matrix(matrix: &mut Matrix<char>, position: Position) -> bool {
    let max_size = *MAX_SIZE.get().unwrap();
    let mut count = 0;

    // Case every true
    match position {
        Position::Up { x, y } if x == 0 && y == 0 => return true,
        Position::Up { x, y } if x == 0 && y == max_size => return true,
        Position::Down { x, y } if x == max_size && y == 0 => return true,
        Position::Down { x, y } if x == max_size && y == max_size => return true,
        _ => {}
    }

    // Indices that it can check
    let indices = match position {
        Position::Up { x, y } => {
            vec![
                (x, y - 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x, y + 1),
            ]
        }
        Position::Down { x, y } => {
            vec![
                (x, y - 1),
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y + 1),
            ]
        }
        Position::Center { x, y } if y == 0 => {
            vec![
                (x - 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
                (x + 1, y),
            ]
        }
        Position::Center { x, y } if y == max_size => {
            vec![
                (x - 1, y),
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x + 1, y),
            ]
        }
        Position::Center { x, y } => {
            vec![
                (x + 1, y + 1),
                (x + 1, y),
                (x + 1, y - 1),
                (x, y - 1),
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y + 1),
            ]
        }
    };

    match position {
        Position::Center { x, y } if x == 1 && y == max_size => {
            println!("--- {position:?} ---");
        }
        _ => {}
    }

    for (x, y) in indices {
        if matrix[x][y] == '@' || matrix[x][y] == 'x' {
            count += 1;
        }
    }

    if count < 4 { true } else { false }
}

pub fn part_two(input: &str) -> Option<u64> {
    let split = input.split('\n').collect::<Vec<_>>();
    MAX_SIZE.get_or_init(|| split[0].len() - 1);

    println!("Length: {}", MAX_SIZE.get().unwrap());

    let mut matrix = init_matrix(split);

    print_matrix(&matrix);

    let mut valid = 0;
    let mut is_finish = true;

    let size = *MAX_SIZE.get().unwrap() + 1;
    while is_finish {
        is_finish = false;
        for i in 0..size {
            for j in 0..size {
                let pos = Position::new(i, j);
                if matrix[i][j] == '.' {
                    continue;
                }

                if check_matrix(&mut matrix, pos) {
                    matrix[i][j] = 'x';
                    valid += 1;
                    is_finish = true;
                }
            }
        }

        print_matrix(&matrix);
        clear_matrix(&mut matrix);
    }
    Some(valid)
}

fn clear_matrix(matrix: &mut Matrix<char>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'x' {
                matrix[i][j] = '.';
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
