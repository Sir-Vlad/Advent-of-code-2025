advent_of_code::solution!(1);
pub fn part_one(input: &str) -> Option<u64> {
    let mut position: i64 = 50;
    let mut key = 0;

    // println!("The dial starts by pointing at {position}.");

    let split = input.split('\n');
    for cmd in split {
        if cmd.is_empty() {
            continue;
        }
        let directions = cmd.chars().nth(0).unwrap();
        let angle = cmd[1..].parse::<i64>().unwrap();
        match directions {
            'R' => {
                position = (position + angle) % 100;
            }
            'L' => {
                let o = (position - angle) % 100;
                if o.is_negative() {
                    position = 100 + o;
                } else {
                    position = o;
                }
            }
            _ => unreachable!(),
        }
        // println!("The dial is rotated {cmd} to point at {position}.");
        if position == 0 {
            key += 1;
        }
    }
    Some(key)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut current_position: i64 = 50;
    let mut key = 0;

    //println!("The dial starts by pointing at {current_position}.");

    let split = input.split('\n');
    for cmd in split {
        if cmd.is_empty() {
            continue;
        }
        let directions = cmd.chars().nth(0).unwrap();
        let angle = cmd[1..].parse::<i64>().unwrap();
        let new_key = match directions {
            'R' => calculate_position(&mut current_position, angle, &mut key, directions),
            'L' => calculate_position(&mut current_position, angle, &mut key, directions),
            _ => unreachable!(),
        };
        //println!("The dial is rotated {cmd} to point at {current_position}.");
        println!("{current_position} {new_key}");
    }
    Some(key as u64)
}

fn calculate_position(current_pos: &mut i64, new_pos: i64, key: &mut i64, direction: char) -> i64 {
    let old_position = *current_pos;
    let new_angle = match direction {
        'R' => {
            let new_angle = *current_pos + new_pos;
            *current_pos = new_angle % 100;
            new_angle
        }
        'L' => {
            let new_angle = *current_pos - new_pos;
            *current_pos = if new_angle.is_negative() {
                (100 - (new_angle % 100).abs()) % 100
            } else {
                new_angle
            };
            new_angle
        }
        _ => unreachable!(),
    };

    let giri = (new_angle / 100).abs();
    let new_key = match direction {
        'R' => {
            if new_angle < 100 {
                0
            } else if giri == 1 && *current_pos == 0 {
                // print!("Zero - ");
                1
            } else if giri > 1 && *current_pos == 0 {
                giri + 1
            } else {
                // print!("Giri - {giri} - ");
                giri
            }
        }
        'L' => {
            if new_angle > 0 {
                0
            } else if giri == 0 && *current_pos == 0 {
                // print!("Zero - ");
                1
            } else if giri == 0 && new_angle <= 0 && old_position != 0 {
                // print!("Once - ");
                1
            } else if giri >= 1 && *current_pos == 0 {
                giri + 1
            } else if giri >= 1
                && (old_position - (new_pos % 100) <= 0 || (old_position != 0 && old_position - (new_pos % 100) >= 0))
            {
                giri + 1
            } else {
                // print!("Giri - {giri} - ");
                giri
            }
        }
        _ => unreachable!(),
    };
    *key += new_key;
    new_key
    // print!("Key: {key} - ")
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_r1000() {
        let result = part_two("R1000");
        assert_eq!(result, Some(10));
    }
}
