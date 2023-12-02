use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<Round>,
}

#[derive(Default, Eq, PartialEq, Debug)]
pub struct Round {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Game (\d+):").unwrap());
        static ROUND_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(\d+) (red|green|blue)").unwrap());

        let id = ID_REGEX.captures(s).unwrap()[1].parse().unwrap();
        let mut rounds = vec![];

        for r in s.split(';') {
            let mut round = Round::default();

            for cap in ROUND_REGEX.captures_iter(r) {
                let count = cap[1].parse().unwrap();
                match &cap[2] {
                    "red" => round.red = count,
                    "green" => round.green = count,
                    "blue" => round.blue = count,
                    _ => panic!(),
                }
            }

            rounds.push(round);
        }

        Ok(Game { id, rounds })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, Input};

    #[test]
    fn test_thing_from_str() {
        assert_eq!(
            get_input::<Game>(Input::Test1)[0],
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Round {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                ]
            }
        );
    }
}
