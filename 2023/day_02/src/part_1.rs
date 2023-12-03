use regex::Regex;

pub fn solve(input: &str) -> i32 {
    let impossible_game = Regex::new(
        r"((1[3-9])|[2-9]\d+|\d\d\d) red|((1[4-9])|[2-9]\d+|\d\d\d) green|((1[5-9])|[2-9]\d+|\d\d\d) blue",
    ).unwrap();

    let game_id = Regex::new(r"^Game (\d+):").unwrap();

    let mut result = 0;
    for line in input.split("\n") {
        if impossible_game.is_match(&line) {
            continue;
        }

        result += game_id
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 2416);
    }

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_1.txt")), 8);
    }
}
