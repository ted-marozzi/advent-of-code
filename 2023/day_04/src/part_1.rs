use crate::CachedWinningNumbers;

pub fn solve(input: &str) -> i32 {
    let mut result = 0;

    let mut winning_numbers = CachedWinningNumbers::new();

    for line in input.split("\n") {
        let winning_numbers_count = winning_numbers.count(line);

        result += match winning_numbers_count {
            0 => 0,
            1 => 1,
            _ => 2_i32.pow((winning_numbers_count - 1).try_into().unwrap()),
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_1.txt")), 13);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 17782);
    }
}
