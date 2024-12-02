fn solution<T, V>(reports: T) -> i32
where
    T: AsRef<[V]>,
    V: AsRef<[i32]>,
{
    let mut count = 0;

    for report in reports.as_ref() {
        let mut safe = true;
        let mut iter = report.as_ref().windows(2).peekable();
        let first = iter.peek().unwrap();
        let trend = if first[0] < first[1] { -1 } else { 1 };

        for pair in iter {
            let diff = trend * (pair[0] - pair[1]);
            if !(1..=3).contains(&diff) {
                safe = false;
                break;
            }
        }

        if safe {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let answer = solution(vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_solution() {
        let answer = solution(crate::INPUT);
        assert_eq!(answer, 246);
    }
}
