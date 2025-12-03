pub fn solution(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines().map(str::trim) {
        let mut biggest_pair = (0, 0);

        for (i, c) in line.chars().enumerate() {
            // since all numbers are 1 digit, we can do a cheap
            // int conversion
            let n = c as usize - '0' as usize;

            if n > biggest_pair.0 && i != (line.len() - 1) {
                biggest_pair.0 = n;
                biggest_pair.1 = 0;
            } else if n > biggest_pair.1 {
                biggest_pair.1 = n;
            }
        }

        sum += (biggest_pair.0 * 10) + biggest_pair.1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTROL_INPUT: &str = include_str!("../input-control-prt1.txt");
    const PRT1_INPUT: &str = include_str!("../input-prt1.txt");

    #[test]
    fn test_control() {
        assert_eq!(solution(CONTROL_INPUT), 357)
    }

    #[test]
    fn test_input() {
        assert_eq!(solution(PRT1_INPUT), 17092)
    }
}
