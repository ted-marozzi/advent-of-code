use std::collections::HashMap;

pub mod part_1;
pub mod part_2;

struct CachedWinningNumbers {
    cache: HashMap<String, usize>,
}

impl CachedWinningNumbers {
    pub fn new() -> Self {
        CachedWinningNumbers {
            cache: HashMap::new(),
        }
    }

    pub fn count(&mut self, line: &str) -> usize {
        if let Some(cached_count) = self.cache.get(line) {
            return *cached_count;
        }

        let winning_numbers = count_winning_numbers(line);

        self.cache.insert(line.into(), winning_numbers);

        winning_numbers
    }
}

pub fn count_winning_numbers(line: &str) -> usize {
    let mut numbers_iter = line.split(": ").last().unwrap().split(" | ");

    let winning_cards = numbers_iter
        .next()
        .unwrap()
        .split(" ")
        .filter(|num| !num.trim().is_empty())
        .map(|num| num.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let count = numbers_iter
        .next()
        .unwrap()
        .split(" ")
        .filter(|num| !num.trim().is_empty())
        .map(|num| num.trim().parse::<i32>().unwrap())
        .filter(|num| winning_cards.contains(num))
        .count();

    count
}
