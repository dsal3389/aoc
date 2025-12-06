enum Operation {
    Add(usize),
    Multi(usize),
}

pub fn solution_prt1(input: &str) -> usize {
    let mut lines = input.trim().lines().rev().map(str::trim);

    // first line should be the operation
    let mut operations = lines
        .next()
        .map(|line| {
            let mut v = Vec::new();
            for op in line.split_whitespace() {
                let op = match op {
                    "*" => Operation::Multi(1),
                    "+" => Operation::Add(0),
                    c => panic!("unexpected operation {}", c),
                };
                v.push(op)
            }
            v
        })
        .unwrap();

    // rest if the lines should be numbers
    for line in lines {
        for (i, n) in line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .enumerate()
        {
            if i > operations.len() {
                panic!("unexpected, got a number that has no operationo")
            }

            operations[i] = match operations[i] {
                Operation::Add(sum) => Operation::Add(sum + n),
                Operation::Multi(sum) => Operation::Multi(sum * n),
            };
        }
    }

    operations.into_iter().fold(0, |acc, n| {
        let n = match n {
            Operation::Add(s) => s,
            Operation::Multi(s) => s,
        };
        acc + n
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_CONTROL_PRT1: &str = include_str!("../input-control-prt1.txt");
    const INPUT_PRT1: &str = include_str!("../input-prt1.txt");

    #[test]
    fn test_control_prt1() {
        assert_eq!(solution_prt1(INPUT_CONTROL_PRT1), 4277556);
    }

    #[test]
    fn test_prt1() {
        assert_eq!(solution_prt1(INPUT_PRT1), 7229350537438);
    }
}
