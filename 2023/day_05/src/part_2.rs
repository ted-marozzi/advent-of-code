use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::MapRanges;
use itertools::Itertools;

pub fn solve(input: &str) -> i64 {
    let lines = input.split("\n\n").collect::<Vec<_>>();

    let seeds = lines
        .get(0)
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|seed_num| seed_num.parse::<i64>().unwrap())
        .chunks(2);

    let maybe_min_location: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(None));
    let mut handles = vec![];

    seeds.into_iter().for_each(|mut chunk| {
        let seed_start = chunk.next().unwrap();
        let range = chunk.next().unwrap();

        let maybe_min_location: Arc<Mutex<Option<i64>>> = Arc::clone(&maybe_min_location);
        let seed_to_soil_values = MapRanges::new(lines.get(1).unwrap());
        let soil_to_fertilizer = MapRanges::new(lines.get(2).unwrap());
        let fertilizer_to_water = MapRanges::new(lines.get(3).unwrap());
        let water_to_light = MapRanges::new(lines.get(4).unwrap());
        let light_to_temperature = MapRanges::new(lines.get(5).unwrap());
        let temperature_to_humidity = MapRanges::new(lines.get(6).unwrap());
        let humidity_to_location = MapRanges::new(lines.get(7).unwrap());

        let handle = thread::spawn(move || {
            let min_location_in_range = (seed_start..seed_start + range)
                .map(|seed| {
                    humidity_to_location.clone().map(
                        temperature_to_humidity.clone().map(
                            light_to_temperature.clone().map(
                                water_to_light.clone().map(
                                    fertilizer_to_water.clone().map(
                                        soil_to_fertilizer
                                            .clone()
                                            .map(seed_to_soil_values.clone().map(seed)),
                                    ),
                                ),
                            ),
                        ),
                    )
                })
                .min()
                .unwrap();

            let mut maybe_current_min_location = maybe_min_location.lock().unwrap();

            if maybe_current_min_location.is_some() {
                if min_location_in_range < maybe_current_min_location.unwrap() {
                    *maybe_current_min_location = Some(min_location_in_range);
                }
            } else {
                *maybe_current_min_location = Some(min_location_in_range);
            }
        });
        handles.push(handle);
    });

    for handle in handles {
        handle.join().unwrap();
    }

    let min_location = maybe_min_location.lock().unwrap().unwrap();

    min_location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_2.txt")), 46);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 81956384);
    }
}
