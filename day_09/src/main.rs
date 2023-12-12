mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 09");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// Extrapolate the next value for each history. What is the sum of these extrapolated values?
fn part_1(mut histories: Vec<History>) -> isize {
    extrapolate(&mut histories);
    histories.iter().map(|h| h.values.last().unwrap()).sum()
}

// Extrapolate backwards; what is the sum of these extrapolated values?
fn part_2(mut histories: Vec<History>) -> isize {
    histories.iter_mut().for_each(|h| h.values.reverse());
    extrapolate(&mut histories);
    histories.iter_mut().for_each(|h| h.values.reverse());
    histories.iter().map(|h| h.values.first().unwrap()).sum()
}

fn extrapolate(histories: &mut [History]) {
    for orig_history in histories.iter_mut() {
        let mut extrapolated = vec![orig_history.clone()];
        let mut current_history = orig_history.clone();

        // Iteratively "reduce" the history until the value change is all zero
        while !current_history.values.iter().all(|v| *v == 0) {
            let mut next_history = History {
                values: Vec::with_capacity(current_history.values.len()),
            };

            let mut values_iter = current_history.values.iter().peekable();
            while let Some(v1) = values_iter.next() {
                if let Some(v2) = values_iter.peek() {
                    next_history.values.push(*v2 - v1);
                }
            }

            extrapolated.push(next_history.clone());
            current_history = next_history;
        }

        // Do the actual extrapolation, working from the zero-change history back up
        let mut previous_extrap = None;
        for history in extrapolated.iter_mut().rev() {
            if let Some(prev) = previous_extrap {
                let extrap_value = prev + history.values.last().unwrap();
                history.values.push(extrap_value);
                previous_extrap = Some(extrap_value);
            } else {
                history.values.push(0);
                previous_extrap = Some(0);
            }
        }

        // Swap out the original history for the now-extrapolated top-level History
        *orig_history = extrapolated.remove(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(Input::Test1)), 2);
    }
}
