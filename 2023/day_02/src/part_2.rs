use regex::Regex;

pub fn solve(input: &str) -> i32 {
    let red_count_ex = Regex::new(r"(\d+) red").unwrap();
    let green_count_ex = Regex::new(r"(\d+) green").unwrap();
    let blue_count_ex = Regex::new(r"(\d+) blue").unwrap();

    let mut result = 0;
    for line in input.split("\n") {
        result += get_minimum_count(&red_count_ex, &line)
            * get_minimum_count(&green_count_ex, &line)
            * get_minimum_count(&blue_count_ex, &line);
    }

    result
}

fn get_minimum_count(ex: &Regex, line: &str) -> i32 {
    let mut minimum_count = 0;
    for capture in ex.captures_iter(&line) {
        if let Some(count_match) = capture.get(1) {
            let count = count_match.as_str().parse::<i32>().unwrap();

            if count > minimum_count {
                minimum_count = count;
            }
        }
    }

    minimum_count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        assert_eq!(solve(include_str!("../input/input.txt")), 63307);
    }
}
