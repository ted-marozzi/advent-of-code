use std::fs::File;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;

use day_two::part_one;
use day_two::part_two;

fn main() {
    let mut reader: BufReader<File> = BufReader::new(File::open("./input/input.txt").unwrap());

    println!("Part one: {}", part_one::solve(&mut reader));

    reader.seek(SeekFrom::Start(0)).unwrap();

    println!("Part two: {}", part_two::solve(&mut reader));
}
