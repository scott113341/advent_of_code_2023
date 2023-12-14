mod data;
mod input;

use data::*;
use input::*;
use itertools::Itertools;

fn main() {
    println!("day: 11");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// Expand the universe, then find the shortest path between every pair of galaxies. What is the sum of these lengths?
fn part_1(lines: Vec<String>) -> usize {
    sum_of_expanded_galaxy_pair_lengths(&lines, 2)
}

// Same, but expand empty rows/cols by 1 million
fn part_2(lines: Vec<String>) -> usize {
    sum_of_expanded_galaxy_pair_lengths(&lines, 1_000_000)
}

fn sum_of_expanded_galaxy_pair_lengths(lines: &[String], expand_by: usize) -> usize {
    SpaceMap::build(lines)
        .expanded(expand_by)
        .galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| g1.row.abs_diff(g2.row) + g1.col.abs_diff(g2.col))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_expanded_galaxy_pair_lengths() {
        assert_eq!(
            sum_of_expanded_galaxy_pair_lengths(&get_input(Input::Test1), 2),
            374
        );

        assert_eq!(
            sum_of_expanded_galaxy_pair_lengths(&get_input(Input::Test1), 10),
            1030
        );

        assert_eq!(
            sum_of_expanded_galaxy_pair_lengths(&get_input(Input::Test1), 100),
            8410
        );
    }
}
