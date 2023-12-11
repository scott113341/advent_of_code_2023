mod data;
mod input;

use data::*;
use input::*;
use std::cmp::Reverse;

fn main() {
    println!("day: 07");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand gets
// rank 1. What are the total winnings?
fn part_1(mut hands: Vec<Hand>) -> usize {
    hands.sort_unstable_by_key(|hand| Reverse((hand.hand_type, Reverse(hand.card_values))));
    total_winnings(&hands)
}

// Now, J cards are jokers - wildcards that can act like whatever card would make the hand the
// strongest type possible. For the purpose of breaking ties, J is still treated as J. J is now also
// the weakest card. What are the new total winnings?
fn part_2(mut hands: Vec<Hand>) -> usize {
    // Change all J values from 11 to 1
    hands.iter_mut().for_each(|hand| {
        hand.card_values.iter_mut().for_each(|c| {
            if *c == 11 {
                *c = 1
            }
        })
    });

    hands.sort_unstable_by_key(|hand| {
        Reverse((hand.j_wildcard_hand_type(), Reverse(hand.card_values)))
    });

    total_winnings(&hands)
}

fn total_winnings(hands: &[Hand]) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 6440);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(Input::Test1)), 5905);
    }
}
