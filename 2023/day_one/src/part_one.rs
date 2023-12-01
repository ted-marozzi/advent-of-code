use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::parse_two_digits;

pub fn solve(reader: &mut BufReader<File>) -> i32 {
    let mut result = 0;

    for maybe_line in reader.lines() {
        if let Ok(line) = maybe_line {
            result += parse_two_digits(
                &line.chars().find(|char| char.is_digit(10)).unwrap(),
                &line.chars().rfind(|char| char.is_digit(10)).unwrap(),
            );
        }
    }
    return result;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut reader: BufReader<File> = BufReader::new(File::open("./input/input.txt").unwrap());

        assert_eq!(solve(&mut reader), 54953);
    }
}
