use std::collections::HashMap;

const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut result = Vec::new();

    let plant_types = HashMap::from([
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]);

    let student_index = if let Some(student_index) = STUDENTS.iter().position(|s| s == &student) {
        student_index * 2 // Each student has 2 slots per row along the window sill
    } else {
        return result;
    };

    let second_row_index = diagram.len() / 2 + 1; // (Ab)using rounding to ignore newline char
    let cup_indices = [
        student_index,
        student_index + 1,
        second_row_index + student_index,
        second_row_index + student_index + 1,
    ];

    for index in cup_indices {
        let Some(plant_letter) = diagram.chars().nth(index) else {
            return result;
        };
        let Some(plant_name) = plant_types.get(&plant_letter) else {
            return result;
        };

        result.push(plant_name as &'static str);
    }

    result
}
