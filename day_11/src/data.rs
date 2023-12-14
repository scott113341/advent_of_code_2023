use std::collections::BTreeSet;

#[derive(Eq, PartialEq, Default, Debug)]
pub struct SpaceMap {
    pub galaxies: BTreeSet<Coord>,
    pub rows: usize,
    pub cols: usize,
}

impl SpaceMap {
    pub fn build(lines: &[String]) -> SpaceMap {
        let mut map = SpaceMap {
            rows: lines.len(),
            cols: lines[0].len(),
            ..Default::default()
        };

        for (row, line) in lines.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                if char == '#' {
                    map.galaxies.insert(Coord { row, col });
                }
            }
        }

        map
    }

    // Any rows or columns that contain no galaxies should all actually be twice as big
    pub fn expanded(&self, factor: usize) -> SpaceMap {
        let expand_by = factor - 1;
        let mut new_map = SpaceMap::default();

        let mut empty_rows: BTreeSet<usize> = (0..self.rows).collect();
        let mut empty_cols: BTreeSet<usize> = (0..self.cols).collect();

        for galaxy in self.galaxies.iter() {
            empty_rows.remove(&galaxy.row);
            empty_cols.remove(&galaxy.col);
        }

        for galaxy in self.galaxies.iter() {
            let empty_rows_above = empty_rows.iter().filter(|&&r| r < galaxy.row).count();
            let empty_cols_to_left = empty_cols.iter().filter(|&&c| c < galaxy.col).count();
            new_map.galaxies.insert(Coord {
                row: galaxy.row + empty_rows_above * expand_by,
                col: galaxy.col + empty_cols_to_left * expand_by,
            });
        }

        new_map
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}
