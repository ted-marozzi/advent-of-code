use crate::CachedWinningNumbers;

pub fn solve(input: &str) -> i32 {
    let mut winning_numbers = CachedWinningNumbers::new();
    count_cards(&mut winning_numbers, input, input, 0)
}

fn count_cards(
    winning_numbers: &mut CachedWinningNumbers,
    original_input: &str,
    input: &str,
    offset: usize,
) -> i32 {
    let mut count = 0;

    if input.is_empty() {
        return 0;
    }

    for (index, line) in input.split("\n").enumerate() {
        let winning_numbers_count = winning_numbers.count(line);

        count += 1 + count_cards(
            winning_numbers,
            original_input,
            &original_input
                .split("\n")
                .skip(index + offset + 1)
                .take(winning_numbers_count)
                .collect::<Vec<_>>()
                .join("\n"),
            index + offset + 1,
        );
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_2.txt")), 30);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 8477787);
    }
}
