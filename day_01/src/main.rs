mod input;

use input::*;
use regex::Regex;

fn main() {
    println!("day: 01");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// Each line originally contained a calibration value that the Elves need to recover. On each line,
// the calibration value can be found by combining the first digit and the last digit to form a
// single two-digit number. What is the sum of all of the calibration values?
fn part_1(lines: Vec<String>) -> usize {
    let mut sum = 0;

    fn find_first_digit(mut chars: impl Iterator<Item = char>) -> usize {
        let digit = chars.find(|c| c.is_ascii_digit()).unwrap();
        digit.to_digit(10).unwrap() as usize
    }

    for line in lines {
        let first_digit = find_first_digit(line.chars());
        let last_digit = find_first_digit(line.chars().rev());
        sum += first_digit * 10 + last_digit;
    }

    sum
}

// It looks like some of the digits are actually spelled out with letters. What is the sum?
fn part_2(lines: Vec<String>) -> usize {
    let mut sum = 0;

    let nums = "([1-9]|one|two|three|four|five|six|seven|eight|nine)";
    let first_re = Regex::new(&format!(r"^.*?{}", nums)).unwrap();
    let last_re = Regex::new(&format!(r"^.*{}", nums)).unwrap();

    fn to_digit(s: &str) -> usize {
        match s {
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => panic!("Unknown digit: {}", s),
        }
    }

    for line in lines {
        let first_caps = first_re.captures(&line).unwrap();
        let first_digit = to_digit(&first_caps[1]);

        let last_caps = last_re.captures(&line).unwrap();
        let last_digit = to_digit(&last_caps[1]);

        sum += first_digit * 10 + last_digit;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 142);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(Input::Test2)), 281);
    }
}
