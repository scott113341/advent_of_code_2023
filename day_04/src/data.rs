use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct Card {
    pub id: usize,
    pub winning_nums: HashSet<usize>,
    pub my_nums: HashSet<usize>,
    pub match_count: usize,
}

pub type Nums = HashSet<usize>;

impl Card {
    pub fn score(&self) -> usize {
        match self.match_count {
            0 => 0,
            1 => 1,
            _ => 2 << (self.match_count - 2),
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^Card\s+(\d+): ([\d\s]+) \| ([\d\s]+)$").unwrap());

        fn get_nums(s: &str) -> Nums {
            s.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        }

        let caps = RE.captures(s).unwrap();
        let id = caps[1].parse().unwrap();
        let winning_nums = get_nums(&caps[2]);
        let my_nums = get_nums(&caps[3]);
        let match_count = winning_nums.intersection(&my_nums).count();

        Ok(Card {
            id,
            winning_nums,
            my_nums,
            match_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, Input};

    #[test]
    fn test_card_from_str() {
        assert_eq!(
            get_input::<Card>(Input::Test1)[0],
            Card {
                id: 1,
                winning_nums: [41, 48, 83, 86, 17].into(),
                my_nums: [83, 86, 6, 31, 17, 9, 48, 53].into(),
                match_count: 4,
            }
        );
    }

    #[test]
    fn test_card_score() {
        assert_eq!(
            get_input::<Card>(Input::Test1)
                .iter()
                .map(Card::score)
                .collect::<Vec<_>>(),
            vec![8, 2, 2, 1, 0, 0],
        );
    }
}
