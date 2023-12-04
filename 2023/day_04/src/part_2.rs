pub fn solve(input: &str) -> i32 {
    count_cards(input, input, 0)
}

fn count_cards(original_input: &str, input: &str, offset: usize) -> i32 {
    let mut count = 0;

    if input.is_empty() {
        return 0;
    }

    for (index, line) in input.split("\n").enumerate() {
        let mut numbers_iter = line.split(": ").last().unwrap().split(" | ");

        let winning_cards = numbers_iter
            .next()
            .unwrap()
            .split(" ")
            .filter(|num| !num.trim().is_empty())
            .map(|num| num.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let my_winning_cards_count = numbers_iter
            .next()
            .unwrap()
            .split(" ")
            .filter(|num| !num.trim().is_empty())
            .map(|num| num.trim().parse::<i32>().unwrap())
            .filter(|num| winning_cards.contains(num))
            .count();

        count += 1 + count_cards(
            original_input,
            &original_input
                .split("\n")
                .skip(index + offset + 1)
                .take(my_winning_cards_count)
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
        assert_eq!(solve(include_str!("../data/input.txt")), 12143);
    }
}
