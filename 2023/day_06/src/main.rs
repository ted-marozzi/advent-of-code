use day_00::part_1;
use day_00::part_2;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Part one: {}", part_1::solve(&input));
    println!("Part two: {}", part_2::solve(&input));
}
