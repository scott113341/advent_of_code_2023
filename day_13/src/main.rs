mod data;
mod input;

use data::*;
use input::*;
use rayon::prelude::*;

fn main() {
    println!("day: 13");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// Add up the number of columns to the left of each vertical line of reflection, plus 100 multiplied by the number of
// rows above each horizontal line of reflection.
fn part_1(lines: Vec<String>) -> usize {
    lines_to_vec_of_patterns(lines)
        .par_iter()
        .map(|p| p.summary().unwrap())
        .sum()
}

// In each pattern, fix the smudge that causes a different reflection line to be valid. What is the new summary?
fn part_2(lines: Vec<String>) -> usize {
    lines_to_vec_of_patterns(lines)
        .par_iter()
        .map(|pattern| {
            let orig_summary = pattern.summary().unwrap();
            let mut unsmudged_patterns = pattern.unsmudged_patterns();

            while let Some(pattern) = unsmudged_patterns.next() {
                if let Some(summary) = pattern.summary_different_than(orig_summary) {
                    return summary;
                }
            }

            panic!(
                "Pattern {:?} never had a different reflection line for all its smudges",
                pattern
            );
        })
        .sum()
}

fn lines_to_vec_of_patterns(lines: Vec<String>) -> Vec<Pattern> {
    let mut patterns = vec![];
    let mut pattern_rows = vec![];

    let mut lines_iter = lines.into_iter().peekable();
    while let Some(line) = lines_iter.next() {
        if line.is_empty() {
            continue;
        }

        pattern_rows.push(line);

        if !lines_iter.peek().is_some_and(|line| !line.is_empty()) {
            patterns.push(Pattern::build(pattern_rows));
            pattern_rows = vec![];
        }
    }

    patterns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 405);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(Input::Test1)), 400);
    }
}
