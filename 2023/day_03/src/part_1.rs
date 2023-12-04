use crate::{get_cell, Position};

#[derive(Clone, Debug)]
struct PartNumber {
    id: String,
    valid: bool,
}

pub fn solve(input: &str) -> i32 {
    let mut result = 0;

    let grid = input.split("\n").collect::<Vec<_>>();
    let mut maybe_current_part: Option<PartNumber> = None;

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, cell) in row.chars().enumerate() {
            if cell.is_digit(10) {
                let adjacent_symbol = check_for_adjacent_symbol(
                    &grid,
                    Position {
                        row: row_index,
                        column: column_index,
                    },
                );

                match maybe_current_part.as_mut() {
                    None => {
                        maybe_current_part = Some(PartNumber {
                            id: cell.to_string(),
                            valid: adjacent_symbol,
                        })
                    }
                    Some(current_part) => {
                        current_part.id += &cell.to_string();
                        if !current_part.valid && adjacent_symbol {
                            current_part.valid = true;
                        }
                    }
                }

                if column_index == row.len() - 1 {
                    if let Some(current_part) = maybe_current_part {
                        if current_part.valid {
                            result += current_part.id.parse::<i32>().unwrap();
                        }
                        maybe_current_part = None;
                    }
                }
            } else {
                if let Some(current_part) = maybe_current_part {
                    if current_part.valid {
                        result += current_part.id.parse::<i32>().unwrap();
                    }
                    maybe_current_part = None;
                }
            }
        }
    }

    result
}

fn check_for_adjacent_symbol(grid: &Vec<&str>, position: Position) -> bool {
    let positions = vec![
        Position {
            row: position.row.checked_sub(1).unwrap_or(0),
            column: position.column.checked_sub(1).unwrap_or(0),
        },
        Position {
            row: position.row.checked_sub(1).unwrap_or(0),
            column: position.column,
        },
        Position {
            row: position.row.checked_sub(1).unwrap_or(0),
            column: position.column + 1,
        },
        Position {
            row: position.row,
            column: position.column.checked_sub(1).unwrap_or(0),
        },
        Position {
            row: position.row,
            column: position.column + 1,
        },
        Position {
            row: position.row + 1,
            column: position.column.checked_sub(1).unwrap_or(0),
        },
        Position {
            row: position.row + 1,
            column: position.column,
        },
        Position {
            row: position.row + 1,
            column: position.column + 1,
        },
    ];

    for position in positions {
        if symbol(grid, &position) {
            return true;
        }
    }

    return false;
}

fn symbol(grid: &Vec<&str>, position: &Position) -> bool {
    if let Some(char) = get_cell(grid, position) {
        !char.is_digit(10) && !(char == '.')
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_1.txt")), 4361);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 529618);
    }
}
