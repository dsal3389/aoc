pub fn solution_prt1(input: &str) -> usize {
    let mut split_count = 0;
    let mut lines = input.trim().lines();

    let mut buffer = lines
        .next()
        .map(|s| {
            let mut buffer = Vec::with_capacity(s.len());

            for c in s.chars() {
                buffer.push(c == 'S');
            }
            buffer
        })
        .unwrap();

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '^' && buffer[i] {
                buffer[i - 1] = true;
                buffer[i + 1] = true;
                buffer[i] = false;
                split_count += 1;
            }
        }
    }

    split_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_CONTROL_PRT1: &str = include_str!("../input-control-prt1.txt");
    const INPUT_PRT1: &str = include_str!("../input-prt1.txt");

    #[test]
    fn test_solution_prt1_control() {
        assert_eq!(solution_prt1(INPUT_CONTROL_PRT1), 21);
    }

    #[test]
    fn test_solution_prt1() {
        assert_eq!(solution_prt1(INPUT_PRT1), 1550);
    }
}
