use day_04::{part_1, part_2};

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Part one: {}", part_1::solve(&input));
    println!("Part two: {}", part_2::solve(&input));
}
