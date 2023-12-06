mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 06");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// Determine the number of ways to beat the record in each race. Multiply these numbers together.
fn part_1(lines: Vec<String>) -> usize {
    build_races(&lines)
        .iter()
        .map(|race| race.ways_to_beat())
        .product()
}

// There's really only one race - ignore the spaces between the numbers on each line. How many ways
// can you beat the record in this one much longer race?
fn part_2(lines: Vec<String>) -> usize {
    let fixed_time_line = lines[0].replace("Time:", "").replace(' ', "");
    let fixed_distance_line = lines[1].replace("Distance:", "").replace(' ', "");
    let race = build_races(&[fixed_time_line, fixed_distance_line])
        .pop()
        .unwrap();

    race.ways_to_beat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(Input::Test1)), 71503);
    }
}
