use std::collections::HashMap;
use std::str::FromStr;
use HandType::*;

#[derive(Eq, PartialEq, Debug)]
pub struct Hand {
    pub cards: [char; 5],
    pub card_values: [usize; 5],
    pub hand_type: HandType,
    pub bid: usize,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    pub fn j_wildcard_hand_type(&self) -> HandType {
        let j_count = self.cards.iter().filter(|c| **c == 'J').count();

        match j_count {
            0 => self.hand_type,
            1 => match self.hand_type {
                FiveOfAKind => unreachable!(),
                FourOfAKind => FiveOfAKind,
                FullHouse => unreachable!("1/FullHouse"),
                ThreeOfAKind => FourOfAKind,
                TwoPair => FullHouse,
                OnePair => ThreeOfAKind,
                HighCard => OnePair,
            },
            2 => match self.hand_type {
                FiveOfAKind => unreachable!(),
                FourOfAKind => unreachable!(),
                FullHouse => FiveOfAKind,
                ThreeOfAKind => FiveOfAKind,
                TwoPair => FourOfAKind,
                OnePair => ThreeOfAKind,
                HighCard => unreachable!(),
            },
            3 => match self.hand_type {
                FiveOfAKind => unreachable!(),
                FourOfAKind => unreachable!(),
                FullHouse => FiveOfAKind,
                ThreeOfAKind => FourOfAKind,
                TwoPair => unreachable!(),
                OnePair => unreachable!(),
                HighCard => unreachable!(),
            },
            4 => FiveOfAKind,
            5 => FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [char; 5] = s.chars().take(5).collect::<Vec<_>>().try_into().unwrap();
        let bid = s.chars().skip(6).collect::<String>().parse().unwrap();

        let card_counts = cards.iter().fold(HashMap::new(), |mut counts, card| {
            *counts.entry(card).or_insert(0) += 1;
            counts
        });

        // Counts the number of counts... for example:
        //   32T3K => { 2: 1, 1: 3 } // OnePair
        //   T55J5 => { 3: 1, 1: 2 } // ThreeOfAKind
        //   KK677 => { 2: 2, 1: 1 } // TwoPair
        let count_counts = card_counts
            .iter()
            .fold(HashMap::new(), |mut counts, (_card, count)| {
                *counts.entry(count).or_insert(0) += 1;
                counts
            });

        let hand_type = if count_counts.contains_key(&5) {
            FiveOfAKind
        } else if count_counts.contains_key(&4) {
            FourOfAKind
        } else if count_counts.contains_key(&3) && count_counts.contains_key(&2) {
            FullHouse
        } else if count_counts.contains_key(&3) {
            ThreeOfAKind
        } else if count_counts.get(&2) == Some(&2) {
            TwoPair
        } else if count_counts.contains_key(&2) {
            OnePair
        } else {
            HighCard
        };

        let card_values = cards.map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).unwrap() as usize,
        });

        Ok(Hand {
            cards,
            card_values,
            hand_type,
            bid,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, Input};

    #[test]
    fn test_hand_from_str() {
        assert_eq!(
            get_input::<Hand>(Input::Test1)[0],
            Hand {
                cards: ['3', '2', 'T', '3', 'K'],
                card_values: [3, 2, 10, 3, 13],
                hand_type: OnePair,
                bid: 765,
            }
        );
    }
}
