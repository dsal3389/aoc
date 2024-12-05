use std::collections::HashMap;

#[allow(dead_code)]
fn is_update_valid(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
    // keep track of what numbers we already saw
    let mut seen = Vec::new();

    for n in update {
        let must_come_after = match rules.get(n) {
            Some(n) => n,
            None => {
                seen.push(*n);
                continue;
            }
        };

        for v in must_come_after {
            // if the value is already seen, it breaks the current rule
            if seen.contains(v) {
                return false;
            }
        }
        seen.push(*n);
    }
    true
}

#[allow(dead_code)]
fn solution(input: String) -> u32 {
    let mut sum = 0_u32;
    let (order_rules, updates) = input.split_once("\n\n").unwrap();
    let rules_map = {
        let mut rules = HashMap::new();

        // create a mapping between each rule "<u32>|<u32>" the value is a vec since
        // it is possible that the same key appear multiple times with different values
        for (lhs, rhs) in order_rules.split('\n').map(|r| r.split_once('|').unwrap()) {
            let lhs = lhs.parse::<u32>().unwrap();
            let rhs = rhs.parse::<u32>().unwrap();
            rules.entry(lhs).or_insert_with(|| Vec::new()).push(rhs);
        }
        rules
    };

    // split each line into its own update, split each update to vector of
    // integers
    for update in updates.split_terminator('\n').map(|row| {
        row.split_terminator(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    }) {
        if is_update_valid(&rules_map, &update) {
            sum += update[update.len() / 2];
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
        assert_eq!(answer, 143);
    }

    #[test]
    fn test_solution() {
        let answer = solution(read_to_string("input.txt").unwrap());
        assert_eq!(answer, 6505);
    }
}
