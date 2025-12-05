pub fn solution_prt1(input: &str) -> usize {
    let mut fresh_count = 0;

    // not most efficient but meh
    let (mut ranges, mut ingridiants): (Vec<&str>, Vec<&str>) = input
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .partition(|line| line.contains('-'));

    let ranges: Vec<(usize, usize)> = ranges
        .iter_mut()
        .flat_map(|s| {
            s.split_once('-')
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
        })
        .collect();

    let ingridiants: Vec<usize> = ingridiants.iter_mut().map(|s| s.parse().unwrap()).collect();

    for ingridiant in ingridiants {
        for range in &ranges {
            if ingridiant >= range.0 && ingridiant <= range.1 {
                fresh_count += 1;
                break;
            }
        }
    }

    fresh_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_CONTROL_PRT1: &str = include_str!("../input-control-prt1.txt");
    const INPUT_PRT1: &str = include_str!("../input-prt1.txt");

    #[test]
    fn test_prt1_control() {
        assert_eq!(solution_prt1(INPUT_CONTROL_PRT1), 3);
    }

    #[test]
    fn test_prt1() {
        assert_eq!(solution_prt1(INPUT_PRT1), 782);
    }
}
