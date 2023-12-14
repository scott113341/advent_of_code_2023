mod data;
mod input;

use data::*;
use input::*;
use rayon::prelude::*;

fn main() {
    println!("day: 12");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    // println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// For each row, count all of the different arrangements of operational and broken springs that meet
// the given criteria. What is the sum of those counts?
fn part_1(spring_rows: Vec<SpringRow>) -> usize {
    spring_rows
        .par_iter()
        .map(|spring_row| spring_row.possible_arrangements())
        .sum()
}

#[allow(dead_code)]
fn part_2(spring_rows: Vec<SpringRow>) -> usize {
    spring_rows
        .par_iter()
        .map(|spring_row| spring_row.unfold().possible_arrangements())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 21);
    }

    #[test]
    fn test_part_2() {
        // assert_eq!(part_2(get_input(Input::Test1)), 525152);
    }
}
