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

    for line in lines.iter() {
        let first = line.iter().next().unwrap();
        let mut line_difference_result = vec![*first];

        find_line_difference(&line, &mut line_difference_result);

        let mut sign: i64 = 1;
        let mut temp = 0;
        for element in line_difference_result {
            temp += element * sign;
            sign *= -1;
        }

        result += temp;
    }

    result
}

fn find_line_difference(line: &Vec<i64>, result: &mut Vec<i64>) -> Vec<i64> {
    if line.iter().all(|x| x == &(0 as i64)) {
        return line.clone();
    }

    let line_difference = &line.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let first_element = line_difference.iter().next().unwrap();

    result.push(*first_element);

    find_line_difference(&line_difference, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_2.txt")), 2);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 964);
    }
}
