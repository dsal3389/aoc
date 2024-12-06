/// THS MF PART2 ALMSOT BROKE ME FFS
/// CODE IS UGLY, AND FK EVERYTHING HERE
use std::collections::HashSet;

fn solution(guard_map: String) -> usize {
    let mut map_vec: Vec<Vec<char>> = guard_map
        .split_terminator('\n')
        .map(|row| row.chars().collect())
        .collect();
    let mut start_position = (0, 0); // x, y

    // find the guard initial position
    for (y, row) in map_vec.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' {
                start_position.0 = x;
                start_position.1 = y;
                break;
            }
        }
    }

    let mut possibilites = 0;

    for y in 0..map_vec.len() {
        for x in 0..map_vec[y].len() {
            if map_vec[y][x] == '#' || map_vec[y][x] == '^' {
                continue;
            }

            // keep track of unique steps we got
            let mut steps = HashSet::new();
            let mut position = start_position;

            // direction can only be -1, 0 and 1 to indicate to what direction
            // the next step will be taken
            let mut direction: (i8, i8) = (0, -1); // x, y

            map_vec[y][x] = 'O';

            let is_looping = loop {
                if (position.0 == 0 && direction.0 == -1) || (position.1 == 0 && direction.1 == -1)
                {
                    break false;
                }

                let next_position = (
                    (position.0 as i64 + direction.0 as i64) as usize,
                    (position.1 as i64 + direction.1 as i64) as usize,
                );

                match map_vec
                    .get(next_position.1)
                    .and_then(|row| row.get(next_position.0))
                {
                    Some(c) => match *c {
                        '#' | 'O' => {
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

                            if steps.contains(&(position, direction)) {
                                break true;
                            }

                            steps.insert((position, direction));
                        }
                        _ => {
                            position = next_position;
                            steps.insert((position, direction));
                        }
                    },
                    // next step is out of bounce, we calculated
                    // all possible steps
                    None => break false,
                }
            };

            map_vec[y][x] = '.';

            if is_looping {
                possibilites += 1;
            }
        }
    }
    possibilites
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_example() {
        let answer = solution(read_to_string("example.txt").unwrap());
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_solution() {
        let answer = solution(read_to_string("input.txt").unwrap());
        assert_eq!(answer, 1915);
    }
}
