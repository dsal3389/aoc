const DIAL_MAX: usize = 100;

pub fn solution(input: &str) -> usize {
    let mut password = 0;
    let mut position: usize = 50;

    for line in input.lines() {
        let mut c = line.trim().chars();
        let direction = c.next().unwrap();
        let n = c
            .collect::<String>()
            .parse::<usize>()
            .expect("parsed an invalid number");

        if direction == 'L' {
            position = (DIAL_MAX + position - (n % DIAL_MAX)) % DIAL_MAX;
        } else {
            position = (DIAL_MAX + position + (n % DIAL_MAX)) % DIAL_MAX;
        }

        password += (position == 0) as usize;
    }
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");
    const CONTROL: &str = r#"L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82"#;

    #[test]
    fn test_controlled_input() {
        assert_eq!(solution(CONTROL), 3);
    }

    #[test]
    fn test_input() {
        assert_eq!(solution(INPUT), 1141);
    }
}
