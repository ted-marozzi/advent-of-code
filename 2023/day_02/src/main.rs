use std::fs::File;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;

use day_02::part_1;
use day_02::part_2;

fn main() {
    let mut reader: BufReader<File> = BufReader::new(File::open("./input/input.txt").unwrap());

    println!("Part one: {}", part_1::solve(&mut reader));

    reader.seek(SeekFrom::Start(0)).unwrap();

    println!("Part two: {}", part_2::solve(&mut reader));
}
