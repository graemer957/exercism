pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    for (row, entire_row) in input.iter().enumerate() {
        for (column, candidate) in entire_row.iter().enumerate() {
            let position = Position { row, column };
            if is_saddle_point(&position, *candidate, input) {
                saddle_points.push((row, column));
            }
        }
    }

    saddle_points
}

struct Position {
    row: usize,
    column: usize,
}

fn is_saddle_point(position: &Position, candidate: u64, input: &[Vec<u64>]) -> bool {
    if input[position.row].iter().any(|value| *value > candidate) {
        return false;
    }

    if input.iter().any(|row| row[position.column] < candidate) {
        return false;
    }

    true
}
