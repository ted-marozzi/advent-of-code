use crate::parse_registers;

pub fn solve(input: &str) -> i64 {
    let mut lines = input.split("\n");
    let instructions = lines.next().unwrap().chars();

    lines.next();

    let registers = parse_registers(lines);

    let mut steps: i64 = 0;

    let mut register = registers.get("AAA").unwrap();
    for instruction in instructions.cycle() {
        steps += 1;

        register = match instruction {
            'L' => {
                if register.left == "ZZZ" {
                    return steps;
                }

                registers.get(register.left).unwrap()
            }
            'R' => {
                if register.right == "ZZZ" {
                    return steps;
                }

                registers.get(register.right).unwrap()
            }
            _ => panic!("Unknown instruction: {instruction}"),
        };
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_1.txt")), 2);
        assert_eq!(solve(include_str!("../data/example_2.txt")), 6);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 20513);
    }
}
