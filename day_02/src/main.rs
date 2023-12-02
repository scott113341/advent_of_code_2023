mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 02");
    println!("  part 1: {}", part_1(get_input(Input::Real)));
    println!("  part 2: {}", part_2(get_input(Input::Real)));
}

// Determine which games would have been possible if the bag had been loaded with only 12 red cubes,
// 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
fn part_1(games: Vec<Game>) -> usize {
    let mut sum = 0;

    for game in games {
        let possible = game
            .rounds
            .iter()
            .all(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14);

        if possible {
            sum += game.id;
        }
    }

    sum
}

// In each game you played, what is the fewest number of cubes of each color that could have been in
// the bag to make the game possible? The power of a set of cubes is equal to the numbers of red,
// green, and blue cubes multiplied together. Find the power of each game. What is the sum?
fn part_2(games: Vec<Game>) -> usize {
    let mut sum = 0;

    for game in games {
        let red_max = game.rounds.iter().map(|g| g.red).max().unwrap();
        let green_max = game.rounds.iter().map(|g| g.green).max().unwrap();
        let blue_max = game.rounds.iter().map(|g| g.blue).max().unwrap();
        let power = red_max * green_max * blue_max;
        sum += power;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_input(Input::Test1)), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_input(Input::Test1)), 2286);
    }
}
