pub mod part_one;
pub mod part_two;

fn parse_two_digits(first_digit: &char, second_digit: &char) -> i32 {
    format!("{}{}", first_digit, second_digit)
        .parse::<i32>()
        .unwrap()
}
