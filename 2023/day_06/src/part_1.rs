pub fn solve(input: &str) -> i64 {
    input
        .split("\n")
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .zip(
            input
                .split("\n")
                .nth(1)
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap()),
        )
        .map(|(time_ms, distance_mm)| {
            (0..time_ms)
                .filter(move |time_held_ms| -> bool {
                    time_held_ms * (time_ms - time_held_ms) > distance_mm
                })
                .count()
                .try_into()
                .unwrap()
        })
        .fold(1, |acc, num_ways_to_win_race: i64| {
            acc * num_ways_to_win_race
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_1.txt")), 288);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 3316275);
    }
}
