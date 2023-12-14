use itertools::Itertools;
use std::str::FromStr;
use Spring::*;

type Springs = Vec<Spring>;
type DamagedRuns = Vec<usize>;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SpringRow {
    springs: Springs,
    damaged_runs: Vec<usize>,
}

impl SpringRow {
    pub fn unknown_idxs(&self) -> Vec<usize> {
        self.springs
            .iter()
            .enumerate()
            .filter_map(|(idx, spring)| (*spring == Unknown).then_some(idx))
            .collect()
    }

    pub fn possible_arrangements(&self) -> usize {
        let mut count = 0;
        let mut springs = self.springs.clone();

        // Foolishly brute-force through every possible manifestation of the Unknown springs
        for damaged_spring_idxs in self.unknown_idxs().into_iter().powerset() {
            // Set this combination's springs to Damaged
            damaged_spring_idxs
                .iter()
                .for_each(|idx| springs[*idx] = Damaged);

            if SpringRow::matches(&springs, &self.damaged_runs) {
                count += 1;
            }

            // Restore previous state
            damaged_spring_idxs
                .iter()
                .for_each(|idx| springs[*idx] = Unknown);
        }

        count
    }

    // Replace the list of spring conditions with five copies of itself (separated by Unknown) and replace the list of
    // damaged spring runs with five copies of itself.
    #[allow(dead_code)]
    pub fn unfold(&self) -> SpringRow {
        let mut springs = Vec::with_capacity(self.springs.len() * 5 + 5);

        for _ in 1..=5 {
            for spring in self.springs.iter() {
                springs.push(*spring);
            }
            springs.push(Unknown);
        }
        springs.pop(); // Remove final Unknown

        let damaged_runs = self.damaged_runs.repeat(5);

        SpringRow {
            springs,
            damaged_runs,
        }
    }

    pub fn matches(springs: &Springs, real_damaged_runs: &DamagedRuns) -> bool {
        let mut damaged_runs = vec![];
        let mut current_run: Option<usize> = None;

        let mut springs_iter = springs.iter().peekable();
        while let Some(spring) = springs_iter.next() {
            if spring == &Damaged {
                if let Some(run) = current_run.as_mut() {
                    *run += 1
                } else {
                    current_run = Some(1);
                }
            }

            let next_is_damaged = springs_iter
                .peek()
                .is_some_and(|spring| **spring == Damaged);

            if let Some(run) = current_run {
                if !next_is_damaged {
                    damaged_runs.push(run);
                    current_run = None;
                }
            }
        }

        damaged_runs == *real_damaged_runs
    }
}

impl FromStr for SpringRow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let springs = s
            .chars()
            .take_while(|&c| c != ' ')
            .map(|c| match c {
                '.' => Operational,
                '#' => Damaged,
                '?' => Unknown,
                _ => panic!("Unknown spring: '{}'", c),
            })
            .collect();

        let damaged_counts = s
            .split_whitespace()
            .last()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(SpringRow {
            springs,
            damaged_runs: damaged_counts,
        })
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, Input};

    #[test]
    fn test_thing_from_str() {
        assert_eq!(
            get_input::<SpringRow>(Input::Test1)[0],
            SpringRow {
                springs: vec![
                    Unknown,
                    Unknown,
                    Unknown,
                    Operational,
                    Damaged,
                    Damaged,
                    Damaged,
                ],
                damaged_runs: vec![1, 1, 3]
            }
        );
    }

    #[test]
    fn test_spring_row_possible_arrangements() {
        assert_eq!(
            get_input::<SpringRow>(Input::Test1)
                .iter()
                .map(|sr| sr.possible_arrangements())
                .collect::<Vec<_>>(),
            vec![1, 4, 1, 1, 4, 10],
        );
    }
}
