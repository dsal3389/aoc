#[allow(dead_code)]
enum Direction {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

#[allow(dead_code)]
fn word_in_direction(
    input: &Vec<Vec<char>>,
    direction: Direction,
    x: usize,
    y: usize,
    mut word: Vec<char>,
) -> bool {
    // if x or y are out of bounce, and if they are not
    // we check if the char in that position is actually
    // what we expect
    match input.get(y).and_then(|chars| chars.get(x)) {
        Some(c) => {
            if *c != word[0] {
                return false;
            }

            word.remove(0);
        }
        None => return false,
    };

    if word.len() == 0 {
        return true;
    }

    match direction {
        Direction::BottomRight => word_in_direction(input, direction, x + 1, y + 1, word),
        Direction::BottomLeft if x != 0 => word_in_direction(input, direction, x - 1, y + 1, word),
        Direction::TopLeft if x != 0 && y != 0 => {
            word_in_direction(input, direction, x - 1, y - 1, word)
        }
        Direction::TopRight if y != 0 => word_in_direction(input, direction, x + 1, y - 1, word),
        _ => false,
    }
}

#[allow(dead_code)]
fn solution(input: String, lookup: String) -> u32 {
    let mut count = 0_u32;
    let lookup = lookup.chars().collect::<Vec<char>>();
    let middle = lookup.len() / 2;

    // convert the input string to 2d array
    let input_array = input
        .split_terminator('\n')
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    for (y, chars) in input_array.iter().enumerate() {
        for (x, c) in chars.iter().enumerate() {
            // if y or x are 0, it means it is impossible to have
            // X shapes
            if x == 0 || y == 0 {
                continue;
            }

            // we find the start of the
            if *c != lookup[middle] {
                continue;
            }

            // look around the char in any direction
            // and for each case of `true` increase count
            let corners = [
                word_in_direction(
                    &input_array,
                    Direction::BottomLeft,
                    x + 1,
                    y - 1,
                    lookup.clone(),
                ),
                word_in_direction(
                    &input_array,
                    Direction::BottomRight,
                    x - 1,
                    y - 1,
                    lookup.clone(),
                ),
                word_in_direction(
                    &input_array,
                    Direction::TopLeft,
                    x + 1,
                    y + 1,
                    lookup.clone(),
                ),
                word_in_direction(
                    &input_array,
                    Direction::TopRight,
                    x - 1,
                    y + 1,
                    lookup.clone(),
                ),
            ]
            .iter()
            .filter(|r| **r)
            .count();
            if corners == 2 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_example() {
        let answer = solution(read_to_string("example.txt").unwrap(), "MAS".to_string());
        assert_eq!(answer, 9);
    }

    #[test]
    fn test_solution() {
        let answer = solution(read_to_string("input.txt").unwrap(), "MAS".to_string());
        assert_eq!(answer, 1945);
    }
}
