mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 05");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// What is the lowest location number that corresponds to any of the initial seed numbers?
fn part_1(lines: Vec<String>) -> usize {
    let (seeds, maps) = parse_input(&lines);
    let mut lowest_location = usize::MAX;

    for seed in seeds.iter() {
        let location = maps.iter().fold(*seed, |n, map| map.destination_for(n));
        lowest_location = lowest_location.min(location);
    }

    lowest_location
}

// It looks like the first line actually describes ranges of seed numbers. Each pair of numbers
// represents a (start, length) of seed numbers.
fn part_2(lines: Vec<String>) -> usize {
    use rayon::prelude::*;

    let (seeds, maps) = parse_input(&lines);
    let seed_ranges = seeds
        .chunks_exact(2)
        .map(|p| [p[0], p[1]])
        .collect::<Vec<_>>();

    seed_ranges
        .par_iter()
        .map(|pair| {
            let mut lowest_location = usize::MAX;
            let seeds_start = pair[0];
            let length = pair[1];

            for seed in seeds_start..(seeds_start + length) {
                let location = maps.iter().fold(seed, |n, map| map.destination_for(n));
                lowest_location = lowest_location.min(location);
            }

            lowest_location
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(Input::Test1)), 46);
    }
}
