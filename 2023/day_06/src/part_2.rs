pub fn solve(input: &str) -> i32 {
    let time_ms = input
        .split("\n")
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let distance_mm = input
        .split("\n")
        .nth(1)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    (0..time_ms)
        .filter(move |time_held_ms| -> bool {
            time_held_ms * (time_ms - time_held_ms) > distance_mm
        })
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_2.txt")), 71503);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 27102791);
    }
}
