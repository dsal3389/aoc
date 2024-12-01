use std::{collections::HashMap, hash::Hash};

fn solution(left: Vec<u64>, right: Vec<u64>) -> u64 {
    let mut counter = HashMap::new();
    let mut distance = 0;

    for n in right {
        counter.entry(n).and_modify(|v| *v += 1).or_insert(1);
    }

    for n in left {
        if let Some(c) = counter.get(&n) {
            distance += n * c;
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
        assert_eq!(answer, 31);
    }

    #[test]
    fn test_solution() {
        let answer = solution(Vec::from(crate::LEFT), Vec::from(crate::RIGHT));
        assert_eq!(answer, 20373490);
    }
}
