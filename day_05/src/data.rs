use std::ops::RangeInclusive;

#[derive(Eq, PartialEq, Debug)]
enum ParseState {
    SeedsLine,
    Waiting,
    InitMap,
    AddRanges(Map),
    FinishMap(Map),
    Done,
}

pub fn parse_input(lines: &[String]) -> (Seeds, Vec<Map>) {
    use ParseState::*;

    let mut seeds = None;
    let mut maps = vec![];
    let mut state = SeedsLine;
    let mut lines = lines.iter().peekable();

    loop {
        match state {
            SeedsLine => {
                let line = lines.next().unwrap();
                seeds = Some(line.split(' ').filter_map(|s| s.parse().ok()).collect());
                state = Waiting;
            }
            Waiting => match lines.peek() {
                Some(l) if l.is_empty() => {
                    lines.next();
                }
                Some(_) => state = InitMap,
                None => state = Done,
            },
            InitMap => {
                let line = lines.next().unwrap();
                let map = Map::new(line.split(' ').next().unwrap().to_string());
                state = AddRanges(map);
            }
            AddRanges(mut map) => {
                let line = lines.next().unwrap();
                let nums: Vec<usize> = line.split(' ').map(|n| n.parse().unwrap()).collect();
                let dest_range_start = nums[0];
                let src_range_start = nums[1];
                let range_length = nums[2];
                let range = RangeMap {
                    source_range: src_range_start..=(src_range_start + range_length - 1),
                    destination_range: dest_range_start..=(dest_range_start + range_length - 1),
                };
                map.ranges.push(range);

                let next_is_empty =
                    lines.peek().is_none() || lines.peek().is_some_and(|l| l.is_empty());
                if next_is_empty {
                    state = FinishMap(map);
                } else {
                    state = AddRanges(map);
                }
            }
            FinishMap(map) => {
                maps.push(map);
                state = Waiting;
            }
            Done => break,
        }
    }

    (seeds.unwrap(), maps)
}

pub type Seeds = Vec<usize>;

#[derive(Eq, PartialEq, Default, Debug)]
pub struct Map {
    pub name: String,
    pub ranges: Vec<RangeMap>,
}

impl Map {
    pub fn new(name: String) -> Map {
        Map {
            name,
            ranges: vec![],
        }
    }

    pub fn destination_for(&self, number: usize) -> usize {
        let special_mapping = self
            .ranges
            .iter()
            .find(|rm| rm.source_range.contains(&number));

        match special_mapping {
            Some(range_map) => {
                let offset = number - range_map.source_range.start();
                range_map.destination_range.start() + offset
            }
            None => number,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct RangeMap {
    pub source_range: RangeInclusive<usize>,
    pub destination_range: RangeInclusive<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, Input};

    #[test]
    fn test_parse_input() {
        let (seeds, maps) = parse_input(&get_input::<String>(Input::Test1));
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(
            maps[0],
            Map {
                name: "seed-to-soil".to_string(),
                ranges: vec![
                    RangeMap {
                        source_range: 98..=99,
                        destination_range: 50..=51
                    },
                    RangeMap {
                        source_range: 50..=97,
                        destination_range: 52..=99
                    }
                ]
            }
        );
        assert_eq!(
            maps[6],
            Map {
                name: "humidity-to-location".to_string(),
                ranges: vec![
                    RangeMap {
                        source_range: 56..=92,
                        destination_range: 60..=96
                    },
                    RangeMap {
                        source_range: 93..=96,
                        destination_range: 56..=59
                    }
                ]
            }
        );
    }

    #[test]
    fn test_map_destination_for() {
        let (_seeds, maps) = parse_input(&get_input::<String>(Input::Test1));
        assert_eq!(maps[0].destination_for(0), 0);
        assert_eq!(maps[0].destination_for(1), 1);
        assert_eq!(maps[0].destination_for(49), 49);
        assert_eq!(maps[0].destination_for(50), 52);
        assert_eq!(maps[0].destination_for(51), 53);
        assert_eq!(maps[0].destination_for(96), 98);
        assert_eq!(maps[0].destination_for(97), 99);
        assert_eq!(maps[0].destination_for(98), 50);
        assert_eq!(maps[0].destination_for(99), 51);
    }
}
