use std::borrow::Borrow;

fn is_safe<V, T>(report: V) -> bool
where
    V: AsRef<[T]>,
    T: Borrow<i32>,
{
    let mut iter = report.as_ref().windows(2).peekable();
    let first = iter.peek().unwrap();
    let trend = if first[0].borrow() < first[1].borrow() {
        -1
    } else {
        1
    };

    for pair in iter {
        let diff = trend * (pair[0].borrow() - pair[1].borrow());
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

fn solution<T, V>(reports: T) -> i32
where
    T: AsRef<[V]>,
    V: AsRef<[i32]>,
{
    let mut count = 0;

    for report in reports.as_ref() {
        let report = report.as_ref();

        if is_safe(report) {
            count += 1;
        } else {
            for i in 0..report.len() {
                let slice = [
                    report[..i].iter().collect::<Vec<&i32>>(),
                    report[i + 1..].iter().collect::<Vec<&i32>>(),
                ]
                .concat();
                if is_safe(slice) {
                    count += 1;
                    break;
                }
            }
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
        assert_eq!(answer, 318);
    }
}
