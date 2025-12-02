pub fn prt1_solution(input: &str) -> usize {
    let mut sum = 0;
    for id_range in input.split(',') {
        let (start, end) = id_range
            .trim()
            .split_once('-')
            .map(|(start, end)| {
                (
                    start.parse::<usize>().unwrap(),
                    end.parse::<usize>().unwrap(),
                )
            })
            .unwrap();

        for i in start..=end {
            let s = i.to_string();

            if s.len() == 1 || s.len() % 2 != 0 || s.starts_with('0') {
                continue;
            }

            if s[0..(s.len() / 2)] == s[(s.len() / 2)..] {
                sum += i as usize;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRT1_INPUT_CONTROLLED: &str = include_str!("../part1-input-controlled.txt");
    const PRT1_INPUT: &str = include_str!("../part1-input.txt");

    #[test]
    fn test_controlled() {
        assert_eq!(prt1_solution(PRT1_INPUT_CONTROLLED), 1227775554);
    }

    #[test]
    fn test_part1() {
        assert_eq!(prt1_solution(PRT1_INPUT), 12850231731);
    }
}
