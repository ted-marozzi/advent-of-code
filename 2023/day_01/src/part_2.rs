use crate::parse_two_digits;

pub fn solve(input: &str) -> i32 {
    let mut result = 0;

    for line in input.split("\n").into_iter() {
        result += parse_two_digits(&find_first_real_digit(&line), &find_last_real_digit(&line));
    }
    result
}

fn find_first_real_digit(line: &str) -> char {
    let length = line.len();

    for i in 0..length {
        if let Some(digit) = parse_real_digit(&line[i..length]) {
            return digit;
        }
    }

    panic!("No digit found");
}

fn find_last_real_digit(line: &str) -> char {
    let length = line.len();

    for i in 0..length {
        if let Some(digit) = parse_real_digit(&line[length - i - 1..length]) {
            return digit;
        }
    }

    panic!("No digit found");
}

fn parse_real_digit(str: &str) -> Option<char> {
    if str.starts_with("1") || str.starts_with("one") {
        return Some('1');
    }

    if str.starts_with("2") || str.starts_with("two") {
        return Some('2');
    }

    if str.starts_with("3") || str.starts_with("three") {
        return Some('3');
    }

    if str.starts_with("4") || str.starts_with("four") {
        return Some('4');
    }

    if str.starts_with("5") || str.starts_with("five") {
        return Some('5');
    }

    if str.starts_with("6") || str.starts_with("six") {
        return Some('6');
    }

    if str.starts_with("7") || str.starts_with("seven") {
        return Some('7');
    }

    if str.starts_with("8") || str.starts_with("eight") {
        return Some('8');
    }

    if str.starts_with("9") || str.starts_with("nine") {
        return Some('9');
    }

    return None;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 53868);
    }

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_2.txt")), 281);
    }
}
