const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_index = STUDENTS
        .iter()
        .position(|s| s == &student)
        .expect("Unknown student {s}!")
        * 2; // Each student has 2 slots per row along the window sill

    let second_row_index = diagram.len() / 2 + 1; // (Ab)using rounding to ignore newline char
    let cup_indices = [
        student_index,
        student_index + 1,
        second_row_index + student_index,
        second_row_index + student_index + 1,
    ];

    cup_indices
        .iter()
        .map(|index| match diagram.chars().nth(*index) {
            Some('G') => "grass",
            Some('C') => "clover",
            Some('R') => "radishes",
            Some('V') => "violets",
            _ => unreachable!("Unexpected plant in cup at index {index}!"),
        })
        .collect()
}
