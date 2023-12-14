#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Pattern {
    rows: Vec<String>,
}

impl Pattern {
    pub fn build(rows: Vec<String>) -> Pattern {
        Pattern { rows }
    }

    pub fn unsmudged_patterns(&self) -> UnsmudgedPatternsIterator {
        UnsmudgedPatternsIterator {
            pattern: self.clone(),
            iteration: 0,
        }
    }

    pub fn flip(&mut self, row_idx: usize, col_idx: usize) {
        let row = self.rows.get_mut(row_idx).unwrap();
        let char = row.chars().nth(col_idx).unwrap();
        let toggled_char = match char {
            '#' => ".",
            '.' => "#",
            _ => panic!(),
        };
        row.replace_range(col_idx..=col_idx, toggled_char);
    }

    pub fn rotated_90_deg(&self) -> Pattern {
        let mut rows = vec![String::new(); self.rows[0].len()];

        for row in self.rows.iter().rev() {
            for (new_row_idx, char) in row.chars().enumerate() {
                rows[new_row_idx].push(char);
            }
        }

        Pattern { rows }
    }

    pub fn summary(&self) -> Option<usize> {
        if let Some(above) = self.rows_above_mirror().first() {
            Some(above * 100)
        } else {
            self.rows_left_of_mirror().first().copied()
        }
    }

    // Returns the summary that is NOT the given value
    pub fn summary_different_than(&self, not_summary: usize) -> Option<usize> {
        let above = self.rows_above_mirror().iter().map(|c| c * 100).collect();
        let left = self.rows_left_of_mirror();
        let summaries = [above, left].concat();
        summaries.into_iter().find(|s| *s != not_summary)
    }

    pub fn rows_above_mirror(&self) -> Vec<usize> {
        let mut solutions = vec![];

        for row_count in 1..=(self.rows.len() - 1) {
            let (take_top, skip_top) = if row_count * 2 <= self.rows.len() {
                (row_count, 0)
            } else {
                // Imagine there are 100 rows, and row_count=98. You can actually only compare against rows 98/99 on the
                // bottom, so the top needs to be rows 96/97, hence take=2 and skip=96.
                let take = self.rows.len() - row_count;
                let skip = row_count - take;
                (take, skip)
            };

            let rows_above = self.rows.iter().skip(skip_top).take(take_top);
            let rows_below = self.rows.iter().skip(skip_top + take_top).take(row_count);

            let mirrored = rows_above.eq(rows_below.rev());
            if mirrored {
                solutions.push(row_count);
            }
        }

        solutions
    }

    pub fn rows_left_of_mirror(&self) -> Vec<usize> {
        self.rotated_90_deg().rows_above_mirror()
    }
}

pub trait StreamingIterator<'a> {
    type Item;
    fn next(&'a mut self) -> Option<Self::Item>;
}

pub struct UnsmudgedPatternsIterator {
    pattern: Pattern,
    iteration: usize,
}

impl<'a> StreamingIterator<'a> for UnsmudgedPatternsIterator {
    type Item = &'a Pattern;

    fn next(&'a mut self) -> Option<Self::Item> {
        let row_col_idx = |it| {
            (
                it / self.pattern.rows[0].len(),
                it % self.pattern.rows[0].len(),
            )
        };

        // Break once all Patterns are exhausted
        let (row, col) = row_col_idx(self.iteration);
        if self.pattern.rows.len() == row {
            return None;
        }

        // Restore previous state
        if let Some(prev_it) = self.iteration.checked_sub(1) {
            let (prev_row, prev_col) = row_col_idx(prev_it);
            self.pattern.flip(prev_row, prev_col);
        }

        // Flip this iteration's smudge
        self.pattern.flip(row, col);

        // Increment and return
        self.iteration += 1;
        Some(&self.pattern)
    }
}
