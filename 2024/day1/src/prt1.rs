fn solution(mut left: Vec<u64>, mut right: Vec<u64>) -> u64 {
    let mut distance = 0_u64;
    left.sort();
    right.sort();

    for (l, r) in std::iter::zip(left, right) {
        if l > r {
            distance += l - r;
        } else {
            distance += r - l;
        }
    }
    distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let answer = solution(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(answer, 11);
    }

    #[test]
    fn test_solution() {
        let answer = solution(Vec::from(crate::LEFT), Vec::from(crate::RIGHT));
        assert_eq!(answer, 1722302);
    }
}
