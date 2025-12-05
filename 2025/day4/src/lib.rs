const PAPER_C: char = '@';

const POSITIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn solution_prt1(input: &str) -> usize {
    let mut positions = 0;

    let diagram: Vec<Vec<char>> = input
        .lines()
        .map(str::trim)
        .map(|line| line.chars().collect())
        .collect();

    for (ri, row) in diagram.iter().enumerate() {
        for (ci, column) in row.iter().enumerate() {
            if *column != PAPER_C {
                continue;
            }

            let mut c = 0;

            for (x_direction, y_direction) in POSITIONS {
                if ri == 0 && y_direction.is_negative()
                    || (ri.strict_add_signed(y_direction)) >= diagram.len()
                {
                    continue;
                }
                if ci == 0 && x_direction.is_negative()
                    || (ci.strict_add_signed(x_direction)) >= row.len()
                {
                    continue;
                }

                let line = &diagram[ri.strict_add_signed(y_direction)];
                if line[ci.strict_add_signed(x_direction)] == PAPER_C {
                    c += 1;

                    if c == 4 {
                        break;
                    }
                }
            }

            if c < 4 {
                positions += 1;
            }
        }
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_CONTROLLED_PRT1: &str = include_str!("../input-control-prt1.txt");
    const INPUT_PRT1: &str = include_str!("../input-prt1.txt");

    #[test]
    fn test_prt1_control() {
        assert_eq!(solution_prt1(INPUT_CONTROLLED_PRT1), 13);
    }

    #[test]
    fn test_prt1() {
        assert_eq!(solution_prt1(INPUT_PRT1), 1578);
    }
}
