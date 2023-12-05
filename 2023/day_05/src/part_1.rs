use crate::MapRanges;

pub fn solve(input: &str) -> i64 {
    let lines = input.split("\n\n").collect::<Vec<_>>();

    let seeds = lines
        .get(0)
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|seed_num| seed_num.parse::<i64>().unwrap());

    let seed_to_soil_values = MapRanges::new(lines.get(1).unwrap());
    let soil_to_fertilizer = MapRanges::new(lines.get(2).unwrap());
    let fertilizer_to_water = MapRanges::new(lines.get(3).unwrap());
    let water_to_light = MapRanges::new(lines.get(4).unwrap());
    let light_to_temperature = MapRanges::new(lines.get(5).unwrap());
    let temperature_to_humidity = MapRanges::new(lines.get(6).unwrap());
    let humidity_to_location = MapRanges::new(lines.get(7).unwrap());

    seeds
        .map(|seed| {
            humidity_to_location.to_owned().map(
                temperature_to_humidity.to_owned().map(
                    light_to_temperature.to_owned().map(
                        water_to_light.to_owned().map(
                            fertilizer_to_water.to_owned().map(
                                soil_to_fertilizer
                                    .to_owned()
                                    .map(seed_to_soil_values.to_owned().map(seed)),
                            ),
                        ),
                    ),
                ),
            )
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_1.txt")), 35);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 218513636);
    }
}
