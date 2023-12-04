mod data;
mod input;

use data::*;
use input::*;
use std::collections::BTreeMap;

fn main() {
    println!("day: 04");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// Each card has a list of winning numbers, a pipe character, and then a list of numbers you have.
// The first match makes the card worth one point, and each subsequent match doubles the value.
fn part_1(cards: Vec<Card>) -> usize {
    cards.iter().map(Card::score).sum()
}

// For M matches on a card, you win one extra copy of each of the next M cards. How many total
// scratchcards do you end up with?
fn part_2(cards: Vec<Card>) -> usize {
    // This function recursively counts the number of cards "under" a given card ID. Given a Card #1
    // that has 4 matches, it'll return 4 + the recursive number of matches in Cards #2 thru #5. It
    // uses memoization for performance, hence needing to pass/return/munge "memo_under_counts".
    fn count_under(
        id: usize,
        match_counts_by_id: &BTreeMap<usize, usize>,
        mut memo_under_counts: BTreeMap<usize, usize>,
    ) -> (usize, BTreeMap<usize, usize>) {
        if let Some(memo_under_count) = memo_under_counts.get(&id) {
            return (*memo_under_count, memo_under_counts);
        }

        // This card's "under count" is the number of matches it has, PLUS the recursive count
        let match_count = match_counts_by_id[&id];
        let mut under_count = match_count;

        for under_id in (id + 1)..=(id + match_count) {
            // Recurse, passing (and then restoring) the memoization map
            let res = count_under(under_id, match_counts_by_id, memo_under_counts);
            let count = res.0;
            memo_under_counts = res.1;

            under_count += count;
            memo_under_counts.entry(under_id).or_insert(count);
        }

        (under_count, memo_under_counts)
    }

    let match_counts_by_id = cards
        .iter()
        .map(|c| (c.id, c.match_count))
        .collect::<BTreeMap<_, _>>();

    let mut total_cards = 0;
    let mut under_counts = BTreeMap::new();

    for card in cards.iter() {
        let res = count_under(card.id, &match_counts_by_id, under_counts);
        let count = res.0;
        under_counts = res.1;
        total_cards += 1 + count;
    }

    total_cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(Input::Test1)), 30);
    }
}
