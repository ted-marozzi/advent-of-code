use num_integer::Integer;
use std::vec;

use crate::parse_registers;

pub fn solve(input: &str) -> i64 {
    let mut lines = input.split("\n");
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();

    lines.next();

    let registers = parse_registers(lines);
    let start_registers = registers
        .iter()
        .filter_map(|(key, value)| {
            if key.ends_with("A") {
                Some(value)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut steps_vec = vec![];

    for register in start_registers {
        let mut steps: i64 = 0;

        let mut current_register = register;
        for instruction in instructions.iter().cycle() {
            steps += 1;

            current_register = match instruction {
                'L' => {
                    if current_register.left.ends_with("Z") {
                        break;
                    }

                    registers.get(current_register.left).unwrap()
                }
                'R' => {
                    if current_register.right.ends_with("Z") {
                        break;
                    }

                    registers.get(current_register.right).unwrap()
                }
                _ => panic!("Unknown instruction: {instruction}"),
            };
        }

        steps_vec.push(steps);
    }

    steps_vec.iter().fold(1, |acc, num| acc.lcm(num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_3.txt")), 6);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 15995167053923);
    }
}
