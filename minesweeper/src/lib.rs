#[rustfmt::skip]
const GRID: [(i32, i32); 9] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0), (0,  0), (1,  0),
    (-1,  1), (0,  1), (1,  1)
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut numbered_mindfield = vec![];

    for (y, row) in minefield.iter().enumerate() {
        if row.is_empty() {
            numbered_mindfield.push("".to_string());
            continue;
        }

        let mut numbered_row = String::new();
        for (x, column) in row.as_bytes().iter().enumerate() {
            match column {
                b'*' => numbered_row.push('*'),
                _ => {
                    let mines = GRID.iter().fold(0, |acc, (x_diff, y_diff)| {
                        match has_mine(minefield, x as i32 + x_diff, y as i32 + y_diff) {
                            true => acc + 1,
                            false => acc,
                        }
                    });

                    if mines == 0 {
                        numbered_row.push(' ');
                    } else {
                        numbered_row.push(char::from_digit(mines, 10).unwrap());
                    }
                }
            };
        }
        numbered_mindfield.push(numbered_row);
    }

    numbered_mindfield
}

fn has_mine(minefield: &[&str], x: i32, y: i32) -> bool {
    if x < 0 || y < 0 || y >= minefield.len() as i32 || x >= minefield[y as usize].len() as i32 {
        false
    } else {
        minefield[y as usize].as_bytes()[x as usize] == b'*'
    }
}
