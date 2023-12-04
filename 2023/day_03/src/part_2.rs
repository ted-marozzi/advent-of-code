use crate::{get_cell, Position};

pub fn solve(input: &str) -> i32 {
    let mut result = 0;

    let grid = input.split("\n").collect::<Vec<_>>();

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, cell) in row.chars().enumerate() {
            result += get_gear_ratio(
                &grid,
                cell,
                Position {
                    row: row_index,
                    column: column_index,
                },
            )
        }
    }

    result
}

fn get_gear_ratio(grid: &Vec<&str>, cell: char, position: Position) -> i32 {
    if cell != '*' {
        return 0;
    }

    let top_left = Position {
        row: position.row.checked_sub(1).unwrap_or(0),
        column: position.column.checked_sub(1).unwrap_or(0),
    };

    let top_left_is_digit = is_digit(grid, &top_left);

    let top_middle = Position {
        row: position.row.checked_sub(1).unwrap_or(0),
        column: position.column,
    };

    let top_middle_is_digit = is_digit(grid, &top_middle);

    let top_right = Position {
        row: position.row.checked_sub(1).unwrap_or(0),
        column: position.column + 1,
    };

    let top_right_is_digit = is_digit(grid, &top_right);

    let middle_left = Position {
        row: position.row,
        column: position.column.checked_sub(1).unwrap_or(0),
    };

    let middle_left_is_digit = is_digit(grid, &middle_left);

    let middle_right = Position {
        row: position.row,
        column: position.column + 1,
    };

    let middle_right_is_digit = is_digit(grid, &middle_right);

    let bottom_left = Position {
        row: position.row + 1,
        column: position.column.checked_sub(1).unwrap_or(0),
    };

    let bottom_left_is_digit = is_digit(grid, &bottom_left);

    let bottom_middle = Position {
        row: position.row + 1,
        column: position.column,
    };

    let bottom_middle_is_digit = is_digit(grid, &bottom_middle);

    let bottom_right = Position {
        row: position.row + 1,
        column: position.column + 1,
    };

    let bottom_right_is_digit = is_digit(grid, &bottom_right);

    let mut adjacent_numbers: Vec<Position> = vec![];

    if top_left_is_digit {
        adjacent_numbers.push(top_left);
        if !top_middle_is_digit && top_right_is_digit {
            adjacent_numbers.push(top_right);
        }
    } else if top_middle_is_digit {
        adjacent_numbers.push(top_middle);
    } else if top_right_is_digit {
        adjacent_numbers.push(top_right);
    }

    if middle_left_is_digit {
        adjacent_numbers.push(middle_left);
    }

    if middle_right_is_digit {
        adjacent_numbers.push(middle_right);
    }

    if bottom_left_is_digit {
        adjacent_numbers.push(bottom_left);
        if !bottom_middle_is_digit && bottom_right_is_digit {
            adjacent_numbers.push(bottom_right);
        }
    } else if bottom_middle_is_digit {
        adjacent_numbers.push(bottom_middle);
    } else if bottom_right_is_digit {
        adjacent_numbers.push(bottom_right);
    }

    if adjacent_numbers.len() != 2 {
        0
    } else {
        parse_adjacent_number(grid, &adjacent_numbers[0])
            * parse_adjacent_number(grid, &adjacent_numbers[1])
    }
}

fn parse_adjacent_number(grid: &Vec<&str>, position: &Position) -> i32 {
    if let Some(cell) = get_cell(grid, position) {
        if !cell.is_digit(10) {
            panic!("Not a number");
        }

        let mut number_str = cell.to_string();

        let mut current_position = position.clone();

        loop {
            if let Some(column) = current_position.column.checked_sub(1) {
                current_position.column = column;
            } else {
                break;
            }

            if let Some(char) = get_cell(grid, &current_position) {
                if char.is_digit(10) {
                    number_str = char.to_string() + &number_str;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let mut current_position: Position = position.clone();

        loop {
            current_position.column += 1;

            if let Some(char) = get_cell(grid, &current_position) {
                if char.is_digit(10) {
                    number_str = number_str + &char.to_string();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        number_str.parse::<i32>().unwrap()
    } else {
        0
    }
}

fn is_digit(grid: &Vec<&str>, position: &Position) -> bool {
    if let Some(cell) = get_cell(grid, position) {
        cell.is_digit(10)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_2.txt")), 467835);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 77509019);
    }
}
