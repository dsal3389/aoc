use std::collections::HashMap;

// takes an update row, and fixes it, returns a boolean value
// indicating if the given update was fixed (manipulated)
#[allow(dead_code)]
fn update_fix(rules: &HashMap<u32, Vec<u32>>, update: &mut Vec<u32>) -> bool {
    // keep track of what numbers we already saw
    let mut mutated = false;

    // loop until the `update` list is valid
    'outer: loop {
        let mut seen = HashMap::new();

        for i in 0..update.len() {
            let n = update[i];
            let must_come_after = match rules.get(&n) {
                Some(n) => n,
                None => {
                    seen.insert(n, i);
                    continue;
                }
            };

            for v in must_come_after {
                // if the value is already seen, it breaks the current rule
                match seen.get(v) {
                    Some(position) => {
                        mutated = true;
                        update.swap(i, *position);
                        continue 'outer;
                    }
                    None => {}
                }
            }
            seen.insert(n, i);
        }

        // if we read here it means that the update vector
        // was not mutated in the last iteration
        break;
    }
    mutated
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
    for mut update in updates.split_terminator('\n').map(|row| {
        row.split_terminator(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    }) {
        if update_fix(&rules_map, &mut update) {
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
        assert_eq!(answer, 123);
    }

    #[test]
    fn test_solution() {
        let answer = solution(read_to_string("input.txt").unwrap());
        assert_eq!(answer, 6505);
    }
}
