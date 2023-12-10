pub fn solve(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut result = 0;

    for line in lines {
        let last = line.last().unwrap();
        let mut line_difference_result = vec![*last];

        find_line_difference(&line, &mut line_difference_result);

        result += &line_difference_result.iter().sum::<i64>();
    }

    result
}

fn find_line_difference(line: &Vec<i64>, result: &mut Vec<i64>) -> Vec<i64> {
    if line.iter().all(|x| x == &(0 as i64)) {
        return line.clone();
    }

    let line_difference = &line.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let last_element = line_difference.last().unwrap();

    result.push(*last_element);

    find_line_difference(&line_difference, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_1.txt")), 114);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 1904165718);
    }
}
