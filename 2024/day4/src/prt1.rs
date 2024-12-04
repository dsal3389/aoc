#[allow(dead_code)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
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
    word: &[char],
) -> bool {
    // if x or y are out of bounce, and if they are not
    // we check if the char in that position is actually
    // what we expect
    if !input
        .get(y)
        .and_then(|chars| chars.get(x))
        .is_some_and(|c| *c == word[0])
    {
        return false;
    }

    // since we on the last check and
    // we know it is valid, that is no need to further process
    if word.len() == 1 {
        return true;
    }

    match direction {
        Direction::Bottom => word_in_direction(input, direction, x, y + 1, &word[1..]),
        Direction::BottomRight => word_in_direction(input, direction, x + 1, y + 1, &word[1..]),
        Direction::BottomLeft if x != 0 => {
            word_in_direction(input, direction, x - 1, y + 1, &word[1..])
        }
        Direction::Right => word_in_direction(input, direction, x + 1, y, &word[1..]),
        Direction::Left if x != 0 => word_in_direction(input, direction, x - 1, y, &word[1..]),
        Direction::Top if y != 0 => word_in_direction(input, direction, x, y - 1, &word[1..]),
        Direction::TopLeft if x != 0 && y != 0 => {
            word_in_direction(input, direction, x - 1, y - 1, &word[1..])
        }
        Direction::TopRight if y != 0 => {
            word_in_direction(input, direction, x + 1, y - 1, &word[1..])
        }
        _ => false,
    }
}

#[allow(dead_code)]
fn solution(input: String, lookup: String) -> u32 {
    let mut count = 0_u32;
    let lookup = lookup.chars().collect::<Vec<char>>();
    let lookup = &lookup[..];

    // convert the input string to 2d array
    let input_array = input
        .split_terminator('\n')
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    for (y, chars) in input_array.iter().enumerate() {
        for (x, c) in chars.iter().enumerate() {
            // we find the start of the
            if *c != lookup[0] {
                continue;
            }

            // look around the char in any direction
            // and for each case of `true` increase count
            for r in [
                word_in_direction(&input_array, Direction::Top, x, y, lookup),
                word_in_direction(&input_array, Direction::Bottom, x, y, lookup),
                word_in_direction(&input_array, Direction::Left, x, y, lookup),
                word_in_direction(&input_array, Direction::Right, x, y, lookup),
                word_in_direction(&input_array, Direction::BottomLeft, x, y, lookup),
                word_in_direction(&input_array, Direction::BottomRight, x, y, lookup),
                word_in_direction(&input_array, Direction::TopLeft, x, y, lookup),
                word_in_direction(&input_array, Direction::TopRight, x, y, lookup),
            ] {
                if r {
                    count += 1;
                }
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
        let answer = solution(read_to_string("example.txt").unwrap(), "XMAS".to_string());
        assert_eq!(answer, 18);
    }

    #[test]
    fn test_solution() {
        let answer = solution(read_to_string("input.txt").unwrap(), "XMAS".to_string());
        assert_eq!(answer, 2458);
    }
}
