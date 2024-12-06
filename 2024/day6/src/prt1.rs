use std::collections::HashSet;

fn solution(guard_map: String) -> usize {
    let map_vec: Vec<Vec<char>> = guard_map
        .split_terminator('\n')
        .map(|row| row.chars().collect())
        .collect();
    let mut position = (0, 0); // x, y

    // direction can only be -1, 0 and 1 to indicate to what direction
    // the next step will be taken
    let mut direction: (i8, i8) = (0, -1); // x, y

    // keep track of unique steps we got
    let mut steps = HashSet::new();

    // find the guard initial position
    for (y, row) in map_vec.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' {
                position.0 = x;
                position.1 = y;
                break;
            }
        }
    }

    loop {
        if (position.0 == 0 && direction.0 == -1) || (position.1 == 0 && direction.1 == -1) {
            return steps.len();
        }

        let next_position = (
            (position.0 as i64 + direction.0 as i64) as usize,
            (position.1 as i64 + direction.1 as i64) as usize,
        );

        match map_vec
            .get(next_position.1)
            .and_then(|row| row.get(next_position.0))
        {
            Some(c) => {
                if *c == '#' {
                    if direction.0 == 0 {
                        if direction.1 == -1 {
                            direction.0 = 1;
                        } else {
                            direction.0 = -1;
                        }
                        direction.1 = 0;
                    } else {
                        if direction.0 == -1 {
                            direction.1 = -1;
                        } else {
                            direction.1 = 1;
                        }
                        direction.0 = 0;
                    }
                } else {
                    position = next_position;
                    steps.insert(position);
                }
            }
            // next step is out of bounce, we calculated
            // all possible steps
            None => return steps.len(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_example() {
        let answer = solution(read_to_string("example.txt").unwrap());
        assert_eq!(answer, 41);
    }

    #[test]
    fn test_solution() {
        let answer = solution(read_to_string("input.txt").unwrap());
        assert_eq!(answer, 5199);
    }
}
