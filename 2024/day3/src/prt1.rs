fn solution(instructions: impl AsRef<str>) -> u32 {
    let chars = instructions.as_ref().chars().collect::<Vec<char>>();
    let mut chars = &chars[..];
    let mut sum = 0_u32;

    'outer: while chars.len() > 0 {
        match chars {
            ['m', 'u', 'l', '(', rest @ ..] => {
                chars = rest;

                // length variable will let us keep track
                // how long we read
                let mut length = 0;

                // buffers to contain the left and right number, the ptr
                // will point to the correct buffer
                let mut buffers = [String::with_capacity(3), String::with_capacity(3)];

                // number to keep track of how many numbers we read
                // increased every time we find the ',' char
                let mut nums = 0;

                for c in chars {
                    length += 1;

                    if *c == ',' {
                        nums += 1;

                        if nums >= buffers.len() {
                            chars = &chars[length..];
                            continue 'outer;
                        }
                        continue;
                    }

                    if ('0'..='9').contains(c) {
                        buffers[nums].push(*c);
                    } else if *c == ')' {
                        break;
                    } else {
                        chars = &chars[length - 1..];
                        continue 'outer;
                    }
                }

                chars = &chars[length..];

                let mut combined = 1_u32;
                for n in buffers {
                    // check if any number in the array
                    // exceeds the size limitations
                    if !(1..=3).contains(&n.len()) {
                        continue 'outer;
                    }

                    // it is safe to unwrap since we made sure we only pushed
                    // numbers to the buffers
                    combined *= n.parse::<u32>().unwrap();
                }
                sum += combined;
            }
            _ => {
                chars = &chars[1..];
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let answer =
            solution("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(answer, 161);
    }

    #[test]
    fn test_solution() {
        let answer = solution(crate::INPUT);
        assert_eq!(answer, 184576302);
    }
}
