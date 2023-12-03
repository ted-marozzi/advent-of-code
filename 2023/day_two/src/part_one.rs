use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve(reader: &mut BufReader<File>) -> i32 {
    let impossible_game = Regex::new(
        r"((1[3-9])|[2-9]\d+|\d\d\d) red|((1[4-9])|[2-9]\d+|\d\d\d) green|((1[5-9])|[2-9]\d+|\d\d\d) blue",
    ).unwrap();

    let game_id = Regex::new(r"^Game (\d+):").unwrap();

    let mut result = 0;
    for maybe_line in reader.lines() {
        if let Ok(line) = maybe_line {
            if !impossible_game.is_match(&line) {
                result += game_id
                    .captures(&line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut reader: BufReader<File> = BufReader::new(File::open("./input/input.txt").unwrap());

        assert_eq!(solve(&mut reader), 2416);
    }
}
