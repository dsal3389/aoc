fn calculation_possible(target: usize, sum: usize, mag: &Vec<usize>, index: usize) -> bool {
    if sum == target {
        return true;
    }
    if sum > target || index >= mag.len() {
        return false;
    }

    let n = mag[index];
    calculation_possible(target, sum * n, mag, index + 1)
        || calculation_possible(target, sum + n, mag, index + 1)
}

fn is_equation_valid(equation: &(usize, Vec<usize>)) -> bool {
    calculation_possible(equation.0, equation.1[0], &equation.1, 1)
}

fn solution(equations: String) -> usize {
    let equations = equations
        .split_terminator('\n')
        .map(|equa| {
            let (res, nums) = equa.split_once(':').unwrap();
            let res = res.parse::<usize>().unwrap();
            let nums = nums
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (res, nums)
        })
        .collect::<Vec<(usize, Vec<usize>)>>();
    let mut sum = 0_usize;

    for equation in equations {
        if is_equation_valid(&equation) {
            sum += equation.0;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_example() {
        let answer = solution(read_to_string("example.txt").unwrap());
        assert_eq!(answer, 3749);
    }

    #[test]
    fn test_input() {
        let answer = solution(read_to_string("input.txt").unwrap());
        assert_eq!(answer, 1153997401072);
    }
}
