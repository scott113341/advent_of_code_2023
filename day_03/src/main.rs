mod data;
mod input;

use data::*;
use input::*;
use std::collections::BTreeMap;

fn main() {
    println!("day: 03");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// Any number adjacent to a symbol, even diagonally, is a "part number". What is the sum of all part
// numbers in the engine schematic? 528231
fn part_1(lines: Vec<String>) -> usize {
    let mut sum = 0;

    let number_map = build_number_map(&lines);
    let part_map = build_part_map(&lines);

    for (coord, num) in number_map.iter() {
        'this_coord: for row in (coord.row - 1)..=(coord.row + 1) {
            for col in (coord.col - 1)..=(coord.col + coord.len) {
                if part_map.contains_key(&Coord { row, col, len: 1 }) {
                    sum += num;
                    break 'this_coord;
                }
            }
        }
    }

    sum
}

// A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result
// of multiplying those two numbers together. What is the sum of all gear ratios?
fn part_2(lines: Vec<String>) -> usize {
    let mut sum = 0;

    let number_map = build_number_map(&lines);
    let part_map = build_part_map(&lines);
    let mut gear_map = BTreeMap::new();

    for (coord, num) in number_map.iter() {
        'this_coord: for row in (coord.row - 1)..=(coord.row + 1) {
            for col in (coord.col - 1)..=(coord.col + coord.len) {
                let check_coord = Coord { row, col, len: 1 };
                if part_map.get(&check_coord) == Some(&'*') {
                    gear_map
                        .entry(check_coord)
                        .or_insert_with(Vec::new)
                        .push(num);
                    break 'this_coord;
                }
            }
        }
    }

    for (_coord, nums) in gear_map.iter() {
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 4361);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(Input::Test1)), 467835);
    }
}
