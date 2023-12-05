pub mod part_1;
pub mod part_2;

#[derive(Debug, Clone)]
struct MapRanges {
    ranges: Vec<MapRange>,
}

impl MapRanges {
    pub fn new(input: &str) -> MapRanges {
        let values = input
            .split("\n")
            .skip(1)
            .map(|line| {
                line.split_whitespace()
                    .map(|seed_num| seed_num.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .into_iter()
            .collect::<Vec<_>>();
        let mut map_ranges = MapRanges { ranges: vec![] };

        for value in values {
            map_ranges.ranges.push(MapRange::new(value))
        }

        map_ranges
    }

    pub fn map(self, value: i64) -> i64 {
        for range in self.ranges {
            if let Some(mapped_value) = range.map(value) {
                return mapped_value;
            }
        }

        return value;
    }
}

#[derive(Debug, Clone)]
struct MapRange {
    source: i64,
    destination: i64,
    range: i64,
}

impl MapRange {
    pub fn new(value: Vec<i64>) -> MapRange {
        MapRange {
            destination: value[0],
            source: value[1],
            range: value[2],
        }
    }

    pub fn map(self, value: i64) -> Option<i64> {
        let lower = self.source;
        let higher = lower + self.range;

        if value >= lower && value < higher {
            let offset = value - lower;
            Some(self.destination + offset)
        } else {
            None
        }
    }
}
