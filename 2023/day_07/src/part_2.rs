use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub fn solve(input: &str) -> i64 {
    let mut hands = input
        .split("\n")
        .map(|line| Hand::from(line))
        .collect::<Vec<_>>();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (index, hand)| acc + (index + 1) as i64 * hand.bid)
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    r#type: Type,
    bid: i64,
}

impl Hand {
    pub fn from(line: &str) -> Hand {
        let mut line_iter = line.split_whitespace();

        let cards = line_iter
            .next()
            .unwrap()
            .chars()
            .map(|char| Card::from(char))
            .collect::<Vec<_>>();

        let r#type = Type::from(&cards.clone());

        Hand {
            cards,
            r#type,
            bid: line_iter.next().unwrap().parse::<i64>().unwrap(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.r#type.cmp(&other.r#type) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    if self_card == other_card {
                        continue;
                    }

                    return self_card.cmp(other_card);
                }

                Ordering::Equal
            }
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.bid != other.bid {
            false
        } else {
            let set: HashSet<_> = self.cards.iter().copied().collect::<HashSet<_>>();
            other.cards.iter().all(|item| set.contains(item))
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Card {
    Ace = 13,
    King = 12,
    Queen = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

impl Card {
    fn from(char: char) -> Card {
        match char {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            'J' => Card::Joker,
            char => panic!("Unknown char: {}", char),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl Type {
    pub fn from(cards: &Vec<Card>) -> Type {
        let mut cards_count = HashMap::<&Card, i64>::new();

        for card in cards {
            let count = cards_count.get(&card).unwrap_or(&0);
            cards_count.insert(card, count + 1);
        }

        let joker_count = *cards_count.get(&Card::Joker).unwrap_or(&0);

        let mut cards_count: Vec<(&&Card, &i64)> = cards_count.iter().collect::<Vec<_>>();
        cards_count.sort_by(|(_, count_a), (_, count_b)| count_a.cmp(count_b));

        let max_card_count = cards_count.pop().unwrap();

        let max_count_card = *max_card_count.0;
        let max_count = *max_card_count.1;

        let second_max_count = cards_count
            .pop()
            .and_then(|(_, count)| Some(count))
            .unwrap_or(&0);

        let max_count = match (max_count_card, joker_count) {
            (Card::Joker, _) => max_count + second_max_count,
            _ => max_count + joker_count,
        };

        match (max_count, second_max_count) {
            (5, _) => Type::FiveOfAKind,
            (4, _) => Type::FourOfAKind,
            (3, 2) => Type::FullHouse,
            (3, _) => Type::ThreeOfAKind,
            (2, 2) => Type::TwoPair,
            (2, _) => Type::OnePair,
            (1, _) => Type::HighCard,
            _ => panic!("Not a valid hand"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(include_str!("../data/example_2.txt")), 5905);
    }

    #[test]
    fn test_input() {
        assert_eq!(solve(include_str!("../data/input.txt")), 255632664);
    }
}
