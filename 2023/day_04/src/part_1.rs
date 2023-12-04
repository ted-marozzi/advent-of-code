pub fn solve(input: &str) -> i32 {
    let mut result = 0;

    for line in input.split("\n") {
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

        result += match my_winning_cards_count {
            0 => 0,
            1 => 1,
            _ => 2_i32.pow((my_winning_cards_count - 1).try_into().unwrap()),
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
