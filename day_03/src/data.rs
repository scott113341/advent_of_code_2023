use std::collections::BTreeMap;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct Coord {
    pub row: isize,
    pub col: isize,
    pub len: isize,
}

type NumberMap = BTreeMap<Coord, usize>;
type PartMap = BTreeMap<Coord, char>;

pub fn build_number_map(lines: &[String]) -> NumberMap {
    let mut numbers = BTreeMap::new();

    for (row, line) in lines.iter().enumerate() {
        let mut col_idx = None;
        let mut num = String::new();

        let mut chars = line.chars().enumerate().peekable();
        while let Some((col, c)) = chars.next() {
            if c.is_ascii_digit() {
                col_idx.get_or_insert(col);
                num.push(c);
            }

            let number_is_built = col_idx.is_some();
            let end_of_number = chars.peek().is_some_and(|(_i, c)| !c.is_ascii_digit());
            let end_of_line = chars.peek().is_none();

            if number_is_built && (end_of_number || end_of_line) {
                numbers.insert(
                    Coord {
                        row: row as isize,
                        col: col_idx.unwrap() as isize,
                        len: num.len() as isize,
                    },
                    num.parse().unwrap(),
                );

                col_idx = None;
                num.clear();
            }
        }
    }

    numbers
}

pub fn build_part_map(lines: &[String]) -> PartMap {
    let mut parts = BTreeMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() || c == '.' {
                continue;
            } else {
                parts.insert(
                    Coord {
                        row: row as isize,
                        col: col as isize,
                        len: 1,
                    },
                    c,
                );
            }
        }
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{get_input, Input};

    #[test]
    fn test_build_number_map() {
        let lines = get_input::<String>(Input::Test1);
        let number_map = build_number_map(&lines);
        assert_eq!(
            number_map[&Coord {
                row: 0,
                col: 0,
                len: 3
            }],
            467
        );
        assert_eq!(
            number_map[&Coord {
                row: 5,
                col: 7,
                len: 2
            }],
            58
        );
    }

    #[test]
    fn test_build_part_map() {
        let lines = get_input::<String>(Input::Test1);
        let part_map = build_part_map(&lines);
        assert_eq!(
            part_map[&Coord {
                row: 1,
                col: 3,
                len: 1
            }],
            '*'
        );
        assert_eq!(
            part_map[&Coord {
                row: 5,
                col: 5,
                len: 1
            }],
            '+'
        );
    }
}
