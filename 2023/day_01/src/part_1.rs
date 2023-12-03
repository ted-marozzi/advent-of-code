use crate::parse_two_digits;

pub fn solve(input: &str) -> i32 {
    let mut result = 0;

    for line in input.split("\n").into_iter() {
        result += parse_two_digits(
            &line.chars().find(|char| char.is_digit(10)).unwrap(),
            &line.chars().rfind(|char| char.is_digit(10)).unwrap(),
        );
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        assert_eq!(solve(include_str!("../input/input.txt")), 54953);
    }
}
