pub mod part_1;
pub mod part_2;

#[derive(Clone, Debug)]
struct Position {
    row: usize,
    column: usize,
}

fn get_cell(grid: &Vec<&str>, position: &Position) -> Option<char> {
    if let Some(row) = grid.get(position.row) {
        if let Some(char) = row.chars().nth(position.column) {
            Some(char)
        } else {
            None
        }
    } else {
        None
    }
}
