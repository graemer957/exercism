pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = vec![];
        for row in 0..row_count as usize {
            let mut columns = vec![];
            for column in 0..=row {
                if column == 0 || column == row {
                    columns.push(1);
                } else {
                    let previous_row = &rows[row - 1];

                    let above_left = previous_row[column - 1];
                    let above = previous_row[column];

                    columns.push(above_left + above);
                }
            }
            rows.push(columns);
        }

        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
